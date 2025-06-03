use std::sync::mpsc::channel;

use amico::ai::services::CompletionServiceDyn;
use amico::resource::Resource;
use amico_core::{traits::System, world::HandlerRegistry};
use amico_mods::std::ai::{
    providers::rig::RigProvider,
    services::{
        InMemoryService,
        speech::{speech_to_text, text_to_speech},
    },
};
use evenio::prelude::*;
use tokio::sync::Mutex;

use super::{
    components::{Player, Recorder},
    events::{AgentContent, PlaybackFinish, RecordFinish, RecordStart, UserContent, UserInput},
    interaction::CliComponent,
};

pub struct ChatbotSystem {
    pub cli_component: Resource<CliComponent>,
    pub recorder: Resource<Mutex<Recorder>>,
}

impl System for ChatbotSystem {
    fn register_to(self, mut registry: HandlerRegistry) {
        let Self {
            cli_component,
            recorder,
        } = self;

        let cli = cli_component.value_ptr();
        let recorder = recorder.value_ptr();
        registry.register(
            move |r: Receiver<UserInput>,
                  mut sender: Sender<(UserContent, RecordStart, RecordFinish)>| {
                tracing::debug!("ChatbotSystem: Received {:?}", r.event);

                let UserInput(content) = r.event;

                let (tx, rx) = channel::<bool>();
                let recorder_resource = recorder.clone();
                tokio::spawn(async move {
                    tx.send(recorder_resource.lock().await.is_recording())
                        .unwrap();
                });

                let is_recording = rx.recv().unwrap();

                // If user typed "s" and the recorder is not recording, start the recorder.
                if content.eq_ignore_ascii_case("s") && !is_recording {
                    sender.send(RecordStart);
                    cli.handle_record_start();
                    return;
                }

                let (tx, rx) = channel::<bool>();
                let recorder_resource = recorder.clone();
                tokio::spawn(async move {
                    tx.send(recorder_resource.lock().await.is_recording())
                        .unwrap();
                });

                let is_recording = rx.recv().unwrap();

                // If any user input is received during recording, stop recording.
                if is_recording {
                    sender.send(RecordFinish);
                    cli.handle_record_finish();
                    return;
                }

                // Send user input directly to agent in any other situations.
                sender.send(UserContent(content.to_string()));
            },
        );

        let cli = cli_component.value_ptr();
        registry.register(move |r: Receiver<AgentContent>| {
            tracing::debug!("ChatbotSystem: Received {:?}", r.event);

            let AgentContent(content) = r.event;
            cli.print_message(content);
        });

        let cli = cli_component.value_ptr();
        registry.register(move |r: Receiver<PlaybackFinish>| {
            tracing::debug!("ChatbotSystem: Received {:?}", r.event);

            cli.handle_playback_finish();
        });
    }
}

pub struct SpeechSystem {
    pub recorder: Resource<Mutex<Recorder>>,
    pub user_mp3_path: &'static str,
    pub agent_mp3_path: &'static str,
}

impl System for SpeechSystem {
    fn register_to(self, mut registry: HandlerRegistry) {
        let Self {
            recorder,
            user_mp3_path,
            agent_mp3_path,
        } = self;

        let recorder_resource = recorder.value_ptr();
        registry.register(move |r: Receiver<RecordStart>| {
            tracing::debug!("SpeechSystem: Received {:?}", r.event);

            let (tx, rx) = channel::<()>();
            let recorder_resource = recorder_resource.clone();
            tokio::spawn(async move {
                if recorder_resource.lock().await.is_recording() {
                    panic!("`RecordStart` handler: Recorder is recording!");
                }

                recorder_resource.lock().await.start_record(user_mp3_path);

                tx.send(()).unwrap();
            });

            rx.recv().unwrap();
        });

        let recorder_resource = recorder.value_ptr();
        registry.register(
            move |r: Receiver<RecordFinish>, mut sender: Sender<UserContent>| {
                tracing::debug!("SpeechSystem: Received {:?}", r.event);

                let (tx, rx) = channel::<String>();
                let recorder_resource = recorder_resource.clone();
                tokio::spawn(async move {
                    if !recorder_resource.lock().await.is_recording() {
                        panic!("`RecordFinish` handler: Recorder is not recording!");
                    }

                    recorder_resource.lock().await.finish_record().unwrap();

                    let content = speech_to_text(user_mp3_path).await.unwrap();
                    tx.send(content).unwrap();
                });
                let content = rx.recv().unwrap();
                sender.send(UserContent(content));
            },
        );

        registry.register(
            move |r: Receiver<AgentContent>, mut sender: Sender<PlaybackFinish>| {
                tracing::debug!("SpeechSystem: Received {:?}", r.event);

                let AgentContent(content) = r.event;
                let content = content.to_string();

                struct Signal;
                let (tx, rx) = channel::<Signal>();
                tokio::spawn(async move {
                    text_to_speech(&content, agent_mp3_path).await.unwrap();

                    Player.play_blocking(agent_mp3_path).unwrap();

                    tx.send(Signal).unwrap();
                });

                rx.recv().unwrap();
                sender.send(PlaybackFinish);
            },
        );
    }
}

pub struct CompletionSystem {
    pub service_resource: Resource<Mutex<InMemoryService<RigProvider>>>,
}

impl System for CompletionSystem {
    fn register_to(self, mut registry: HandlerRegistry) {
        let service_resource = self.service_resource.clone();
        registry.register(
            move |r: Receiver<UserContent>, mut sender: Sender<AgentContent>| {
                tracing::debug!("CompletionSystem: Received {:?}", r.event);

                let UserContent(text) = r.event;
                let text = text.to_string();

                // Spawning an async task in tokio to run `generate_text`.
                // We can't await a JoinHandle in sync block, so use a channel instead.
                let (tx, rx) = channel::<String>();
                let service_resource = service_resource.clone();
                tokio::spawn(async move {
                    let response = service_resource
                        .value()
                        .lock()
                        .await
                        .generate_text_dyn(text)
                        .await
                        .unwrap_or_else(|err| format!("Service error: {:?}", err.to_string()));

                    tx.send(response).unwrap();
                });

                let response = rx.recv().unwrap();
                sender.send(AgentContent(response));
            },
        )
    }
}

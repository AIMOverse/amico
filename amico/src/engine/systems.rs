use std::sync::mpsc::channel;

use amico::ai::services::CompletionServiceDyn;
use amico_mods::std::ai::tasks::chatbot::speech::{speech_to_text, text_to_speech};
use evenio::prelude::*;

use super::{
    components::{AiService, Player, Recorder},
    events::{AgentContent, PlaybackFinish, RecordFinish, RecordStart, UserContent, UserInput},
    interaction::Stdio,
};

pub struct ChatbotSystem {
    pub itr_layer: EntityId,
    pub env_layer: EntityId,
}

impl ChatbotSystem {
    pub fn register_to(&self, world: &mut World) {
        let Self {
            itr_layer,
            env_layer,
        } = *self;

        world.add_handler(
            move |r: Receiver<UserInput>,
                  mut sender: Sender<(UserContent, RecordStart, RecordFinish)>,
                  io_fetcher: Fetcher<&Stdio>,
                  rcd_fetcher: Fetcher<&Recorder>| {
                tracing::debug!("ChatbotSystem: Received {:?}", r.event);

                let UserInput(content) = r.event;
                let stdio = io_fetcher.get(itr_layer).unwrap();
                let recorder = rcd_fetcher.get(env_layer).unwrap();

                // If user typed "s" and the recorder is not recording, start the recorder.
                if content.to_ascii_lowercase() == "s" && !recorder.is_recording() {
                    sender.send(RecordStart);
                    stdio.handle_record_start();
                    return;
                }

                // If any user input is received during recording, stop recording.
                if recorder.is_recording() {
                    sender.send(RecordFinish);
                    stdio.handle_record_finish();
                    return;
                }

                // Send user input directly to agent in any other situations.
                sender.send(UserContent(content.to_string()));
            },
        );

        world.add_handler(
            move |r: Receiver<AgentContent>, io_fetcher: Fetcher<&Stdio>| {
                tracing::debug!("ChatbotSystem: Received {:?}", r.event);

                let stdio = io_fetcher.get(itr_layer).unwrap();
                let AgentContent(content) = r.event;
                stdio.print_message(content);
            },
        );

        world.add_handler(
            move |r: Receiver<PlaybackFinish>, io_fetcher: Fetcher<&Stdio>| {
                tracing::debug!("ChatbotSystem: Received {:?}", r.event);

                let stdio = io_fetcher.get(itr_layer).unwrap();
                stdio.handle_playback_finish();
            },
        );
    }
}

pub struct SpeechSystem {
    pub env_layer: EntityId,

    pub user_mp3_path: &'static str,
    pub agent_mp3_path: &'static str,
}

impl SpeechSystem {
    pub fn register_to(&self, world: &mut World) {
        let Self {
            env_layer,
            user_mp3_path,
            agent_mp3_path,
        } = *self;

        world.add_handler(
            move |r: Receiver<RecordStart>, mut rcd_fetcher: Fetcher<&mut Recorder>| {
                tracing::debug!("SpeechSystem: Received {:?}", r.event);

                let recorder = rcd_fetcher.get_mut(env_layer).unwrap();

                if recorder.is_recording() {
                    panic!("`RecordStart` handler: Recorder is recording!");
                }

                recorder.start_record(user_mp3_path);
            },
        );

        world.add_handler(
            move |r: Receiver<RecordFinish>,
                  mut sender: Sender<UserContent>,
                  mut rcd_fetcher: Fetcher<&mut Recorder>| {
                tracing::debug!("SpeechSystem: Received {:?}", r.event);

                let recorder = rcd_fetcher.get_mut(env_layer).unwrap();

                if !recorder.is_recording() {
                    panic!("`RecordFinish` handler: Recorder is not recording!");
                }

                recorder.finish_record().unwrap();

                let (tx, rx) = channel::<String>();
                tokio::spawn(async move {
                    let content = speech_to_text(user_mp3_path).await.unwrap();
                    tx.send(content).unwrap();
                });
                let content = rx.recv().unwrap();
                sender.send(UserContent(content));
            },
        );

        world.add_handler(
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
    pub ai_layer: EntityId,
}

impl CompletionSystem {
    pub fn register_to(&self, world: &mut World) -> HandlerId {
        let ai_layer = self.ai_layer;
        world.add_handler(
            move |r: Receiver<UserContent>,
                  mut sender: Sender<AgentContent>,
                  ai_fetcher: Fetcher<&AiService>| {
                tracing::debug!("CompletionSystem: Received {:?}", r.event);

                let service = ai_fetcher.get(ai_layer).unwrap().get();
                let UserContent(text) = r.event;
                let text = text.to_string();

                // Spawning an async task in tokio to run `generate_text`.
                // We can't await a JoinHandle in sync block, so use a channel instead.
                let (tx, rx) = channel::<String>();
                tokio::spawn(async move {
                    let response = service
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

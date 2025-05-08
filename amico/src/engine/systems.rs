use std::sync::mpsc::channel;

use amico::ai::services::CompletionServiceDyn;
use evenio::prelude::*;

use super::{components::AiService, events::UserContent, interaction::Stdio};

pub struct ChatbotSystem {
    pub itr_layer: EntityId,
    pub ai_layer: EntityId,
}

impl ChatbotSystem {
    pub fn register_to(&self, world: &mut World) -> HandlerId {
        let itr_layer = self.itr_layer;
        let ai_layer = self.ai_layer;
        world.add_handler(
            move |r: Receiver<UserContent>,
                  it_fetcher: Fetcher<&Stdio>,
                  ai_fetcher: Fetcher<&AiService>| {
                let stdio = it_fetcher.get(itr_layer).unwrap();
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
                stdio.print_message(response);
            },
        )
    }
}

use nostr_sdk::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum SubEventErrorType {
    InvalidEvent,
}

#[derive(Clone)]
pub struct Nostr<'a> {
    client: &'a Client,
}

impl<'a> Nostr<'a> {
    pub fn new(c: &'a Client) -> Self {
        Self { client: c }
    }

    pub async fn get_event_by_id(&self, id: EventId) -> Result<Event, SubEventErrorType> {
        let shared_data = Arc::new(Mutex::new(Vec::<Event>::new()));

        let filter = Filter::new().id(id);
        self.client.subscribe(vec![filter]).await;
        self.client
            .handle_notifications(|notification| async {
                match notification {
                    RelayPoolNotification::Event(_url, event) => {
                        if event.id == id {
                            let mut shared_data = shared_data.lock().unwrap();
                            shared_data.push(event.clone());

                            return Ok(true);
                        } else {
                            println!("invalid, {:#?}", event);
                        }
                    }
                    RelayPoolNotification::Stop => return Ok(true),
                    RelayPoolNotification::Message(_url, relay_message) => {
                        println!("{:#?}", relay_message);
                        return Ok(true);
                    }
                    _ => {}
                }

                Ok(false)
            })
            .await
            .expect("");

        let shared_data = shared_data.lock().unwrap();

        shared_data
            .last()
            .cloned()
            .ok_or(SubEventErrorType::InvalidEvent)
    }
}

use std::u8;
use serde_json::{Value as JSONValue, json};

use crate::config::Config;
use crate::event::event_emitter::EventEmitter;
use crate::channel::{MessageType, ChannelError, create_tassl, channel_fetch};

type ListenerResult = Result<JSONValue, ChannelError>;

pub struct EventService {
    config: Config,
    block_notify_event_emitter: EventEmitter<ListenerResult>,
}

impl EventService {
    pub fn new(config: &Config) -> EventService {
        EventService { 
            config: config.clone(), 
            block_notify_event_emitter: EventEmitter::new() 
        }
    }

    pub fn register_block_notify_listener(&mut self, group_id: u32, listener: fn(&ListenerResult)) {
        self.block_notify_event_emitter.on(&group_id.to_string(), listener);
    }

    pub fn remove_block_notify_listener(&mut self, group_id: u32) {
        self.block_notify_event_emitter.remove(&group_id.to_string());
    }

    pub async fn run_block_notify(&self, group_id: u32) {
        let response = match create_tassl(&self.config) {
            Ok(tassl) => {
                let params = json!([format!("_block_notify_{:?}", group_id)]);
                channel_fetch(
                    &tassl,
                    &params,
                    MessageType::AMOPClientTopics
                ).map(|buffer| {
                    let topic_len = u8::from_be_bytes(buffer[0].to_be_bytes()) as usize;
                    let values: Vec<String> = std::str::from_utf8(&buffer[topic_len..])
                        .unwrap_or("")
                        .to_string()
                        .split(",")
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect();
                    json!({
                        "group_id": String::from(&values[0]).parse::<i32>().unwrap_or(-1),
                        "block_height": String::from(&values[1]).parse::<i32>().unwrap_or(-1)
                    })
                })
            },
            Err(err) =>  Err(ChannelError::TASSLError(err))
        };
        self.block_notify_event_emitter.emit(&group_id.to_string(), &response);
    }
}
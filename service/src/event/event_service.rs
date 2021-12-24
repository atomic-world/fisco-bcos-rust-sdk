use std::{thread, u8};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use thiserror::Error;
use serde_json::{Value as JSONValue, json};

use crate::config::Config;
use crate::tassl::TASSLError;
use crate::event::event_emitter::EventEmitter;
use crate::channel::{MessageType, ChannelError, channel_read, pack_channel_message, open_tassl};

type ListenerResult = Result<JSONValue, EventServiceError>;

#[derive(Error, Debug)]
pub enum EventServiceError {
    #[error("tassl error")]
    TASSLError(#[from] TASSLError),

    #[error("channel error")]
    ChannelError(#[from] ChannelError),

    #[error("event service custom error")]
    CustomError {
        message: String,
    }
}

pub struct EventService {
    config: Config,
    block_notify_event_emitter: EventEmitter<ListenerResult>,
    block_notify_loop_lock: Arc<RwLock<Vec<u32>>>,
}

impl EventService {
    fn set_block_notify_loop_status(&self, group_id: u32, status: bool) {
        let rw_lock = self.block_notify_loop_lock.clone();
        let mut write_lock = rw_lock.write().unwrap();
        match write_lock.iter().position(|v| *v == group_id) {
            Some(index) => {
                write_lock.remove(index);
                if status {
                    write_lock.push(group_id);
                }
            },
            None => {
                if status {
                    write_lock.push(group_id);
                }
            },
        };
    }

    fn get_block_notify_loop_status(&self, group_id: u32) -> bool {
        let rw_lock = self.block_notify_loop_lock.clone();
        let read_lock = rw_lock.read().unwrap();
        read_lock.contains(&group_id)
    }

    pub fn new(config: &Config) -> EventService {
        EventService { 
            config: config.clone(),
            block_notify_event_emitter: EventEmitter::new(),
            block_notify_loop_lock: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn register_block_notify_listener(&mut self, group_id: u32, listener: fn(&ListenerResult)) {
        self.block_notify_event_emitter.on(&group_id.to_string(), listener);
    }

    pub fn remove_block_notify_listener(&mut self, group_id: u32) {
        self.block_notify_event_emitter.remove(&group_id.to_string());
    }

    pub async fn run_block_notify_loop(
        &self,
        group_id: u32,
        sleep_seconds: u32,
        max_retry_times: i32,
    ) {
        self.set_block_notify_loop_status(group_id, true);
        match open_tassl(&self.config) {
            Ok(tassl) => {
                let params = json!([format!("_block_notify_{:?}", group_id)]);
                let request_data = pack_channel_message(
                    &serde_json::to_vec(&params).unwrap(),
                    MessageType::AMOPClientTopics,
                );
                match tassl.write(&request_data) {
                    Ok(_) => {
                        let mut remain_retry_times = max_retry_times;
                        loop {
                            if self.get_block_notify_loop_status(group_id) {
                                let response = channel_read(&tassl).map(|buffer| {
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
                                });
                                match response {
                                    Ok(value) => {
                                        remain_retry_times = max_retry_times;
                                        self.block_notify_event_emitter.emit(
                                            &group_id.to_string(),
                                            &Ok(value),
                                        );
                                    },
                                    Err(err) => {
                                        self.block_notify_event_emitter.emit(
                                            &group_id.to_string(),
                                            &Err(EventServiceError::ChannelError(err)),
                                        );
                                        if max_retry_times != -1 && remain_retry_times == 0 {
                                            self.stop_block_notify_loop(group_id);
                                            break;
                                        } else {
                                            remain_retry_times -= 1;
                                            thread::sleep(Duration::from_millis((sleep_seconds * 1000) as u64));
                                        }
                                    },
                                };
                            } else {
                                let err = EventServiceError::CustomError {
                                    message: format!("block_notify_loop for group {:?} is stopped", group_id),
                                };
                                self.block_notify_event_emitter.emit(&group_id.to_string(), &Err(err));
                                self.stop_block_notify_loop(group_id);
                                break;
                            }
                        }
                    },
                    Err(err) => {
                        self.block_notify_event_emitter.emit(
                            &group_id.to_string(),
                            &Err(EventServiceError::TASSLError(err)),
                        );
                    }
                };
            },
            Err(err) =>  {
                self.block_notify_event_emitter.emit(
                    &group_id.to_string(),
                    &Err(EventServiceError::TASSLError(err)),
                );
            }
        };
    }

    pub fn stop_block_notify_loop(&self, group_id: u32) {
        self.set_block_notify_loop_status(group_id, false);
    }
}
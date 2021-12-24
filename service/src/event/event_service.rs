use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use thiserror::Error;
use serde_json::{Value as JSONValue, json};

use crate::config::Config;
use crate::tassl::TASSLError;
use crate::event::event_emitter::EventEmitter;
use crate::channel::{MessageType, ChannelError, channel_read, pack_channel_message, open_tassl, parse_block_notify_data};

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
                        self.set_block_notify_loop_status(group_id, true);
                        while self.get_block_notify_loop_status(group_id) {
                            match channel_read(&tassl).map(parse_block_notify_data) {
                                Ok(value) => {
                                    remain_retry_times = max_retry_times;
                                    self.block_notify_event_emitter.emit(
                                        &group_id.to_string(),
                                        &Ok(value),
                                    );
                                },
                                Err(err) => {
                                    if max_retry_times != -1 && remain_retry_times == 0 {
                                        let err = EventServiceError::CustomError {
                                            message: format!(
                                                "SSL_read invoked had failed over {:?} times, stopping the loop now",
                                                max_retry_times
                                            ),
                                        };
                                        self.block_notify_event_emitter.emit(
                                            &group_id.to_string(),
                                            &Err(err),
                                        );
                                        self.stop_block_notify_loop(group_id);
                                        break;
                                    } else {
                                        self.block_notify_event_emitter.emit(
                                            &group_id.to_string(),
                                            &Err(EventServiceError::ChannelError(err)),
                                        );
                                        remain_retry_times -= 1;
                                        thread::sleep(Duration::from_millis((sleep_seconds * 1000) as u64));
                                    }
                                },
                            };
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
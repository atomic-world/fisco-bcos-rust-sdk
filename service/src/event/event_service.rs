use std::thread;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use thiserror::Error;
use serde_json::{Value as JSONValue, json};

use crate::config::Config;
use crate::tassl::TASSLError;
use crate::event::event_emitter::EventEmitter;
use crate::channel::{MessageType, ChannelError, channel_read, pack_channel_message, open_tassl, parse_block_notify_data};

type EventEmitterResult = Result<JSONValue, EventServiceError>;

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

pub struct EventService<'l> {
    config: Config,
    event_emitter: EventEmitter<'l, EventEmitterResult>,
    event_loop_lock: Arc<RwLock<HashSet<String>>>,
}

impl<'l> EventService<'l> {
    fn get_block_notify_key(&self, group_id: u32) -> String {
        format!("_block_notify_{:?}", group_id)
    }

    fn get_event_loop_running_status(&self, key: &str) -> bool {
        let rw_lock = self.event_loop_lock.clone();
        let read_lock = rw_lock.read().unwrap();
        read_lock.contains(key)
    }

    fn set_event_loop_running_status(&self, key: &str, status: bool) {
        let rw_lock = self.event_loop_lock.clone();
        let mut write_lock = rw_lock.write().unwrap();
        if write_lock.contains(key) {
            write_lock.remove(key);
        }
        if status {
            write_lock.insert(key.to_owned());
        }
    }

    fn stop_event_loop(&self, key: &str) {
        self.set_event_loop_running_status(key, false);
    }

    async fn run_event_loop<F>(&self, key: &str, request_data: &[u8], sleep_seconds: u32, max_retry_times: i32, fn_result_parse: F)
        where F: FnOnce(Vec<u8>) -> JSONValue + Copy
    {
        match open_tassl(&self.config) {
            Ok(tassl) => {
                match tassl.write(&request_data) {
                    Ok(_) => {
                        let mut remain_retry_times = max_retry_times;
                        self.set_event_loop_running_status(key, true);
                        while self.get_event_loop_running_status(key) {
                            match channel_read(&tassl).map(fn_result_parse) {
                                Ok(value) => {
                                    remain_retry_times = max_retry_times;
                                    self.event_emitter.emit(
                                        key,
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
                                        self.event_emitter.emit(
                                            key,
                                            &Err(err),
                                        );
                                        self.stop_event_loop(key);
                                    } else {
                                        self.event_emitter.emit(
                                            key,
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
                        self.event_emitter.emit(
                            key,
                            &Err(EventServiceError::TASSLError(err)),
                        );
                    }
                };
            },
            Err(err) =>  {
                self.event_emitter.emit(
                    key,
                    &Err(EventServiceError::TASSLError(err)),
                );
            }
        };
    }

    pub fn new(config: &Config) -> EventService<'l> {
        EventService { 
            config: config.clone(),
            event_emitter: EventEmitter::new(),
            event_loop_lock: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn register_block_notify_listener<F>(&mut self, group_id: u32, listener: F) where F: Fn(&EventEmitterResult) + 'l {
        let key = self.get_block_notify_key(group_id);
        self.event_emitter.on(&key, listener);
    }

    pub fn remove_block_notify_listener(&mut self, group_id: u32) {
        let key = self.get_block_notify_key(group_id);
        self.event_emitter.remove(&key);
    }

    pub async fn run_block_notify_loop(&self, group_id: u32, sleep_seconds: u32, max_retry_times: i32) {
        let key = self.get_block_notify_key(group_id);
        let params = json!([format!("_block_notify_{:?}", group_id)]);
        let request_data = pack_channel_message(
            &serde_json::to_vec(&params).unwrap(),
            MessageType::AMOPClientTopics,
        );
        self.run_event_loop(&key, &request_data, sleep_seconds, max_retry_times, parse_block_notify_data).await
    }

    pub fn stop_block_notify_loop(&self, group_id: u32) {
        let key = self.get_block_notify_key(group_id);
        self.stop_event_loop(&key);
    }
}
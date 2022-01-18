use std::thread;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use thiserror::Error;
use serde_json::{Value as JSONValue, json};

use crate::config::Config;
use crate::tassl::TASSLError;
use crate::event::event_emitter::EventEmitter;
use crate::channel::{
    MessageType,
    ChannelError,
    open_tassl,
    channel_read,
    pack_channel_message,
    parse_block_notify_data,
};

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

    fn run_event_loop<F>(
        &self,
        key: &str,
        request_data: &[u8],
        sleep_seconds: u32,
        max_retry_times: i32,
        fn_result_parse: F,
    ) where F: FnOnce(Vec<u8>) -> JSONValue + Copy {
        match open_tassl(&self.config) {
            Ok(mut tassl) => {
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
                                    if !self.get_event_loop_running_status(key) {
                                        self.event_emitter.emit(
                                            key,
                                            &Err(EventServiceError::ChannelError(err)),
                                        );
                                        break;
                                    } else if max_retry_times != -1 && remain_retry_times == 0 {
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
                                        break;
                                    } else {
                                        let (need_reconnect, mut need_re_request) = match &err {
                                            ChannelError::TASSLError(TASSLError::ServiceError { error_code, .. }) => {
                                                // 1. 根据调试，关闭服务节点时，得到的错误状态码为 5（SSL_ERROR_SYSCALL）,
                                                // 所以在状态码为 6（SSL_ERROR_ZERO_RETURN）或 5（SSL_ERROR_SYSCALL）时，尝试重新发起连接操作。
                                                // 2. SSL 重连后，需要重新发送监听请求，为避免请求失败【状态码为 1（SSL_ERROR_SSL）】，
                                                // 故需要检测该值，以便在下次循环中尝试重新发送监听请求。
                                                match error_code {
                                                    Some(code) => (*code == 5 || *code == 6, *code == 1),
                                                    None => (false, false),
                                                }
                                            },
                                            _ => (false, false),
                                        };
                                        if need_reconnect || need_re_request {
                                            if need_reconnect {
                                                tassl.close();
                                                if let Err(err) = tassl.connect(&self.config.node.host, self.config.node.port) {
                                                    self.event_emitter.emit(
                                                        key,
                                                        &Err(EventServiceError::TASSLError(err)),
                                                    );
                                                } else {
                                                    need_re_request = true;
                                                }
                                            }
                                            if need_re_request {
                                                if let Err(err) = tassl.write(&request_data) {
                                                    self.event_emitter.emit(
                                                        key,
                                                        &Err(EventServiceError::TASSLError(err)),
                                                    );
                                                } else {
                                                    continue;
                                                }
                                            }
                                        } else {
                                            self.event_emitter.emit(
                                                key,
                                                &Err(EventServiceError::ChannelError(err)),
                                            );
                                        }
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

    pub fn register_block_notify_listener<F>(
        &self,
        group_id: u32,
        listener: F
    ) where F: Fn(&EventEmitterResult) + Send + Sync + 'l {
        let key = self.get_block_notify_key(group_id);
        self.event_emitter.on(&key, listener);
    }

    pub fn remove_block_notify_listener(&self, group_id: u32) {
        let key = self.get_block_notify_key(group_id);
        self.event_emitter.remove(&key);
    }

    ///
    /// sleep_seconds：链上数据读取失败后，进入下一轮监听前要等待的时间（单位为秒）。
    ///
    /// max_retry_times：链上数据读取失败后，最大重试次数，如果失败次数大于指定的值，将主动终止 loop。当值为 -1 时，表示无限循环。
    ///
    pub fn run_block_notify_loop(&self, group_id: u32, sleep_seconds: u32, max_retry_times: i32) {
        let key = self.get_block_notify_key(group_id);
        let params = json!([format!("_block_notify_{:?}", group_id)]);
        let request_data = pack_channel_message(
            &serde_json::to_vec(&params).unwrap(),
            MessageType::AMOPClientTopics,
        );
        self.run_event_loop(&key, &request_data, sleep_seconds, max_retry_times, parse_block_notify_data)
    }

    pub fn stop_block_notify_loop(&self, group_id: u32) {
        let key = self.get_block_notify_key(group_id);
        self.stop_event_loop(&key);
    }
}
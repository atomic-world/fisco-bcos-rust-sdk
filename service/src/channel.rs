use std::convert::TryInto;

use serde_json::{json, Value as JSONValue};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    config::Config,
    tassl::{TASSLError, TASSL},
};

#[derive(Error, Debug)]
pub enum ChannelError {
    #[error("tassl error")]
    TASSLError(#[from] TASSLError),

    #[error("std::str::Utf8Error")]
    StrFromUtf8Error(#[from] std::str::Utf8Error),

    #[error("serde_json::Error")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("std::array::TryFromSliceError")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
}

// 格式详情参见：
// https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/design/protocol_description.html#channelmessage-v2
#[derive(Debug)]
pub enum MessageType {
    RpcRequest,
    ClientRegisterEventLog,
    AMOPClientTopics,
    BlockNotify,
    EventLogPush,
}

impl MessageType {
    fn value(&self) -> i16 {
        match *self {
            MessageType::RpcRequest => 0x12_i16,
            MessageType::ClientRegisterEventLog => 0x15_i16,
            MessageType::AMOPClientTopics => 0x32_i16,
            MessageType::BlockNotify => 0x1001_i16,
            MessageType::EventLogPush => 0x1002_i16,
        }
    }
}

pub fn pack_channel_message(data: &Vec<u8>, message_type: MessageType) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.append(&mut Vec::from(((data.len() + 42) as u32).to_be_bytes()));
    buffer.append(&mut Vec::from(message_type.value().to_be_bytes()));
    buffer.append(&mut Uuid::new_v4().to_string().replace("-", "").into_bytes());
    buffer.append(&mut Vec::from(0_i32.to_be_bytes()));
    buffer.append(&mut data.clone());
    buffer
}

pub fn pack_amop_message(topic: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![];
    buffer.append(&mut Vec::from((1 + topic.len() as u8).to_be_bytes()));
    buffer.append(&mut topic.clone());
    buffer.append(&mut data.clone());
    buffer
}

pub fn open_tassl(config: &Config) -> Result<TASSL, TASSLError> {
    let tassl = TASSL::new(config.timeout_seconds);
    tassl.init();
    tassl.load_auth_files(
        &config.authentication.ca_cert,
        &config.authentication.sign_key,
        &config.authentication.sign_cert,
        &config.authentication.enc_key,
        &config.authentication.enc_cert,
    )?;
    tassl.connect(&config.node.host, config.node.port)?;
    Ok(tassl)
}

pub fn channel_read(tassl: &TASSL) -> Result<JSONValue, ChannelError> {
    let mut buffer: Vec<u8> = vec![0; 4];
    tassl.read(&mut buffer[0..])?;
    let buffer_size = u32::from_be_bytes(buffer.clone().as_slice().try_into()?) as usize;
    buffer.append(&mut vec![0; buffer_size - 4]);
    tassl.read(&mut buffer[4..])?;
    let message_type = i16::from_be_bytes((&buffer[4..6]).try_into()?);
    let data = Vec::from(&buffer[42..buffer_size]);
    if message_type == MessageType::BlockNotify.value() {
        Ok(parse_block_notify(data))
    } else if message_type == MessageType::ClientRegisterEventLog.value() {
        parse_client_register_event_log(data)
    } else {
        Ok(serde_json::from_str(
            std::str::from_utf8(&data)?.trim_end_matches("\n"),
        )?)
    }
}

fn parse_block_notify(buffer: Vec<u8>) -> JSONValue {
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
        "block_height": String::from(&values[1]).parse::<i32>().unwrap_or(-1),
    })
}

fn parse_client_register_event_log(buffer: Vec<u8>) -> Result<JSONValue, ChannelError> {
    let topic_len = u8::from_be_bytes(buffer[0].to_be_bytes()) as usize;
    Ok(serde_json::from_str(
        std::str::from_utf8(&buffer[topic_len..])?.trim_end_matches("\n"),
    )?)
}

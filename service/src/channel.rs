use uuid::Uuid;
use thiserror::Error;
use std::convert::TryInto;
use serde_json::Value as JSONValue;
use crate::Config;
use crate::tassl::{TASSL, TASSLError};


#[derive(Error, Debug)]
pub enum ChannelError {
    #[error("tassl error")]
    TASSLError(#[from] TASSLError),

    #[error("serde_json::Error")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("std::array::TryFromSliceError")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
}

// 格式详情参见：
// https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/design/protocol_description.html#channelmessage-v2
pub enum MessageType {
    RpcRequest,
    AMOPClientTopics,
}

impl MessageType {
    fn value(&self) -> i16 {
        match *self {
            MessageType::RpcRequest => 0x12_i16,
            MessageType::AMOPClientTopics => 0x32_i16,
        }
    }
}

fn pack_channel_message(data: &Vec<u8>, message_type: MessageType) -> Vec<u8> {
    let mut buffer:Vec<u8> = Vec::new();
    buffer.append(&mut Vec::from(((data.len() + 42) as u32).to_be_bytes()));
    buffer.append(&mut Vec::from(message_type.value().to_be_bytes()));
    buffer.append(&mut Uuid::new_v4().to_string().replace("-", "").into_bytes());
    buffer.append(&mut Vec::from(0_i32.to_be_bytes()));
    buffer.append(&mut data.clone());
    buffer
}

pub fn create_tassl(config: &Config) -> Result<TASSL, TASSLError> {
    let mut tassl = TASSL::new(config.timeout_seconds);
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

pub fn channel_fetch(
    tassl: &TASSL,
    params: &JSONValue,
    message_type: MessageType,
) -> Result<Vec<u8>, ChannelError> {
    let request_data = pack_channel_message(&serde_json::to_vec(&params)?, message_type);
    tassl.write(&request_data)?;
    let mut buffer:Vec<u8> = vec![0; 4];
    tassl.read(&mut buffer[0..])?;
    let buffer_size = u32::from_be_bytes(buffer.clone().as_slice().try_into()?) as usize;
    buffer.append(&mut vec![0; buffer_size - 4]);
    tassl.read(&mut buffer[4..])?;
    Ok(Vec::from(&buffer[42..buffer_size]))
}
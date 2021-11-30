use uuid::Uuid;
use serde_json::Value as JSONValue;
use async_trait::async_trait;
use std::convert::TryInto;

use crate::tassl::TASSL;
use crate::config::Config;
use crate::web3::{
    service_error::ServiceError,
    fetcher_trait::{FetcherTrait, parse_response},
};

// 格式详情参见：
// https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/design/protocol_description.html#channelmessage-v2
fn pack_channel_message(data: &Vec<u8>) -> Vec<u8> {
    let mut buffer:Vec<u8> = Vec::new();
    buffer.append(&mut Vec::from(((data.len() + 42) as u32).to_be_bytes()));
    buffer.append(&mut Vec::from(0x12_i16.to_be_bytes()));
    buffer.append(&mut Uuid::new_v4().to_string().replace("-", "").into_bytes());
    buffer.append(&mut Vec::from(0_i32.to_be_bytes()));
    buffer.append(&mut data.clone());
    buffer
}

pub struct ChannelFetcher {
    config: Config,
}

impl ChannelFetcher {
    pub fn new(config: &Config) -> ChannelFetcher {
        ChannelFetcher { config: config.clone() }
    }
}

#[async_trait]
impl FetcherTrait for ChannelFetcher {
    async fn fetch(&self, params: &JSONValue) -> Result<JSONValue, ServiceError> {
        let mut tassl = TASSL::new(self.config.timeout_seconds);
        tassl.init();
        tassl.load_auth_files(
            &self.config.authentication.ca_cert,
            &self.config.authentication.sign_key,
            &self.config.authentication.sign_cert,
            &self.config.authentication.enc_key,
            &self.config.authentication.enc_cert,
        )?;
        tassl.connect(&self.config.node.host, self.config.node.port)?;
        let request_data = pack_channel_message(&serde_json::to_vec(&params)?);
        tassl.write(&request_data)?;

        let mut buffer:Vec<u8> = vec![0; 4];
        tassl.read(&mut buffer[0..])?;
        let buffer_size = u32::from_be_bytes(buffer.clone().as_slice().try_into()?) as usize;
        buffer.append(&mut vec![0; buffer_size - 4]);
        tassl.read(&mut buffer[4..])?;
        let response: JSONValue = serde_json::from_slice(&buffer[42..buffer_size - 1])?;
        parse_response(&response)
    }
}


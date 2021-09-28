use uuid::Uuid;
use serde_json::Value as JSONValue;
use async_trait::async_trait;
use std::convert::TryInto;

use crate::tassl::TASSL;
use crate::web3::{fetcher_trait::FetcherTrait, service_error::ServiceError};


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
    host: String,
    port: i32,
    ca_cert_file: String,
    sign_cert_file: String,
    sign_key_file: String,
    enc_cert_file: String,
    enc_key_file: String,
    timeout_seconds: i64,
}

impl ChannelFetcher {
    pub fn new(
        host: &str,
        port: i32,
        ca_cert_file: &str,
        sign_key_file: &str,
        sign_cert_file: &str,
        enc_key_file: &str,
        enc_cert_file: &str,
        timeout_seconds: i64,
    ) -> ChannelFetcher {
        ChannelFetcher {
            port,
            timeout_seconds,
            host: host.to_owned(),
            ca_cert_file: ca_cert_file.to_owned(),
            sign_key_file: sign_key_file.to_owned(),
            sign_cert_file: sign_cert_file.to_owned(),
            enc_key_file: enc_key_file.to_owned(),
            enc_cert_file: enc_cert_file.to_owned(),
        }
    }
}

#[async_trait]
impl FetcherTrait for ChannelFetcher {
    async fn fetch(&self, params: &JSONValue) -> Result<JSONValue, ServiceError> {
        let mut tassl = TASSL::new(self.timeout_seconds);
        tassl.init();
        tassl.load_auth_files(
            &self.ca_cert_file,
            &self.sign_key_file,
            &self.sign_cert_file,
            &self.enc_key_file,
            &self.enc_cert_file,
        )?;
        tassl.connect(&self.host, self.port)?;
        let request_data = pack_channel_message(&serde_json::to_vec(&params)?);
        tassl.write(&request_data)?;

        let mut buffer:Vec<u8> = vec![0; 4];
        tassl.read(&mut buffer[0..])?;
        let buffer_size = u32::from_be_bytes(buffer.clone().as_slice().try_into()?) as usize;
        buffer.append(&mut vec![0; buffer_size - 4]);
        tassl.read(&mut buffer[4..])?;
        let response: JSONValue = serde_json::from_slice(&buffer[42..buffer_size - 1])?;
        let result = &response["result"];
        let error = &response["error"];
        match error.is_null() {
            true => Ok(result.clone()),
            false => {
                Err(ServiceError::FiscoBcosError {
                    code: error["code"].as_i64().unwrap(),
                    message: error["message"].to_string(),
                })
            }
        }
    }
}


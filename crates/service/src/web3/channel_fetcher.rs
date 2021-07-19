use uuid::Uuid;
use std::path::Path;
use serde_json::Value;
use async_trait::async_trait;
use std::io::{Write, Read};
use std::net::TcpStream;
use openssl::ssl::{SslMethod, SslVerifyMode, SslFiletype, SslConnector, SslStream};

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
    ca_file: String,
    cert_file: String,
    key_file: String,
}

impl ChannelFetcher {
    pub fn new(host: &str, port: i32, ca_file: &str, cert_file: &str, key_file: &str) -> ChannelFetcher {
        ChannelFetcher {
            port,
            host: host.to_owned(),
            ca_file: ca_file.to_owned(),
            cert_file: cert_file.to_owned(),
            key_file: key_file.to_owned(),
        }
    }
}

#[async_trait]
impl FetcherTrait for ChannelFetcher {
    async fn fetch(&self, params: &Value) -> Result<Value, ServiceError> {
        let ca_file_path = Path::new(&self.ca_file);
        let curve = openssl::ec::EcKey::from_curve_name(openssl::nid::Nid::SECP256K1)?;
        let mut ssl_builder = SslConnector::builder(SslMethod::tls())?;
        ssl_builder.set_verify(SslVerifyMode::NONE);
        ssl_builder.set_tmp_ecdh(&curve)?;
        ssl_builder.set_ca_file(ca_file_path)?;
        ssl_builder.set_certificate_chain_file(&Path::new(&self.cert_file))?;
        ssl_builder.set_private_key_file(&Path::new(&self.key_file), SslFiletype::PEM)?;
        let ssl = ssl_builder.build().configure()?.into_ssl(&self.host)?;
        let tcp_stream = TcpStream::connect(format!("{}:{}", self.host, self.port))?;
        let mut ssl_stream = SslStream::new(ssl, tcp_stream)?;
        ssl_stream.connect()?;

        let request_data = pack_channel_message(&serde_json::to_vec(&params)?);
        ssl_stream.write(&request_data)?;
        let mut buffer_size = 0;
        let mut buffer:Vec<u8> = Vec::new();
        'outer: loop {
            let start_index = buffer.len();
            buffer.append(&mut vec![0; 256]);
            let read_size = ssl_stream.read(&mut buffer[start_index..])?;
            buffer_size += read_size;
            if read_size < 256 {
                break 'outer;
            }
        }
        let response: Value = serde_json::from_slice(&buffer[42..buffer_size - 1])?;
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


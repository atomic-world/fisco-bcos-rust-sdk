use uuid::Uuid;
use std::pin::Pin;
use std::path::Path;
use async_trait::async_trait;
use serde_json::Value;
use tokio_openssl::SslStream;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use openssl::ssl::{SslMethod, SslVerifyMode, SslFiletype, SslConnector};

use crate::service_trait::ServiceTrait;
use crate::service_error::ServiceError;

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

pub struct ChannelService {
    host: String,
    port: i32,
    ca_file: String,
    cert_file: String,
    key_file: String,
}

impl ChannelService {
    pub fn new(host: &str, port: i32, ca_file: &str, cert_file: &str, key_file: &str) -> ChannelService {
        ChannelService {
            host: host.to_owned(),
            port: port,
            ca_file: ca_file.to_owned(),
            cert_file: cert_file.to_owned(),
            key_file: key_file.to_owned(),
        }
    }
}

#[async_trait]
impl ServiceTrait for ChannelService {
    async fn fetch(&self, params: &Value) -> Result<Value, ServiceError> {
        let ca_file_path = Path::new(&self.ca_file);
        let mut ssl_builder = SslConnector::builder(SslMethod::tls())?;
        let curve = openssl::ec::EcKey::from_curve_name(openssl::nid::Nid::SECP256K1)?;
        ssl_builder.set_verify(SslVerifyMode::NONE);
        ssl_builder.set_tmp_ecdh(&curve)?;
        ssl_builder.set_ca_file(ca_file_path)?;
        ssl_builder.set_certificate_chain_file(&Path::new(&self.cert_file))?;
        ssl_builder.set_private_key_file(&Path::new(&self.key_file), SslFiletype::PEM)?;

        let ssl = ssl_builder.build().configure()?.into_ssl(&self.host)?;
        let tcp_stream = TcpStream::connect(format!("{}:{}", self.host, self.port)).await?;
        let mut ssl_stream = SslStream::new(ssl, tcp_stream)?;
        Pin::new(&mut ssl_stream).connect().await?;

        let channel_message = pack_channel_message(&serde_json::to_vec(&params)?);
        ssl_stream.write_all(&channel_message).await?;

        let mut response = vec![];
        ssl_stream.read_to_end(&mut response).await?;
        let data: Value = serde_json::from_slice(&Vec::from(&response[42..]))?;
        let result = &data["result"];
        let error = &data["error"];
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

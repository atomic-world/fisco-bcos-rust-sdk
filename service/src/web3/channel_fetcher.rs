use serde_json::Value as JSONValue;
use async_trait::async_trait;

use crate::config::Config;
use crate::web3::{
    service::ServiceError,
    fetcher_trait::{FetcherTrait, parse_response},
};
use crate::channel::{MessageType, create_tassl, channel_fetch};

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
        let tassl = create_tassl(&self.config)?;
        let data= channel_fetch(&tassl, params, MessageType::RpcRequest)?;
        tassl.close();
        let response= serde_json::from_str(std::str::from_utf8(&data)?.trim_end_matches("\n"))?;
        parse_response(&response)
    }
}


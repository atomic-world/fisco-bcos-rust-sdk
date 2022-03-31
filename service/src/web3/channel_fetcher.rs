use async_trait::async_trait;
use serde_json::Value as JSONValue;

use crate::{
    channel::{channel_read, open_tassl, pack_channel_message, MessageType},
    config::Config,
    web3::{
        fetcher_trait::{parse_response, FetcherTrait},
        service::ServiceError,
    },
};

pub struct ChannelFetcher {
    config: Config,
}

impl ChannelFetcher {
    pub fn new(config: &Config) -> ChannelFetcher {
        ChannelFetcher {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl FetcherTrait for ChannelFetcher {
    async fn fetch(&self, params: &JSONValue) -> Result<JSONValue, ServiceError> {
        let tassl = open_tassl(&self.config)?;
        let request_data =
            pack_channel_message(&serde_json::to_vec(&params)?, MessageType::RpcRequest);
        tassl.write(&request_data)?;
        let response = channel_read(&tassl)?;
        tassl.close();
        parse_response(&response)
    }
}

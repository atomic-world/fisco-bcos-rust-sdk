use async_trait::async_trait;
use serde_json::Value as JSONValue;
use crate::web3::service_error::ServiceError;

#[async_trait]
pub trait FetcherTrait {
    async fn fetch(&self, params: &JSONValue) -> Result<JSONValue, ServiceError>;
}
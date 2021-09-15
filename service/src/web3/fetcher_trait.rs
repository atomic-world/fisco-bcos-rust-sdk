use async_trait::async_trait;
use serde_json::Value;
use crate::web3::service_error::ServiceError;

#[async_trait]
pub trait FetcherTrait {
    async fn fetch(&self, params: &Value) -> Result<Value, ServiceError>;
}
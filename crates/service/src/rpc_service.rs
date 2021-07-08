use serde_json::Value;
use async_trait::async_trait;
use hyper::http::{Request, Method};
use hyper::{Body, Client};

use crate::service_trait::ServiceTrait;
use crate::service_error::ServiceError;

pub struct RPCService {
  node: String,
}

impl RPCService {
    pub fn new(node: &str) -> RPCService {
        RPCService {
            node: node.to_owned(),
        }
    }
}

#[async_trait]
impl ServiceTrait for RPCService {
    async fn fetch(&self, params: &Value) -> Result<Value, ServiceError> {
        let request_body = serde_json::to_string(&params)?;
        let request = Request::builder()
            .method(Method::POST)
            .uri(&self.node)
            .body(Body::from(request_body))?;
        let client = Client::new();
        let response = client.request(request).await?;
        let response_body = hyper::body::to_bytes(response.into_body()).await?;
        let data: Value = serde_json::from_slice(&response_body.to_vec())?;
        let error = &data["error"];
        if error.is_null() {
            Ok(data)
        } else {
            Err(ServiceError::FiscoBcosError {
                code: error["code"].as_i64().unwrap(),
                message: error["message"].to_string(),
            })
        }
    }
}
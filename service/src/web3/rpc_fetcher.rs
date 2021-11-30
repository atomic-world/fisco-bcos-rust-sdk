use serde_json::Value as JSONValue;
use async_trait::async_trait;
use hyper::{Body, Client};
use hyper::http::{Request, Method};
use crate::web3::{
    service_error::ServiceError,
    fetcher_trait::{FetcherTrait, parse_response},
};

pub struct RPCFetcher  {
    host: String,
    port: i32,
}

impl RPCFetcher {
    pub fn new(host: &str, port: i32) -> RPCFetcher {
        RPCFetcher {
            host: host.to_owned(),
            port,
        }
    }
}

#[async_trait]
impl FetcherTrait for RPCFetcher {
    async fn fetch(&self, params: &JSONValue) -> Result<JSONValue, ServiceError> {
        let request_body = serde_json::to_string(&params)?;
        let request = Request::builder()
            .method(Method::POST)
            .uri(format!("http://{}:{}", self.host, self.port))
            .body(Body::from(request_body))?;
        let client = Client::new();
        let response = client.request(request).await?;
        let response_body = hyper::body::to_bytes(response.into_body()).await?;
        let data: JSONValue = serde_json::from_slice(&response_body.to_vec())?;
        parse_response(&data)
    }
}
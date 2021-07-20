pub mod web3;
pub mod helpers;

use std::{fs, path::Path};
use serde_json::Value;
use helpers::parse_serde_json_string_value;

///
/// 配置文件格式为：
///
/// ```json
/// {
///    "service_type": "rpc",
///    "node": {
///        "host": "127.0.0.1",
///        "port": 8545
///    },
///    "authentication": {
///        "key": "./sdk.key",
///        "cert": "./sdk.crt",
///        "ca": "./ca.crt"
///    },
///    "groupID": 1,
///    "chainID": 1
/// }
/// ```
///
pub fn create_web3_service(config_file_path: &str) -> Result<web3::service::Service, web3::service_error::ServiceError>  {
    let config_path = Path::new(config_file_path);
    let config: Value = serde_json::from_slice(fs::read(config_path)?.as_slice())?;
    let node = &config["node"];
    let host = node["host"].as_str().unwrap();
    let port = node["port"].as_u64().unwrap() as i32;
    if parse_serde_json_string_value(&config["service_type"]).eq("rpc") {
        let fetcher = web3::rpc_fetcher::RPCFetcher::new(host, port);
        Ok(web3::service::Service::new(Box::new(fetcher)))
    } else {
        let config_dir_path= config_path.parent().unwrap();
        let authentication = &config["authentication"];
        let fetcher = web3::channel_fetcher::ChannelFetcher::new(
            host,
            port,
            config_dir_path.join(
                authentication["ca"].as_str().unwrap()
            ).into_os_string().to_str().unwrap(),
            config_dir_path.clone().join(
                authentication["cert"].as_str().unwrap()
            ).into_os_string().to_str().unwrap(),
            config_dir_path.clone().join(
                authentication["key"].as_str().unwrap()
            ).into_os_string().to_str().unwrap(),
        );
        Ok(web3::service::Service::new(Box::new(fetcher)))
    }
}
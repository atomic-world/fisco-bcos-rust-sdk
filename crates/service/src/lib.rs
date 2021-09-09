pub mod web3;
pub mod account;
pub mod abi;
pub mod transaction;
mod helpers;

use std::{fs, path::Path};
use serde_json::Value;
use helpers::parse_serde_json_string_value;

fn get_real_file_path(base_path: &Path, file_path: &Value) -> String {
    base_path.join(file_path.as_str().unwrap()).into_os_string().into_string().unwrap()
}

///
/// 根据配置文件创建 web3 service 服务实例。
///
/// 配置文件格式如下所示：
///
/// ```json
/// {
///    "service_type": "rpc",
///    "node": {
///        "host": "127.0.0.1",
///        "port": 8545
///    },
///    "account": "./accounts/alice.pem",
///    "authentication": {
///        "key": "./sdk.key",
///        "cert": "./sdk.crt",
///        "ca": "./ca.crt"
///    },
///    "group_id": 1,
///    "chain_id": 1,
//     "timeout_seconds": 10
/// }
/// ```
///
pub fn create_web3_service(config_file_path: &str) -> Result<web3::service::Service, web3::service_error::ServiceError>  {
    let config_path = Path::new(config_file_path);
    let config: Value = serde_json::from_slice(fs::read(config_path)?.as_slice())?;
    let group_id= config["group_id"].as_u64().unwrap() as u32;
    let chain_id = config["chain_id"].as_u64().unwrap() as u32;
    let timeout_seconds = config["timeout_seconds"].as_u64().unwrap();
    let node = &config["node"];
    let host = node["host"].as_str().unwrap();
    let port = node["port"].as_u64().unwrap() as i32;
    let config_dir_path= config_path.parent().unwrap();
    let account_pem_path = get_real_file_path(config_dir_path, &config["account"]);
    if parse_serde_json_string_value(&config["service_type"]).eq("rpc") {
        let fetcher = web3::rpc_fetcher::RPCFetcher::new(host, port);
        web3::service::Service::new(
            group_id,
            chain_id,
            timeout_seconds,
            &account_pem_path,
            Box::new(fetcher)
        )
    } else {
        let authentication = &config["authentication"];
        let fetcher = web3::channel_fetcher::ChannelFetcher::new(
            host,
            port,
            &get_real_file_path(config_dir_path, &authentication["ca"]),
            &get_real_file_path(config_dir_path, &authentication["cert"]),
            &get_real_file_path(config_dir_path, &authentication["key"]),
        );
        web3::service::Service::new(
            group_id,
            chain_id,
            timeout_seconds,
            &account_pem_path,
            Box::new(fetcher)
        )
    }
}
pub mod web3;
pub mod account;
pub mod abi;
pub mod transaction;
pub mod helpers;
pub use serde_json;

use std::{fs, path::Path};
use serde_json::Value as JSONValue;
use web3::{
    service::Service as Web3Service,
    service_error::ServiceError as Web3ServiceError,
    rpc_fetcher::RPCFetcher as Web3RPCFetcher,
    channel_fetcher::ChannelFetcher as Web3ChannelFetcher,
};
use helpers::parse_json_string;

fn get_real_file_path(base_path: &Path, file_path: &JSONValue) -> String {
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
///    "sm_crypto": false,
///    "group_id": 1,
///    "chain_id": 1,
//     "timeout_seconds": 10
/// }
/// ```
///
pub fn create_web3_service(config_file_path: &str) -> Result<Web3Service, Web3ServiceError>  {
    let config_path = Path::new(config_file_path);
    let config: JSONValue = serde_json::from_slice(fs::read(config_path)?.as_slice())?;
    let group_id= config["group_id"].as_u64().unwrap() as u32;
    let chain_id = config["chain_id"].as_u64().unwrap() as u32;
    let timeout_seconds = config["timeout_seconds"].as_u64().unwrap();
    let node = &config["node"];
    let host = node["host"].as_str().unwrap();
    let port = node["port"].as_u64().unwrap() as i32;
    let config_dir_path= config_path.parent().unwrap();
    let account_pem_path = get_real_file_path(config_dir_path, &config["account"]);
    let sm_crypto = config["sm_crypto"].as_bool().unwrap_or(false);

    if parse_json_string(&config["service_type"]).eq("rpc") {
        let fetcher = Web3RPCFetcher::new(host, port);
        Web3Service::new(
            group_id,
            chain_id,
            timeout_seconds,
            &account_pem_path,
            sm_crypto,
            Box::new(fetcher)
        )
    } else {
        let authentication = &config["authentication"];
        let fetcher = Web3ChannelFetcher::new(
            host,
            port,
            &get_real_file_path(config_dir_path, &authentication["ca"]),
            &get_real_file_path(config_dir_path, &authentication["cert"]),
            &get_real_file_path(config_dir_path, &authentication["key"]),
        );
        Web3Service::new(
            group_id,
            chain_id,
            timeout_seconds,
            &account_pem_path,
            sm_crypto,
            Box::new(fetcher)
        )
    }
}
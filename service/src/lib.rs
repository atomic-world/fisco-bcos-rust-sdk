pub mod web3;
pub mod account;
pub mod abi;
pub mod transaction;
pub mod helpers;
pub mod tassl;
pub mod config;
pub mod precompiled;
pub mod event;
pub mod channel;

pub use ethabi;
pub use serde_json;

use web3::{
    rpc_fetcher::RPCFetcher as Web3RPCFetcher,
    channel_fetcher::ChannelFetcher as Web3ChannelFetcher,
    service::{ Service as Web3Service, ServiceError as Web3ServiceError },
};
use config::{Config, create_config_with_file};

///
/// 根据配置文件创建 web3 service 服务实例。
///
/// 完整配置文件格式如下所示：
///
/// ```json
/// {
///    "service_type": "rpc",
///    "node": {
///        "host": "127.0.0.1",
///        "port": 8545
///    },
///    "account": "./accounts/alice.pem",
///    "contract":  {
///        "solc": "./bin/solc-0.4.25",
///        "source": "./contracts",
///        "output": "./contracts/.output"
///    },
///    "authentication": {
///        "ca_cert": "./authentication/gm/gmca.crt",
///        "sign_cert": "./authentication/gm/gmsdk.crt",
///        "sign_key": "./authentication/gm/gmsdk.key",
///        "enc_key": "./authentication/gm/gmensdk.key",
///        "enc_cert": "./authentication/gm/gmensdk.crt"
///    },
///    "sm_crypto": false,
///    "group_id": 1,
///    "chain_id": 1,
///    "timeout_seconds": 10
/// }
/// ```
///
pub fn create_web3_service(config_file_path: &str) -> Result<Web3Service, Web3ServiceError>  {
    let config = create_config_with_file(config_file_path)?;
    if config.service_type.eq("rpc") {
        let fetcher = Web3RPCFetcher::new(&config.node.host, config.node.port);
        Web3Service::new(&config, Box::new(fetcher)
        )
    } else {
        let fetcher = Web3ChannelFetcher::new(&config);
        Web3Service::new(&config, Box::new(fetcher))
    }
}
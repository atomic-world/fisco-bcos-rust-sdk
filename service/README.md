# fisco-bcos-service

Rust SDK For FISCO BCOS 2.7.0+

# 安装

```toml
[dependencies]
fisco-bcos-service = "0.1"
```

此 crate 使用了 [TASSL](https://github.com/jntass/TASSL) 来处理 `TLS` 连接，在 `Linux` 或 `Macos` 下无需做任何额外操作，其他环境下则需要指定以下环境变量：

* `TASSL_LIB_PATH`：lib 库加载路径。
* `TASSL_INCLUDE_PATH`：头文件检索路径。
* `TASSL_LIB_KIND`：lib 库类型，可用值为：`static` 或 `dylib`。

在 `Linux` 或 `Macos` 下，如果你已经编译好了 `TASSL`，也可以通过指定以上环境变量来缩短编译时间。

# 使用

## Web3Service

### 通过方法 `create_web3_service` 获得 `Web3Service` 实例

```rust
use fisco_bcos_service::create_web3_service;

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
```

其中 `config_file_path` 为包含以下信息的 `json` 文件路径：

```json
{
    "service_type": "rpc",
    "node": {
        "host": "127.0.0.1",
        "port": 8545
    },
    "account": "./accounts/alice.pem",
    "authentication": {
        "ca_cert": "./authentication/gm/gmca.crt",
        "sign_cert": "./authentication/gm/gmsdk.crt",
        "sign_key": "./authentication/gm/gmsdk.key",
        "enc_key": "./authentication/gm/gmensdk.key",
        "enc_cert": "./authentication/gm/gmensdk.crt"
    },
    "sm_crypto": false,
    "group_id": 1,
    "chain_id": 1,
    "timeout_seconds": 10
}
```

每一项的解释如下：

* `service_type`：服务类型，可用值为：`rpc` 或 `channel`。

* `node`：服务节点信息，包含以下属性：

    * `host`：服务节点主机地址。
    * `port`：服务节点端口号。

* `account`：用户证书文件路径（仅支持 `pem` 格式）。

* `authentication`：节点验证配置信息（仅在 `service_type` 为 `channel` 时设置），包含以下属性：

    * `ca_cert`：CA 证书文件路径。
    * `sign_cert`：签名证书文件路径。
    * `sign_key`：签名密钥文件路径。
    * `enc_key`：enc 密钥文件路径（`非国密`模式下无需设置）。
    * `enc_cert`：enc 证书文件路径（`非国密`模式下无需设置）。

* `sm_crypto`：交易签名是否使用`国密`。
* `group_id`：组 ID。
* `chain_id`：链 ID。
* `timeout_seconds`： 网络过期时间（单位为秒）。

**注：配置项中 `account` 及 `authentication` 中的路径如果使用相对路径，它的参考路径为该配置文件所在路径。**

### 自行实例化 `Web3Service`

如果你不想通过 `create_web3_service` 创建 `Web3Service` 实例，亦可参照 [create_web3_service](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/lib.rs#L56-L99) 的实现自行实例化。

### API 列表

* 列表详情参见：[Web3Service](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/web3/service.rs#L57-L451)。

* 除接口 `call`、`deploy` 的返回值与 FISCO BCOS JSON-RPC 相关方法不一致外，其余方法的返回值结构参见 [FISCO BCOS JSON-RPC](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/api.html)。
* 接口 `call`、`send_raw_transaction`、`send_raw_transaction_and_get_proof`、`deploy` 中有关 `abi_path`、`abi_bin_path` 为合约编译后的 `abi` 及 `bin` 路径，目前暂不支持自动编译，请通过 [download_solc.sh](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/bin/download_solc.sh) 下载编译器，并手动执行合约编译。
* 接口 `deploy` 的返回值结构如下所示：

  ```json
  {
    "status": "0x0",
    "transactionHash": "0x31ad4fd454fbe72557cbcb55bde067cfcd80fa43e9d97bdf2c13d2007f066370",
    "contractAddress": "0x62195d0f77f66c445c4878b845f55d266875705d"
  }
  ```
* 接口中的 `Token` 实为 `ethabi::token::Token`，具体使用参见 [ethabi token](https://github.com/rust-ethereum/ethabi/blob/v14.1.0/ethabi/src/token/token.rs#L227-L299)，在使用过程中无需安装 `ethabi` 依赖，只需 `use fisco_bcos_service::ethabi::token::Token` 即可。


**注：所有接口均为异步调用（使用了 Rust 的 [async](https://rust-lang.github.io/async-book/) 特性）。**

# License

[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0.txt)

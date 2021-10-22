# fisco-bcos-cli

Rust 版 FISCO BCOS 可交互式命令行工具。

# 安装

```shell
$ cargo install --force fisco-bcos-cli
```

此 crate 使用了 [TASSL](https://github.com/jntass/TASSL) 来处理 `TLS` 连接，在 `Linux` 或 `Macos` 下无需做任何额外操作，其他环境下则需要指定以下环境变量：

* `TASSL_LIB_PATH`：lib 库加载路径。
* `TASSL_INCLUDE_PATH`：头文件检索路径。
* `TASSL_LIB_KIND`：lib 库类型，可用值为：`static` 或 `dylib`。

在 `Linux` 或 `Macos` 下，如果你已经编译好了 `TASSL`，也可以通过指定以上环境变量来缩短编译时间。

# 使用

```shell
$ cargo fisco-bcos-cli
```

执行上述命令，你将进入以下交互页面：

```shell
Welcome to cli for FISCO BCOS. Type help to get help
>>
```

输入 `help`，获取帮助信息：

```shell
>> help

1. Use set_config function to initialize the environment(e.g., set_config "./config/config.json")
2. Use the below functions to interact with the FISCO BCOS service: ["get_client_version", "get_block_number", "get_pbft_view", "get_sealer_list", "get_observer_list", "get_consensus_status", "get_sync_status", "get_peers", "get_group_peers", "get_node_id_list", "get_group_list", "get_block_by_hash", "get_block_by_number", "get_block_header_by_hash", "get_block_header_by_number", "get_block_hash_by_number", "get_transaction_by_hash", "get_transaction_by_block_hash_and_index", "get_transaction_by_block_number_and_index", "get_transaction_receipt", "get_pending_transactions", "get_pending_tx_size", "get_code", "get_total_transaction_count", "call", "send_raw_transaction", "send_raw_transaction_and_get_proof", "deploy", "get_system_config_by_key", "get_transaction_by_hash_with_proof", "get_transaction_receipt_by_hash_with_proof", "generate_group", "start_group", "stop_group", "remove_group", "recover_group", "query_group_status", "get_node_info", "get_batch_receipts_by_block_number_and_range", "get_batch_receipts_by_block_hash_and_range"](e.g., get_block_by_number "0x0")
3. Type help to get help
4. Type CTRL-C or CTRL-D to quit
5. Visit https://github.com/kkawakam/rustyline#actions to get more actions
```

在与 `FISCO BCOS service` 交互前，需先调用 `set_config` 来设置环境信息（配置信息详情参见：[服务配置](https://github.com/atomic-world/fisco-bcos-rust-sdk/tree/main/service#%E9%80%9A%E8%BF%87%E6%96%B9%E6%B3%95-create_web3_service-%E8%8E%B7%E5%BE%97-web3service-%E5%AE%9E%E4%BE%8B)），比如：

```shell
>> set_config ./configs/config.json
```

而后便可调用 `FISCO BCOS service` 交互方法对链上数据进行交互，比如：

``` shell
>> get_client_version

Object({"Build Time": String("20210201 10:15:37"), "Build Type": String("Darwin/appleclang/RelWithDebInfo"), "Chain Id": String("1"), "FISCO-BCOS Version": String("2.7.2"), "Git Branch": String("HEAD"), "Git Commit Hash": String("4c8a5bbe44c19db8a002017ff9dbb16d3d28e9da"), "Supported Version": String("2.7.2")})
```

`FISCO BCOS service` 交互方法参数信息参见：

* Web3 服务：[https://docs.rs/fisco_bcos_service/0.1.0/web3/service/struct.Service.html](https://docs.rs/fisco_bcos_service/0.1.0/web3/service/struct.Service.html)。


`FISCO BCOS service` 交互方法参数注意事项：

* 方法名与参数、参数与参数之间以`空格`分割（比如：`call ./contracts/HelloWorldV4.abi 0x62195d0f77f66c445c4878b845f55d266875705d get
`）。
* `call`、`send_raw_transaction`、`send_raw_transaction_and_get_proof`、`deploy` 等方法的签名最后一个参数为 `Vec<Token>`，在调用时直接将其拆分为多个参数，然后以空格分开即可（比如：`send_raw_transaction ./contracts/HelloWorldV4.abi 0x62195d0f77f66c445c4878b845f55d266875705d set hello`）。
* 因为参数之间以`空格`分割，因此如某个参数内部需要有空格，请使用双引号或单引号包裹（比如：`send_raw_transaction ./contracts/HelloWorldV4.abi 0x62195d0f77f66c445c4878b845f55d266875705d set "hello world"` 或 `send_raw_transaction ./contracts/HelloWorldV4.abi 0x62195d0f77f66c445c4878b845f55d266875705d set 'hello world'`）。

# License

[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0.txt)
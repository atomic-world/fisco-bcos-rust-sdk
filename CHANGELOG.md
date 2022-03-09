# 0.4.1 2022-03-09
## Features

* 移除 `EventLogParam#remove_topic`、`EventLogParam#remove_address` 及 `EventEmitter#remove` 中不必要的读锁逻辑。

# 0.4.0 2022-03-09
## Features

* 支持合约事件监听。
* CLI 工具交互优化。

# 0.3.0 2021-12-17

## Features

* 提供[预编译合约](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/precompiled_contract.html)接口。


# 0.2.0 2021-11-24

## Features

* 支持合约编译，可将 Solidity 合约编译成 `abi` 和 `bin` 文件。
* 优化 Web3Service 中的 `call`、`send_transaction`、`send_raw_transaction`、`deploy` 接口参数（根据合约名称自动获取合约的 `abi` 及 `bin` 信息）。

# 0.1.0 2021-10-22

## Features

* 提供 [JSON-RPC](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/api.html) 接口的 Rust API。
* 支持国密和非国密的 [Channel](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/design/protocol_description.html#channelmessage) 协议。
* 支持国密和非国密下部署、调用 Solidity 合约的 Rust API。
* 提供交互式的 CLI 工具，支持用户部署及调用合约、管理区块链状态等操作。
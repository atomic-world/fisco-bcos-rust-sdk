use std::fmt::Debug;
use std::str::FromStr;
use futures::future::BoxFuture;
use fisco_bcos_service::{
    create_web3_service,
    serde_json::{json, Value as JSONValue},
    web3::{service::Service as Web3Service, service_error::ServiceError as Web3ServiceError},
};

pub(crate) struct Cli {
    web3_service: Option<Web3Service>,
}

fn valid_args_len(command_parts_length: usize, min_len: usize) -> bool {
    if command_parts_length - 1 < min_len {
        println!("\nArgument count should not less than {:}\n", min_len);
        false
    } else {
        true
    }
}

fn convert_str_to_bool(value: &str) -> bool {
    value.eq_ignore_ascii_case("true")
}

fn convert_str_to_number<T:FromStr>(value: &str, default: T) -> T {
    String::from(value).parse::<T>().unwrap_or(default)
}

fn convert_str_to_json(value: &str) -> JSONValue {
    fisco_bcos_service::serde_json::from_str::<JSONValue>(value).unwrap_or(json!(null))
}

impl Cli {
    fn set_config(&mut self, config_path: &str) {
        match create_web3_service(config_path) {
            Ok(web3_service) => self.web3_service = Some(web3_service),
            Err(error) => println!("\n Web3 Service initialize error: {:?}\n", error),
        };
    }

    async fn call_web3_service<'a, T, F>(&'a self, f: F)
        where
            T: Debug,
            F: FnOnce(&'a Web3Service) -> BoxFuture<'a, Result<T, Web3ServiceError>>
    {
        if self.web3_service.is_some() {
            let web3_service = self.web3_service.as_ref().unwrap();
            match f(web3_service).await {
                Ok(data) => println!("\n{:?}\n", data),
                Err(error) => println!("\nError: {:?}\n", error),
            };
        } else {
            println!("\nError: Please initialize the environment with set_config function first\n");
        }
    }

    fn echo_help(&self) {
        println!("\n1. Use set_config function to initialize the environment(e.g., set_config \"./config/config.json\")");
        println!(
            "2. Use the below functions to interact with the FISCO BCOS service: {:?}(e.g., get_block_by_number \"0x0\")",
            vec![
                "get_client_version", "get_block_number", "get_pbft_view",
                "get_sealer_list", "get_observer_list", "get_consensus_status",
                "get_sync_status", "get_peers", "get_group_peers",
                "get_node_id_list", "get_group_list", "get_block_by_hash",
                "get_block_by_number", "get_block_header_by_hash", "get_block_header_by_number",
                "get_block_hash_by_number", "get_transaction_by_hash", "get_transaction_by_block_hash_and_index",
                "get_transaction_by_block_number_and_index", "get_transaction_receipt", "get_pending_transactions",
                "get_pending_tx_size", "get_code", "get_total_transaction_count",
                "call", "send_raw_transaction", "send_raw_transaction_and_get_proof", "deploy",
                "get_system_config_by_key", "get_transaction_by_hash_with_proof", "get_transaction_receipt_by_hash_with_proof",
                "generate_group", "start_group", "stop_group",
                "remove_group", "recover_group", "query_group_status",
                "get_node_info", "get_batch_receipts_by_block_number_and_range", "get_batch_receipts_by_block_hash_and_range",
            ],
        );
        println!("3. Type help to get help");
        println!("4. Type CTRL-C or CTRL-D to quit");
        println!("5. Visit https://github.com/kkawakam/rustyline#actions to get more actions\n");
    }

    pub(crate) fn new() -> Cli {
        Cli {  web3_service: None }
    }

    pub(crate) async fn run_command(&mut self, command: &str) {
        let command_parts: Vec<&str> = command.split_whitespace()
            .into_iter()
            .map(|item| item.trim_start_matches("\"").trim_end_matches("\""))
            .collect();
        let command_parts_length = command_parts.len();
        match command_parts[0] {
            "help" => self.echo_help(),
            "set_config" => {
                if valid_args_len(command_parts_length, 1) {
                    self.set_config(command_parts[1]);
                }
            },
            "get_client_version" => {
                self.call_web3_service(|service| Box::pin(service.get_client_version())).await;
            },
            "get_block_number" => {
                self.call_web3_service(|service| Box::pin(service.get_block_number())).await;
            },
            "get_pbft_view" => {
                self.call_web3_service(|service| Box::pin(service.get_pbft_view())).await;
            },
            "get_sealer_list" => {
                self.call_web3_service(|service| Box::pin(service.get_sealer_list())).await;
            },
            "get_observer_list" => {
                self.call_web3_service(|service| Box::pin(service.get_observer_list())).await;
            },
            "get_consensus_status" => {
                self.call_web3_service(|service| Box::pin(service.get_consensus_status())).await;
            },
            "get_sync_status" => {
                self.call_web3_service(|service| Box::pin(service.get_sync_status())).await;
            },
            "get_peers" => {
                self.call_web3_service(|service| Box::pin(service.get_peers())).await;
            },
            "get_group_peers" => {
                self.call_web3_service(|service| Box::pin(service.get_group_peers())).await;
            },
            "get_node_id_list" => {
                self.call_web3_service(|service| Box::pin(service.get_node_id_list())).await;
            },
            "get_group_list" => {
                self.call_web3_service(|service| Box::pin(service.get_group_list())).await;
            },
            "get_block_by_hash" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_block_by_hash(
                            command_parts[1],
                            convert_str_to_bool(command_parts[2]),
                        )
                    )).await;
                }
            },
            "get_block_by_number" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_block_by_number(
                            command_parts[1],
                            convert_str_to_bool(command_parts[2]),
                        )
                    )).await;
                }
            },
            "get_block_header_by_hash" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_block_header_by_hash(
                            command_parts[1],
                            convert_str_to_bool(command_parts[2]),
                        )
                    )).await;
                }
            },
            "get_block_header_by_number" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_block_header_by_number(
                            command_parts[1],
                            convert_str_to_bool(command_parts[2]),
                        )
                    )).await;
                }
            },
            "get_block_hash_by_number" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_block_hash_by_number(command_parts[1])
                    )).await;
                }
            },
            "get_transaction_by_hash" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_transaction_by_hash(command_parts[1])
                    )).await;
                }
            },
            "get_transaction_by_block_hash_and_index" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_transaction_by_block_hash_and_index(
                            command_parts[1],
                            command_parts[2],
                        )
                    )).await;
                }
            },
            "get_transaction_by_block_number_and_index" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_transaction_by_block_number_and_index(
                            command_parts[1],
                            command_parts[2],
                        )
                    )).await;
                }
            },
            "get_transaction_receipt" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_transaction_receipt(command_parts[1])
                    )).await;
                }
            },
            "get_pending_transactions" => {
                self.call_web3_service(|service| Box::pin(
                    service.get_pending_transactions()
                )).await;
            },
            "get_pending_tx_size" => {
                self.call_web3_service(|service| Box::pin(
                    service.get_pending_tx_size()
                )).await;
            },
            "get_code" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_code(command_parts[1])
                    )).await;
                }
            },
            "get_total_transaction_count" => {
                self.call_web3_service(|service| Box::pin(
                    service.get_total_transaction_count()
                )).await;
            },
            "call" => {
                if valid_args_len(command_parts_length, 3) {
                    let params: Vec<String> = if command_parts_length > 3 {
                        command_parts[4..].iter().map(|&v| v.to_owned()).collect()
                    } else {
                        vec![]
                    };
                    self.call_web3_service(|service| Box::pin(
                        service.call(
                            command_parts[1],
                            command_parts[2],
                            command_parts[3],
                            &params,
                        )
                    )).await;
                }
            },
            "send_raw_transaction" => {
                if valid_args_len(command_parts_length, 3) {
                    let params: Vec<String> = if command_parts_length > 3 {
                        command_parts[4..].iter().map(|&v| v.to_owned()).collect()
                    } else {
                        vec![]
                    };
                    self.call_web3_service(|service| Box::pin(
                        service.send_raw_transaction(
                            command_parts[1],
                            command_parts[2],
                            command_parts[3],
                            &params,
                        )
                    )).await;
                }
            },
            "send_raw_transaction_and_get_proof" => {
                if valid_args_len(command_parts_length, 3) {
                    let params: Vec<String> = if command_parts_length > 3 {
                        command_parts[4..].iter().map(|&v| v.to_owned()).collect()
                    } else {
                        vec![]
                    };
                    self.call_web3_service(|service| Box::pin(
                        service.send_raw_transaction_and_get_proof(
                            command_parts[1],
                            command_parts[2],
                            command_parts[3],
                            &params,
                        )
                    )).await;
                }
            },
            "deploy" => {
                if valid_args_len(command_parts_length, 2) {
                    let params: Vec<String> = if command_parts_length > 3 {
                        command_parts[3..].iter().map(|&v| v.to_owned()).collect()
                    } else {
                        vec![]
                    };
                    self.call_web3_service(|service| Box::pin(
                        service.deploy(
                            command_parts[1],
                            command_parts[2],
                            &params,
                        )
                    )).await;
                }
            },
            "get_system_config_by_key" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_system_config_by_key(command_parts[1])
                    )).await;
                }
            },
            "get_transaction_by_hash_with_proof" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_transaction_by_hash_with_proof(
                            command_parts[1],
                        )
                    )).await;
                }
            },
            "get_transaction_receipt_by_hash_with_proof" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_transaction_receipt_by_hash_with_proof(
                            command_parts[1],
                        )
                    )).await;
                }
            },
            "generate_group" => {
                if valid_args_len(command_parts_length, 1) {
                    let params = convert_str_to_json(command_parts[1]);
                    self.call_web3_service(|service| Box::pin(service.generate_group(&params))).await;
                }
            },
            "start_group" => {
                self.call_web3_service(|service| Box::pin(service.start_group())).await;
            },
            "stop_group" => {
                self.call_web3_service(|service| Box::pin(service.stop_group())).await;
            },
            "remove_group" => {
                self.call_web3_service(|service| Box::pin(service.remove_group())).await;
            },
            "recover_group" => {
                self.call_web3_service(|service| Box::pin(service.recover_group())).await;
            },
            "query_group_status" => {
                self.call_web3_service(|service| Box::pin(
                    service.query_group_status()
                )).await;
            },
            "get_node_info" => {
                self.call_web3_service(|service| Box::pin(
                    service.get_node_info()
                )).await;
            },
            "get_batch_receipts_by_block_number_and_range" => {
                if valid_args_len(command_parts_length, 4) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_batch_receipts_by_block_number_and_range(
                            command_parts[1],
                            convert_str_to_number::<u32>(command_parts[2], 0),
                            convert_str_to_number::<i32>(command_parts[2], -1),
                            convert_str_to_bool(command_parts[4]),
                        )
                    )).await;
                }
            },
            "get_batch_receipts_by_block_hash_and_range" => {
                if valid_args_len(command_parts_length, 4) {
                    self.call_web3_service(|service| Box::pin(
                        service.get_batch_receipts_by_block_hash_and_range(
                            command_parts[1],
                            convert_str_to_number::<u32>(command_parts[2], 0),
                            convert_str_to_number::<i32>(command_parts[2], -1),
                            convert_str_to_bool(command_parts[4]),
                        )
                    )).await;
                }
            },
            command => println!("\nUnavailable command {:?}\n", command),
        }
    }
}
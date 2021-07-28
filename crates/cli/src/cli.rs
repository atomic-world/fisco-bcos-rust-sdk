use std::fmt::Debug;
use futures::future::BoxFuture;
use fisco_bcos_service::{
    create_web3_service,
    web3::{service::Service as Web3Service, service_error::ServiceError as Web3ServiceError},
};

pub(crate) struct Cli {
    web3_service: Option<Web3Service>,
}

fn valid_args_len(command_parts_length: usize, min_len: usize) -> bool {
    if command_parts_length - 1 < min_len {
        println!("\nArgument count should be {:}\n", min_len);
        false
    } else {
        true
    }
}

fn convert_str_to_bool(value: &str) -> bool {
    value.eq_ignore_ascii_case("true")
}

impl Cli {
    fn set_config(&mut self, config_path: &str) {
        match create_web3_service(config_path) {
            Ok(web3_service) => self.web3_service = Some(web3_service),
            Err(error) => println!("\n Web3 Service initialize error: {:?}\n", error),
        };
    }

    async fn call_web3_service<'a, T, F>(&self, f: F)
    where
        T: Debug,
        F: FnOnce() -> BoxFuture<'a, Result<T, Web3ServiceError>>
    {
        if self.web3_service.is_some() {
            match f().await {
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
                "get_transaction_by_block_number_and_index", "get_transaction_receipt",
                "get_pending_transactions", "get_pending_tx_size", "get_code",
                "get_total_transaction_count", "get_system_config_by_key",
                "get_transaction_by_hash_with_proof", "get_transaction_receipt_by_hash_with_proof",
                "query_group_status", "get_node_info", "get_batch_receipts_by_block_number_and_range",
                "get_batch_receipts_by_block_hash_and_range",
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
                self.call_web3_service(|| Box::pin(self.web3_service.as_ref().unwrap().get_client_version())).await;
            },
            "get_block_number" => {
                self.call_web3_service(|| Box::pin(self.web3_service.as_ref().unwrap().get_block_number())).await;
            },
            "get_pbft_view" => {
                self.call_web3_service(|| Box::pin(self.web3_service.as_ref().unwrap().get_pbft_view())).await;
            },
            "get_sealer_list" => {
                self.call_web3_service(|| Box::pin(self.web3_service.as_ref().unwrap().get_sealer_list())).await;
            },
            "get_observer_list" => {
                self.call_web3_service(|| Box::pin(self.web3_service.as_ref().unwrap().get_observer_list())).await;
            },
            "get_consensus_status" => {
                self.call_web3_service(|| Box::pin(self.web3_service.as_ref().unwrap().get_consensus_status())).await;
            },
            "get_sync_status" => {
                self.call_web3_service(|| Box::pin(self.web3_service.as_ref().unwrap().get_sync_status())).await;
            },
            "get_peers" => {
                self.call_web3_service(|| Box::pin(self.web3_service.as_ref().unwrap().get_peers())).await;
            },
            "get_group_peers" => {
                self.call_web3_service(|| Box::pin(self.web3_service.as_ref().unwrap().get_group_peers())).await;
            },
            "get_node_id_list" => {
                self.call_web3_service(|| Box::pin(self.web3_service.as_ref().unwrap().get_node_id_list())).await;
            },
            "get_group_list" => {
                self.call_web3_service(|| Box::pin(self.web3_service.as_ref().unwrap().get_group_list())).await;
            },
            "get_block_by_hash" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_block_by_hash(
                            command_parts[1],
                            convert_str_to_bool(command_parts[2]),
                        )
                    )).await;
                }
            },
            "get_block_by_number" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_block_by_number(
                            command_parts[1],
                            convert_str_to_bool(command_parts[2]),
                        )
                    )).await;
                }
            },
            "get_block_header_by_hash" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_block_header_by_hash(
                            command_parts[1],
                            convert_str_to_bool(command_parts[2]),
                        )
                    )).await;
                }
            },
            "get_block_header_by_number" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_block_header_by_number(
                            command_parts[1],
                            convert_str_to_bool(command_parts[2]),
                        )
                    )).await;
                }
            },
            "get_block_hash_by_number" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_block_hash_by_number(command_parts[1])
                    )).await;
                }
            },
            "get_transaction_by_hash" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_transaction_by_hash(command_parts[1])
                    )).await;
                }
            },
            "get_transaction_by_block_hash_and_index" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_transaction_by_block_hash_and_index(
                            command_parts[1],
                            command_parts[2],
                        )
                    )).await;
                }
            },
            "get_transaction_by_block_number_and_index" => {
                if valid_args_len(command_parts_length, 2) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_transaction_by_block_number_and_index(
                            command_parts[1],
                            command_parts[2],
                        )
                    )).await;
                }
            },
            "get_transaction_receipt" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_transaction_receipt(command_parts[1])
                    )).await;
                }
            },
            "get_pending_transactions" => {
                self.call_web3_service(|| Box::pin(
                    self.web3_service.as_ref().unwrap().get_pending_transactions()
                )).await;
            },
            "get_pending_tx_size" => {
                self.call_web3_service(|| Box::pin(
                    self.web3_service.as_ref().unwrap().get_pending_tx_size()
                )).await;
            },
            "get_code" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_code(command_parts[1])
                    )).await;
                }
            },
            "get_total_transaction_count" => {
                self.call_web3_service(|| Box::pin(
                    self.web3_service.as_ref().unwrap().get_total_transaction_count()
                )).await;
            },
            "get_system_config_by_key" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_system_config_by_key(command_parts[1])
                    )).await;
                }
            },
            "get_transaction_by_hash_with_proof" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_transaction_by_hash_with_proof(
                            command_parts[1],
                        )
                    )).await;
                }
            },
            "get_transaction_receipt_by_hash_with_proof" => {
                if valid_args_len(command_parts_length, 1) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_transaction_receipt_by_hash_with_proof(
                            command_parts[1],
                        )
                    )).await;
                }
            },
            "query_group_status" => {
                self.call_web3_service(|| Box::pin(
                    self.web3_service.as_ref().unwrap().query_group_status()
                )).await;
            },
            "get_node_info" => {
                self.call_web3_service(|| Box::pin(
                    self.web3_service.as_ref().unwrap().get_node_info()
                )).await;
            },
            "get_batch_receipts_by_block_number_and_range" => {
                if valid_args_len(command_parts_length, 4) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_batch_receipts_by_block_number_and_range(
                            command_parts[1],
                            String::from(command_parts[2]).parse::<u32>().unwrap(),
                            String::from(command_parts[3]).parse::<i32>().unwrap(),
                            convert_str_to_bool(command_parts[4]),
                        )
                    )).await;
                }
            },
            "get_batch_receipts_by_block_hash_and_range" => {
                if valid_args_len(command_parts_length, 4) {
                    self.call_web3_service(|| Box::pin(
                        self.web3_service.as_ref().unwrap().get_batch_receipts_by_block_hash_and_range(
                            command_parts[1],
                            String::from(command_parts[2]).parse::<u32>().unwrap(),
                            String::from(command_parts[3]).parse::<i32>().unwrap(),
                            convert_str_to_bool(command_parts[4]),
                        )
                    )).await;
                }
            },
            command => println!("\nUnavailable command {:?}\n", command),
        }
    }
}
use serde::Deserialize;
use std::{fs, path::{Path, PathBuf}};

#[derive(Deserialize, Clone, Debug)]
pub struct Node {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Authentication {
    pub ca_cert: String,
    pub sign_cert: String,
    pub sign_key: String,
    #[serde(default)]
    pub enc_key: String,
    #[serde(default)]
    pub enc_cert: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Contract {
    pub solc: String,
    pub source: String,
    pub output: String,
}

impl Contract {
    pub fn get_contract_path(&self, contract_name: &str) -> PathBuf {
        Path::new(&self.source).join(format!("{:}.sol", contract_name))
    }

    pub fn get_abi_path(&self, contract_name: &str) -> PathBuf {
        Path::new(&self.output).join(format!("{:}.abi", contract_name))
    }

    pub fn get_abi_bin_path(&self, contract_name: &str) -> PathBuf {
        Path::new(&self.output).join(format!("{:}.bin", contract_name))
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub service_type: String,
    pub node: Node,
    pub contract: Contract,
    pub account: String,
    #[serde(default)]
    pub authentication: Authentication,
    pub sm_crypto: bool,
    pub group_id: u32,
    pub chain_id: u32,
    pub timeout_seconds: i64,
}

impl Config {
    fn get_file_real_path(&self, base_path: &Path, file_path: &str) -> String {
        if file_path.len() > 0 {
            fs::canonicalize(base_path.join(file_path)).unwrap().display().to_string()
        } else {
            String::default()
        }
    }

    pub fn convert_paths(&mut self, base_path: &Path) {
        self.account = self.get_file_real_path(base_path, &self.account);
        self.contract = Contract {
            solc: self.get_file_real_path(base_path, &self.contract.solc),
            source: self.get_file_real_path(base_path, &self.contract.source),
            output: self.get_file_real_path(base_path, &self.contract.output),
        };
        self.authentication = Authentication {
            ca_cert: self.get_file_real_path(base_path, &self.authentication.ca_cert),
            sign_cert: self.get_file_real_path(base_path, &self.authentication.sign_cert),
            sign_key: self.get_file_real_path(base_path, &self.authentication.sign_key),
            enc_key: self.get_file_real_path(base_path, &self.authentication.enc_key),
            enc_cert: self.get_file_real_path(base_path, &self.authentication.enc_cert),
        };
    }
}
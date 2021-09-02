use thiserror::Error;
use std::fs::File;
use ethabi::{
    Contract,
    param_type::{ParamType},
    token::{StrictTokenizer, Token, Tokenizer},
};

#[derive(Error, Debug)]
pub enum ABIError {
    #[error("std::io::Error")]
    StdIOError(#[from] std::io::Error),

    #[error("ethabi::Error")]
    ETHABIError(#[from] ethabi::Error),

    #[error("hex::FromHexError")]
    FromHexError(#[from] hex::FromHexError),
}

pub struct ABI {
    contract: Contract,
}

impl ABI {
    pub fn new(abi_path: &str) -> Result<ABI, ABIError> {
        let contract = Contract::load(File::open(abi_path)?)?;
        Ok(ABI { contract })
    }

    pub fn encode_input(&self, function_name: &str, params: &Vec<String>) -> Result<Vec<u8>, ABIError> {
        let function = self.contract.function(&function_name)?;
        let params: Vec<(ParamType, &str)> = function.inputs.iter()
            .map(|param| param.kind.clone())
            .zip(params.iter().map(|v| v as &str)).collect();
        let tokens = params.iter()
            .map(|&(ref param, value)| StrictTokenizer::tokenize(param, value))
            .collect::<Result<Vec<Token>, ethabi::Error>>()?;
        Ok(Vec::from(function.encode_input(&tokens)?))
    }

    pub fn decode_output(&self, function_name: &str, value: &str) -> Result<Vec<Token>, ABIError> {
        let data = hex::decode(value.to_string().trim_start_matches("0x").as_bytes())?;
        let function = self.contract.function(&function_name)?;
        Ok(function.decode_output(&data)?)
    }
}
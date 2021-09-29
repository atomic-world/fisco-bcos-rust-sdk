use std::fs;
use thiserror::Error;
use wedpr_l_utils::traits::Hash;
use wedpr_l_crypto_hash_sm3::WedprSm3;
use ethabi::{
    Contract, Bytes, Function,
    param_type::{ParamType, Writer},
    token::Token,
    Error as ETHError,
    Result as ETHResult,
    decode as eth_decode,
    encode as eth_encode,
};

fn sm_short_signature(name: &str, params: &[ParamType]) -> Vec<u8> {
    let types = params.iter().map(Writer::write).collect::<Vec<String>>().join(",");
    let data: Vec<u8> = From::from(format!("{}({})", name, types).as_str());
    let sm3_hash = WedprSm3::default();
    sm3_hash.hash(&data)[..4].to_vec()
}

#[derive(Error, Debug)]
pub enum ABIError {
    #[error("std::io::Error")]
    StdIOError(#[from] std::io::Error),

    #[error("ethabi::Error")]
    ETHABIError(#[from] ethabi::Error),

    #[error("hex::FromHexError")]
    FromHexError(#[from] hex::FromHexError),

    #[error("abi custom error")]
    CustomError {
        message: String,
    }
}

pub struct ABI {
    contract: Contract,
    sm_crypto: bool,
}

impl ABI {
    fn encode_sm_input(&self, function: &Function, tokens: &Vec<Token>) -> ETHResult<Bytes> {
        let params: Vec<ParamType> = function.inputs.iter().map(|p| p.kind.clone()).collect();
        if !Token::types_check(tokens, &params) {
            return Err(ETHError::InvalidData);
        }
        let mut transaction_data = sm_short_signature(&function.name, &params);
        transaction_data.extend(eth_encode(tokens));
        Ok(transaction_data)
    }

    pub fn new(abi_path: &str, sm_crypto: bool) -> Result<ABI, ABIError> {
        let contract = Contract::load(fs::File::open(abi_path)?)?;
        Ok(ABI { contract, sm_crypto })
    }

    pub fn encode_constructor_input(&self, abi_bin_path: &str, tokens: &Vec<Token>) -> Result<Vec<u8>, ABIError> {
        let constructor = self.contract.constructor.as_ref().unwrap();
        Ok(hex::decode(Vec::from(constructor.encode_input(fs::read(&abi_bin_path)?, &tokens)?))?)
    }

    pub fn encode_function_input(&self, function_name: &str, tokens: &Vec<Token>) -> Result<Vec<u8>, ABIError> {
        let function = self.contract.function(&function_name)?;
        if self.sm_crypto {
            Ok(self.encode_sm_input(&function, &tokens)?)
        } else {
            Ok(function.encode_input(&tokens)?)
        }
    }

    pub fn decode_output(&self, function_name: &str, value: &str) -> Result<Option<Vec<Token>>, ABIError> {
        if value.eq("0x") {
            return Ok(None);
        }
        let function = self.contract.function(&function_name)?;
        if value.starts_with("0x08c379a0") {
            let output = &value[10..];
            let data = hex::decode(output.to_owned().as_bytes())?;
            let params: Vec<ParamType> = vec![ParamType::String];
            let tokens = eth_decode(&params, &data)?;
            return Err(ABIError::CustomError { message: tokens[0].to_string() });
        }
        let data = hex::decode(value.to_owned().trim_start_matches("0x").as_bytes())?;
        Ok(Some(function.decode_output(&data)?))
    }
}
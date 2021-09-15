use std::fs;
use thiserror::Error;
use wedpr_l_utils::traits::Hash;
use wedpr_l_crypto_hash_sm3::WedprSm3;
use ethabi::{
    Contract, Param, Bytes, Function,
    param_type::{ParamType, Writer},
    token::{StrictTokenizer, Token, Tokenizer},
    Error as ETHError,
    Result as ETHResult,
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
}

pub struct ABI {
    contract: Contract,
    sm_crypto: bool,
}

impl ABI {
    fn encode_sm_input(&self, function: &Function, tokens: &[Token]) -> ETHResult<Bytes> {
        let params: Vec<ParamType> = function.inputs.iter().map(|p| p.kind.clone()).collect();
        if !Token::types_check(tokens, &params) {
            return Err(ETHError::InvalidData);
        }
        let mut transaction_data = sm_short_signature(&function.name, &params);
        transaction_data.extend(eth_encode(tokens));
        Ok(transaction_data)
    }

    fn get_tokens_from_inputs(&self, inputs: &Vec<Param>, params: &Vec<String>) -> Result<Vec<Token>, ABIError> {
        let params: Vec<(ParamType, &str)> = inputs.iter()
            .map(|param| param.kind.clone())
            .zip(params.iter().map(|v| v as &str)).collect();
        let tokens = params.iter()
            .map(|&(ref param, value)| StrictTokenizer::tokenize(param, value))
            .collect::<Result<Vec<Token>, ethabi::Error>>()?;
        Ok(tokens)
    }

    pub fn new(abi_path: &str, sm_crypto: bool) -> Result<ABI, ABIError> {
        let contract = Contract::load(fs::File::open(abi_path)?)?;
        Ok(ABI { contract, sm_crypto })
    }

    pub fn encode_constructor_input(&self, abi_bin_path: &str, params: &Vec<String>) -> Result<Vec<u8>, ABIError> {
        let constructor = self.contract.constructor.as_ref().unwrap();
        let tokens = self.get_tokens_from_inputs(&constructor.inputs, params)?;
        Ok(hex::decode(Vec::from(constructor.encode_input(fs::read(&abi_bin_path)?, &tokens)?))?)
    }

    pub fn encode_input(&self, function_name: &str, params: &Vec<String>) -> Result<Vec<u8>, ABIError> {
        let function = self.contract.function(&function_name)?;
        let tokens = self.get_tokens_from_inputs(&function.inputs, params)?;
        if self.sm_crypto {
            Ok(self.encode_sm_input(&function, &tokens)?)
        } else {
            Ok(function.encode_input(&tokens)?)
        }
    }

    pub fn decode_output(&self, function_name: &str, value: &str) -> Result<Vec<Token>, ABIError> {
        let data = hex::decode(value.to_owned().trim_start_matches("0x").as_bytes())?;
        let function = self.contract.function(&function_name)?;
        Ok(function.decode_output(&data)?)
    }
}
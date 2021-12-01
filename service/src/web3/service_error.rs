use thiserror::Error;
use crate::abi::ABIError;
use crate::account::AccountError;
use crate::transaction::TransactionError;
use crate::tassl::TASSLError;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("std::io::Error")]
    StdIOError(#[from] std::io::Error),

    #[error("hyper::Error")]
    HyperError(#[from] hyper::Error),

    #[error("hyper::http::Error")]
    HyperHttpError(#[from] hyper::http::Error),

    #[error("serde_json::Error")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("std::string::FromUtf8Error")]
    StringFromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("std::array::TryFromSliceError")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),

    #[error("tassl error")]
    TASSLError(#[from] TASSLError),

    #[error("abi error")]
    ABIError(#[from] ABIError),

    #[error("account error")]
    AccountError(#[from] AccountError),

    #[error("transaction error")]
    TransactionError(#[from] TransactionError),

    #[error("fisco bcos custom error")]
    CustomError {
        message: String,
    },

    #[error("fisco bcos response error")]
    FiscoBcosError {
        code: i32,
        message: String,
    }
}
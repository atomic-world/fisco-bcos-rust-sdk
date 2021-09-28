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

    #[error("service tassl error")]
    ServiceTASSLError(#[from] TASSLError),

    #[error("service abi error")]
    ServiceABIError(#[from] ABIError),

    #[error("service account error")]
    ServiceAccountError(#[from] AccountError),

    #[error("service transaction error")]
    ServiceTransactionError(#[from] TransactionError),

    #[error("fisco bcos service error")]
    FiscoBcosError {
        code: i64,
        message: String,
    }
}
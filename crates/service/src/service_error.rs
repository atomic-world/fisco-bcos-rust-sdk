use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("hyper::Error")]
    HyperError(#[from] hyper::Error),

    #[error("hyper::http::Error")]
    HyperHttpError(#[from] hyper::http::Error),

    #[error("serde_json::Error")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("fisco bcos service error")]
    FiscoBcosError {
        code: i64,
        message: String,
    }
}
use cosmwasm_std::StdError;
#[cfg(feature = "backtraces")]
use std::backtrace::Backtrace;

use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("not implemented")]
    NotImplemented,

    #[error("unknown request")]
    UnknownRequest,
}

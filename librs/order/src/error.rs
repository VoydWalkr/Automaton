use cosmrs::ErrorReport;
use k256::ecdsa;
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderError {
  #[error("{0}")]
  Std(#[from] StdError),
  
  #[error("{0}")]
  SignError(#[from] ErrorReport),
  
  #[error("{0}")]
  ECDSAError(#[from] ecdsa::Error),
  
  #[error("Unsigned")]
  Unsigned,
}

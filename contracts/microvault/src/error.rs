use cosmwasm_std::StdError;
use k256::ecdsa;
use thiserror::Error;
use voydwalkr_vanguard_order::error::OrderError;

#[derive(Error, Debug)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("Unauthorized")]
  Unauthorized,
  
  #[error("Illegal")]
  Illegal,
  
  #[error("VerificationFailure")]
  VerificationFailure(#[from] VerificationError),
  
  #[error("NotImplemented")]
  NotImplemented,
}

#[derive(Error, Debug)]
pub enum VerificationError {
  #[error("{0}")]
  ECDSA(#[from] ecdsa::Error),
  
  #[error("{0}")]
  Order(#[from] OrderError),
  
  #[error("Bech32")]
  Bech32(#[from] bech32::Error),
}

impl From<ecdsa::Error> for ContractError {
  fn from(error: ecdsa::Error) -> Self {
    VerificationError::from(error).into()
  }
}

impl From<OrderError> for ContractError {
  fn from(error: OrderError) -> Self {
    VerificationError::from(error).into()
  }
}

impl From<bech32::Error> for ContractError {
  fn from(error: bech32::Error) -> Self {
    VerificationError::from(error).into()
  }
}

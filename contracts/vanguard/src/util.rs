use cosmwasm_std::{Deps, MessageInfo};
use voydwalkr_util::{Rate, SerRate};

use crate::error::ContractError;
use crate::state::STATE;

pub fn normalize_serrate(rate: SerRate) -> SerRate {
  SerRate::from(Rate::from(rate))
}

pub fn assert_owner(deps: Deps, info: MessageInfo) -> Result<(), ContractError> {
  let state = STATE.load(deps.storage)?;
  if state.owner != info.sender {
    Err(ContractError::Unauthorized {})
  }
  else {
    Ok(())
  }
}

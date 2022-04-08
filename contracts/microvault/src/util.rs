use cosmwasm_std::{Deps, MessageInfo};

use crate::error::ContractError;
use crate::state::STATE;

pub fn assert_owner(deps: Deps, info: MessageInfo) -> Result<(), ContractError> {
  let state = STATE.load(deps.storage)?;
  if state.owner != info.sender {
    Err(ContractError::Unauthorized {})
  }
  else {
    Ok(())
  }
}

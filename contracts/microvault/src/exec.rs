use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::primitives::ContractResult;
use crate::state::{STATE, State};
use crate::util::{assert_owner};
use crate::evt;

pub fn transfer_ownership(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  owner: Addr,
) -> ContractResult {
  assert_owner(deps.as_ref(), info)?;
  STATE.update(deps.storage, | mut state | -> Result<State, ContractError> {
    state.owner = owner.clone();
    Ok(state)
  })?;
  Ok(Response::new().add_event(evt::transfer_ownership(owner)))
}

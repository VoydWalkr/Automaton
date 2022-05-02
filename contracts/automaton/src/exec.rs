use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};
use crate::error::ContractError;
use crate::state::{State, STATE};
use crate::util::assert_owner;
use crate::evt;

pub fn transfer_ownership(deps: DepsMut, _env: Env, info: MessageInfo, new_owner: Addr) -> Result<Response, ContractError> {
  assert_owner(deps.as_ref(), info)?;
  STATE.update(deps.storage, | mut state | -> Result<State, ContractError> {
    state.owner = new_owner.clone();
    Ok(state)
  })?;
  Ok(Response::new()
    .add_event(evt::transfer_ownership(new_owner))
  )
}

use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};

use voydwalkr_automaton_order::Order;
use crate::error::{ContractError};
use crate::primitives::ContractResult;
use crate::state::{STATE, State};
use crate::util::{assert_owner};
use crate::evt;
use crate::qry;

pub fn transfer_ownership(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  owner: Addr,
) -> ContractResult {
  assert_owner(deps.as_ref(), &info)?;
  
  let canonical = deps.api.addr_canonicalize(info.sender.as_ref())?;
  
  STATE.update(deps.storage, | mut state | -> Result<State, ContractError> {
    state.owner = canonical;
    Ok(state)
  })?;
  Ok(Response::new().add_event(evt::transfer_ownership(owner)))
}

pub fn execute_order(
  deps: DepsMut,
  env: Env,
  _info: MessageInfo,
  order: Order,
) -> ContractResult {
  qry::verify_order(deps.as_ref(), env, order)?;
  Err(ContractError::NotImplemented)
}

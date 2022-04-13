use cosmwasm_std::{Deps, Env, StdResult};
use voydwalkr_vanguard_order::Order;

use crate::error::ContractError;
use crate::msg::{QueryOwnerResponse};
use crate::state::STATE;
use crate::util::addr_from_verifykey;

pub fn owner(deps: Deps, _env: Env) -> StdResult<QueryOwnerResponse> {
  let state = STATE.load(deps.storage)?;
  Ok(QueryOwnerResponse {
    owner: deps.api.addr_humanize(&state.owner)?,
  })
}

pub fn verify_order(deps: Deps, _env: Env, order: Order) -> Result<(), ContractError> {
  let state = STATE.load(deps.storage)?;
  let recovered = order.recover_verify_key()?;
  
  let addr = addr_from_verifykey(&recovered)?;
  let canonical = deps.api.addr_canonicalize(addr.as_ref())?;
  
  if canonical != state.owner {
    return Err(ContractError::Unauthorized);
  }
  
  order.verify(&recovered)?;
  Ok(())
}

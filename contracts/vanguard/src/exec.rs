use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};

use voydwalkr_util::SerRate;
use crate::error::ContractError;
use crate::evt;
use crate::primitives::ContractResult;
use crate::state::{STATE, State};
use crate::util::{assert_owner, normalize_serrate};

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

pub fn adjust_tax_rate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  tax_rate: SerRate,
) -> ContractResult {
  assert_owner(deps.as_ref(), info)?;
  let norm_tax_rate = normalize_serrate(tax_rate);
  STATE.update(deps.storage, | mut state | -> Result<State, ContractError> {
    state.tax_rate = norm_tax_rate;
    Ok(state)
  })?;
  Ok(Response::new().add_event(evt::adjust_taxrate(norm_tax_rate)))
}

pub fn list_coin(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  coins: Vec<String>,
) -> ContractResult {
  assert_owner(deps.as_ref(), info)?;
  STATE.update(deps.storage, | mut state | -> Result<State, ContractError> {
    state.coin_list.append(&mut coins.clone());
    Ok(state)
  })?;
  Ok(Response::new()
    .add_event(evt::list_coin(coins))
  )
}

pub fn list_token(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  tokens: Vec<Addr>,
) -> ContractResult {
  assert_owner(deps.as_ref(), info)?;
  STATE.update(deps.storage, | mut state | -> Result<State, ContractError> {
    state.token_list.append(&mut tokens.clone());
    Ok(state)
  })?;
  Ok(Response::new()
    .add_event(evt::list_token(tokens))
  )
}

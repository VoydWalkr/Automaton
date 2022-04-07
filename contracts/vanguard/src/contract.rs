#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::primitives::ContractResult;
use crate::state::{State, STATE};
use crate::util::{normalize_serrate};

use crate::evt;
use crate::exec;
use crate::qry;

// version info for migration info
const CONTRACT_NAME: &str = "voydwalkr.xyz:vanguard";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  let tax_rate = normalize_serrate(msg.tax_rate);
  
  let state = State {
    owner: info.sender.clone(),
    tax_rate: tax_rate,
    coin_list: vec![],
    token_list: vec![],
  };
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  STATE.save(deps.storage, &state)?;

  Ok(Response::new()
    .add_attribute("method", "instantiate")
    .add_event(evt::transfer_ownership(info.sender))
    .add_event(evt::adjust_taxrate(tax_rate))
  )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
  _deps: DepsMut,
  _env: Env,
  _msg: MigrateMsg,
) -> StdResult<Response> {
  Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> ContractResult {
  let response = match msg {
    ExecuteMsg::TransferOwnership { new_owner } => exec::transfer_ownership(deps, env, info, new_owner)?,
    ExecuteMsg::AdjustTaxRate { new_tax_rate } => exec::adjust_tax_rate(deps, env, info, new_tax_rate)?,
    ExecuteMsg::ListCoin { coins } => exec::list_coin(deps, env, info, coins)?,
    ExecuteMsg::ListToken { tokens } => exec::list_token(deps, env, info, tokens)?,
  };
  Ok(response
    .add_attribute("method", "execute")
  )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    QueryMsg::GetConfig {} => to_binary(&qry::config(deps, env)?),
    QueryMsg::GetAssetList {} => to_binary(&qry::asset_list(deps, env)?),
  }
}

#[cfg(test)]
mod tests {
  // use super::*;
  // use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
  // use cosmwasm_std::{coins, from_binary};
  
  
}

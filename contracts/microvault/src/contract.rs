#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, BoolResponse};
use crate::primitives::ContractResult;
use crate::state::{State, STATE};

use crate::evt;
use crate::exec;
use crate::qry;

// version info for migration info
const CONTRACT_NAME: &str = "voydwalkr.xyz:vanguard-microvault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  let state = State {
    owner: deps.api.addr_canonicalize(info.sender.as_ref())?,
  };
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  STATE.save(deps.storage, &state)?;
  
  Ok(Response::new()
    .add_attribute("method", "instantiate")
    .add_event(evt::transfer_ownership(info.sender))
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
    ExecuteMsg::ExecuteOrder { order } => exec::execute_order(deps, env, info, order)?,
  };
  Ok(response
    .add_attribute("method", "execute")
  )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    QueryMsg::GetOwner {} => to_binary(&qry::owner(deps, env)?),
    QueryMsg::VerifyOrder { order } => {
      match qry::verify_order(deps, env, order) {
        Err(_) => to_binary(&BoolResponse::new(false)),
        Ok(()) => to_binary(&BoolResponse::new(true)),
      }
    },
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::state::{STATE, State};
  use bip32::{Mnemonic, XPrv};
  use cosmwasm_std::{Addr, Api, BankMsg, CosmosMsg, Empty};
  use cosmwasm_std::testing::{mock_dependencies, mock_env};
  // use cosmwasm_std::{coins, from_binary};
  use k256::ecdsa::{SigningKey};
  use voydwalkr_vanguard_order::Order;
  
  const TEST1_MNEM: &str = "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius";
  const TEST2_MNEM: &str = "quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty";
  
  #[test]
  fn generate_address_works() {
    assert_eq!(addr_from_prv(&getprv(TEST1_MNEM, 0, 0)), "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v");
    assert_eq!(addr_from_prv(&getprv(TEST2_MNEM, 0, 0)), "terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp");
  }
  
  #[test]
  fn verify_order_works() {
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let prv1 = getprv(TEST1_MNEM, 0, 0);
    let prv2 = getprv(TEST2_MNEM, 0, 0);
    let addr1 = addr_from_prv(&prv1);
    let addr2 = addr_from_prv(&prv2);
    let canonical1 = deps.api.addr_canonicalize(addr1.as_ref()).unwrap();
    
    STATE.save(&mut deps.storage, &State {
      owner: canonical1,
    }).unwrap();
    
    let order = Order::<Empty>::create_and_sign(&SigningKey::from(prv1), vec![
      CosmosMsg::Bank(BankMsg::Send {
        amount: vec![],
        to_address: addr2.to_string(),
      })
    ]).unwrap();
    
    qry::verify_order(deps.as_ref(), env, order).unwrap();
  }
  
  #[test]
  fn verify_order_fails() {
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let prv1 = getprv(TEST1_MNEM, 0, 0);
    let prv2 = getprv(TEST2_MNEM, 0, 0);
    let addr1 = addr_from_prv(&prv1);
    let addr2 = addr_from_prv(&prv2);
    let canonical1 = deps.api.addr_canonicalize(addr1.as_ref()).unwrap();
    
    STATE.save(&mut deps.storage, &State {
      owner: canonical1,
    }).unwrap();
    
    let order = Order::<Empty>::create_and_sign(&SigningKey::from(prv2), vec![
      CosmosMsg::Bank(BankMsg::Send {
        amount: vec![],
        to_address: addr2.to_string(),
      })
    ]).unwrap();
    
    qry::verify_order(deps.as_ref(), env, order).unwrap_err();
  }
  
  fn getprv(mnemonic: &str, account: u64, index: u64) -> XPrv {
    let mnem = Mnemonic::new(mnemonic, bip32::Language::English).unwrap();
    let seed = mnem.to_seed("");
    let path = format!("m/44'/330'/{}'/0/{}", account, index);
    XPrv::derive_from_path(&seed, &path.parse().unwrap()).unwrap()
  }
  
  fn addr_from_prv(xprv: &XPrv) -> Addr {
    crate::util::addr_from_verifykey(xprv.public_key().public_key()).unwrap()
  }
}

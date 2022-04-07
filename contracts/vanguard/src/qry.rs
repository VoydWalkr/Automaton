use cosmwasm_std::{Deps, Env, StdResult};

use crate::msg::{QueryAssetListResponse, QueryConfigResponse};
use crate::state::STATE;

pub fn config(deps: Deps, _env: Env) -> StdResult<QueryConfigResponse> {
  let state = STATE.load(deps.storage)?;
  Ok(QueryConfigResponse {
    owner: state.owner,
    tax_rate: state.tax_rate,
  })
}

pub fn asset_list(deps: Deps, _env: Env) -> StdResult<QueryAssetListResponse> {
  let store = STATE.load(deps.storage)?;
  Ok(QueryAssetListResponse {
    coins: store.coin_list.clone(),
    tokens: store.token_list.clone(),
  })
}

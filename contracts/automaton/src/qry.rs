use cosmwasm_std::{Deps, Env, StdResult};

use crate::msg::{QueryOwnerResponse};
use crate::state::{STATE};

pub fn owner(deps: Deps, _env: Env) -> StdResult<QueryOwnerResponse> {
  let state = STATE.load(deps.storage)?;
  Ok(QueryOwnerResponse {
    owner: state.owner,
  })
}

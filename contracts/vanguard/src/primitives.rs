use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Response};
use voydwalkr_util::{SerRate};

use crate::error::ContractError;

/** (BaseToken, QuoteToken, TradeRate) */
pub type OrderKey = (Addr, Addr, SerRate);

pub type ContractResult = Result<Response, ContractError>;

// Base token, quote token, order owner, and rate are keys in cw_storage_plus::Map
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderState {
  pub tax_rate: SerRate,
  pub size: u128,
}

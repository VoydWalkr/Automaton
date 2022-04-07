use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use voydwalkr_util::SerRate;

use crate::primitives::{
  OrderKey,
  OrderState,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
  pub owner: Addr,
  pub tax_rate: SerRate,
  pub coin_list: Vec<String>,
  pub token_list: Vec<Addr>,
}

pub const STATE: Item<State> = Item::new("state");
pub const ORDERS: Map<OrderKey, OrderState> = Map::new("orders");

use cosmwasm_std::{Addr, Event};
use voydwalkr_util::SerRate;

pub const EVT_LIST_COIN: &str = "list_coin";
pub const EVT_LIST_TOKEN: &str = "list_token";
pub const EVT_TAXRATE: &str = "adjust_tax_rate";
pub const EVT_TRANSFER_OWNERSHIP: &str = "transfer_ownership";

pub fn transfer_ownership(new_owner: Addr) -> Event {
  Event::new(EVT_TRANSFER_OWNERSHIP)
    .add_attribute("owner", new_owner)
}

pub fn adjust_taxrate(new_tax_rate: SerRate) -> Event {
  Event::new(EVT_TAXRATE)
    .add_attribute("tax_rate", new_tax_rate)
}

pub fn list_coin(coins: Vec<String>) -> Event {
  Event::new(EVT_LIST_COIN)
    .add_attribute("coins", coins.join(", "))
}

pub fn list_token(tokens: Vec<Addr>) -> Event {
  let addrs: Vec<_> = tokens
    .iter()
    .map(|item| String::from(item))
    .collect();
  
  Event::new(EVT_LIST_TOKEN)
    .add_attribute("tokens", addrs.join(", "))
}

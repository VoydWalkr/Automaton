use cosmwasm_std::{Addr, Event};

pub fn transfer_ownership(owner: Addr) -> Event {
  Event::new("transfer_ownership")
    .add_attribute("owner", owner)
}

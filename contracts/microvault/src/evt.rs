use cosmwasm_std::{Addr, Event};

pub const EVT_TRANSFER_OWNERSHIP: &str = "transfer_ownership";

pub fn transfer_ownership(new_owner: Addr) -> Event {
  Event::new(EVT_TRANSFER_OWNERSHIP)
    .add_attribute("owner", new_owner)
}

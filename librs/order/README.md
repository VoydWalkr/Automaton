# VoydWalkr's Automaton Order
*Automaton* builds upon *Order*s which represent a series of [`CosmosMsg`s](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/enum.CosmosMsg.html) signed by a [MicroVault](https://github.com/voydwalkr/automaton/tree/master/contracts/microvault)'s owner. This crate provides the `Order` structure, as well as a helper to create & sign `Order`s using [cosmrs](https://github.com/cosmos/cosmos-rust/tree/main/cosmrs).

**Definition**
```rust
struct Order<T = Empty>
where T: // man I wish we had trait aliases already...
{
  signature: Vec<u8>,
  msgs: Vec<CosmosMsg<T>>,
}

impl Order {
  fn create<T>(msgs: Vec<CosmosMsg<T>>) -> Order<T>;
  
  fn createAndSign<T>(
    signer: cosmrs::Wallet,
    msgs: Vec<CosmosMsg<T>>,
  ) -> Order<T>;
  
  fn is_valid(self: &Self, signer: Addr) -> bool;
}
```

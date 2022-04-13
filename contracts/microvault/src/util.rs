use bech32::{ToBase32, Variant::Bech32};
use cosmwasm_std::{Addr, Deps, MessageInfo};
use k256::ecdsa::{VerifyingKey};
use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

use crate::error::ContractError;
use crate::state::STATE;

pub fn assert_owner(deps: Deps, info: &MessageInfo) -> Result<(), ContractError> {
  let state = STATE.load(deps.storage)?;
  let canonical = deps.api.addr_canonicalize(info.sender.as_ref())?;
  if state.owner != canonical {
    Err(ContractError::Unauthorized {})
  }
  else {
    Ok(())
  }
}

pub fn addr_from_verifykey(key: &VerifyingKey) -> Result<Addr, ContractError> {
  let mut sha_hasher = Sha256::new();
  sha_hasher.update(key.to_bytes());
  let sha_hash = sha_hasher.finalize();
  
  let mut rip_hasher = Ripemd160::new();
  rip_hasher.update(sha_hash);
  let rip_hash = rip_hasher.finalize();
  
  Ok(Addr::unchecked(
    bech32::encode(
      "terra",
      rip_hash.to_base32(),
      Bech32
    )?
  ))
}

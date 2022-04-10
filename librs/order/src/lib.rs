pub mod error;

use cosmwasm_std::{CosmosMsg, Empty, to_binary};
use k256::ecdsa::{Signature, signature::{Signer, Verifier}};
use schemars::JsonSchema;
use serde::{Serialize};

use crate::error::OrderError;

pub struct Order<T = Empty>
where T: Serialize + Clone + std::fmt::Debug + PartialEq + JsonSchema
{
  signature: Option<Signature>,
  msgs: Vec<CosmosMsg<T>>
}

#[allow(dead_code)]
impl<T> Order<T>
where T: Serialize + Clone + std::fmt::Debug + PartialEq + JsonSchema
{
  fn create(msgs: Vec<CosmosMsg<T>>) -> Order<T> {
    Order {
      signature: None,
      msgs: msgs,
    }
  }
  
  fn create_and_sign<S: Signer<Signature>>(signer: &S, msgs: Vec<CosmosMsg<T>>) -> Result<Order<T>, OrderError>
  {
    Order::create(msgs).sign(signer)
  }
  
  fn sign<S: Signer<Signature>>(&self, signer: &S) -> Result<Self, OrderError>
  {
    Ok(Order {
      signature: Some(signer.sign(to_binary(&self.msgs)?.as_slice())),
      msgs: self.msgs.clone(),
    })
  }
  
  fn verify<V: Verifier<Signature>>(&self, verifier: &V) -> Result<(), OrderError>
  {
    match self.signature {
      None => Err(OrderError::Unsigned),
      Some(signature) => Ok(
        verifier.verify(
          to_binary(&self.msgs)?.as_slice(),
          &signature,
        )?
      ),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cosmwasm_std::BankMsg;
  use k256::ecdsa::{SigningKey, VerifyingKey};
  use rand_core::OsRng;
  
  #[test]
  fn sign_verify() -> Result<(), OrderError> {
    let key = SigningKey::random(OsRng);
    let order = Order::<Empty>::create_and_sign(
      &key,
      vec![
        CosmosMsg::Bank(BankMsg::Send {
          amount: vec![],
          to_address: "foo".to_string(),
        })
      ],
    )?;
    
    order.verify(&VerifyingKey::from(key))?;
    
    Ok(())
  }
}

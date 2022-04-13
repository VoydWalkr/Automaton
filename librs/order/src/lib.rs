pub mod error;

use cosmwasm_std::{CosmosMsg, Empty, to_binary};
use k256::ecdsa::{
  recoverable::Signature,
  signature::{Signature as Sig, Signer, Verifier},
  VerifyingKey,
};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::error::OrderError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Order<T = Empty>
where T: Serialize + Clone + std::fmt::Debug + PartialEq + JsonSchema
{
  signature: Option<String>,
  msgs: Vec<CosmosMsg<T>>
}

#[allow(dead_code)]
impl<T> Order<T>
where T: Serialize + Deserialize<'static> + Clone + std::fmt::Debug + PartialEq + JsonSchema
{
  pub fn create(msgs: Vec<CosmosMsg<T>>) -> Order<T> {
    Order {
      signature: None,
      msgs: msgs,
    }
  }
  
  pub fn create_and_sign<S: Signer<Signature>>(signer: &S, msgs: Vec<CosmosMsg<T>>) -> Result<Order<T>, OrderError>
  {
    Order::create(msgs).sign(signer)
  }
  
  pub fn sign<S: Signer<Signature>>(&self, signer: &S) -> Result<Self, OrderError>
  {
    let sig = signer.sign(self.get_message_bytes()?.as_slice());
    let data = base64::encode(sig.as_bytes());
    
    Ok(Order {
      signature: Some(data),
      msgs: self.msgs.clone(),
    })
  }
  
  pub fn verify<V: Verifier<Signature>>(&self, verifier: &V) -> Result<(), OrderError>
  {
    match &self.signature {
      None => Err(OrderError::Unsigned),
      Some(signature) => Ok(
        verifier.verify(
          self.get_message_bytes()?.as_slice(),
          &Signature::from_bytes(
            base64::decode(signature)?.as_slice()
          )?,
        )?
      ),
    }
  }
  
  pub fn recover_verify_key(&self) -> Result<VerifyingKey, OrderError> {
    match &self.signature {
      Some(sig) => Ok(
        Signature::from_bytes(base64::decode(sig)?.as_slice())
          ?.recover_verify_key(self.get_message_bytes()?.as_slice())?
      ),
      None => Err(OrderError::Unsigned),
    }
  }
  
  fn get_message_bytes(&self) -> Result<Vec<u8>, OrderError> {
    Ok(to_binary(&self.msgs)?.to_vec())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cosmwasm_std::BankMsg;
  use k256::ecdsa::{SigningKey, VerifyingKey};
  use rand_core::OsRng;
  use serde_json::json;
  
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
  
  #[test]
  fn serializable() -> Result<(), OrderError> {
    let key = SigningKey::random(OsRng);
    let order = Order::<Empty>::create_and_sign(
      &key,
      vec![
        CosmosMsg::Bank(BankMsg::Send {
          amount: vec![],
          to_address: "bar".to_string(),
        })
      ],
    )?;
    
    let value = json!(order);
    let r_serialized = serde_json::to_string_pretty(&value);
    let serialized = match r_serialized {
      Err(err) => panic!("serialization failed; reason: {}", err),
      Ok(s) => s,
    };
    
    println!("{}", serialized);
    let r_deserialized = serde_json::from_str::<Order>(serialized.as_ref());
    let deserialized = match r_deserialized {
      Err(err) => panic!("deserialization failed; reason: {}", err),
      Ok(d) => d,
    };
    
    assert_eq!(deserialized, order);
    Ok(())
  }
  
  #[test]
  fn recover_public_key() {
    let signer = SigningKey::random(OsRng);
    let verifier = signer.verifying_key();
    let order = Order::<Empty>::create_and_sign(&signer, vec![
      CosmosMsg::Bank(BankMsg::Send {
        to_address: "foo".to_string(),
        amount: vec![],
      })
    ]).unwrap();
    
    let recovered = order.recover_verify_key().unwrap();
    assert_eq!(verifier, recovered);
  }
}

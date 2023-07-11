use crate::{Client};
use std::io::Read;
use std::fmt::Error;
use std::time::Duration;
use crate::pkg::CustomError;

const TIMEOUT: u64 = 600;

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
  pub id:       String,
  pub name:     String,
  pub node:     Option<String>,
  pub LockDelay: String,
  pub Behavior:  String,
  pub TTL:      String,
  pub node_checks:  Vec<String>,
  service_checks: Option<String>,
  create_index:   u32,
  modify_index:   u32,
  // timeout:        u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SessionSet {
  ID:   String,
}

impl Session {

  pub fn new() -> Self {
    Session {
      id: "".to_string(),
      name: "".to_string(),
      node: None,
      LockDelay: "".to_string(),
      Behavior: "".to_string(),
      TTL: "".to_string(),
      node_checks: vec![],
      service_checks: None,
      create_index: 0,
      modify_index: 0,
      // timeout:      600,
    }
  }

  pub fn set(&self, c: &Client, s: &Session) -> String {
    "".to_string()
  }

  pub fn renew(&self, c: &Client, sid: &str) -> Result<(), CustomError> {
    Err(CustomError("".to_string()))
  }

  pub fn delete(&self, c: &Client, sid: &str) -> String {
    "".to_string()
  }
}





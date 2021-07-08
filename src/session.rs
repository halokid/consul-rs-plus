use crate::{Client};
use std::io::Read;
use std::fmt::Error;
use crate::pkg::CustomError;

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
      modify_index: 0
    }
  }

  pub fn set(&self, c: &Client, s: &Session) -> String {
    let url = format!("http://{}:{}/v1/session/create", c.host, c.port);
    let payload = serde_json::to_string(s).unwrap();
    c.debug_print(format!("set session payload ------ {}", payload).as_str());
    let mut rsp = reqwest::Client::new()
      .put(&url)
      .body(payload)
      .send()
      .map_err( |e| e.to_string() ).unwrap();
    let mut body = String::new();
    // rsp.read_to_string(&mut body).map_err( |e| e.to_string());
    // c.debug_print(format!("session set: {:?}", body).as_str());
    let session_set: SessionSet = rsp.json().unwrap();
    c.debug_print(format!("session set: {:?}", session_set).as_str());
    session_set.ID
  }

  pub fn renew(&self, c: &Client, sid: &str) -> Result<(), CustomError> {
    let url = format!("http://{}:{}/v1/session/renew/{}", c.host, c.port, sid);
    let mut rsp = reqwest::Client::new()
      .put(&url)
      .send()
      .map_err( |e| e.to_string() ).unwrap();
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err( |e| e.to_string());
    c.debug_print(format!("session renew: {:?}", body).as_str(), );
    if rsp.status().is_success() {
      Ok(())
    } else {
      Err(CustomError(format!("renew session err: {}", sid)))
    }
  }
}





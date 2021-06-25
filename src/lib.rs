#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate reqwest;
extern crate serde_json;

pub mod kv;
mod config;
pub mod session;

use self::kv::*;
use std::io::Read;

pub struct Client {
  host: String,
  port: u16,

  kv:   KVPair,
  session: session::Session,
}

impl Client {
  pub fn new<S: Into<String>>(host: S, port: u16) -> Client {
    Client {
      host: host.into(),
      port: port,
      kv: KVPair::new(),
      session: session::Session::new(),
    }
  }

  pub fn kv_get<S: Into<String>>(&self, key: S) -> Result<Vec<KVPair>, String> {
    self.kv.get(self, key)
  }

  pub fn kv_set<S: Into<String>>(&self, key: S, v: S) -> Result<bool, String> {
    self.kv.set(self, key, v)
  }

  pub fn kv_delete<S: Into<String>>(&self, key: S) -> Result<bool, String> {
    self.kv.delete(self, key)
  }

  pub fn session_set(&self, lock_delay: String, name: String, node: String,
                     behavior: String, ttl: String) -> String {
    let mut s = session::Session::new();
    s.lock_delay = "15s".to_string();
    s.name = "my-test-session".to_string();
    s.node = node;
    s.behavior = "release".to_string();
    s.ttl = "10m0s".to_string();
    self.session.set(self, &s)
  }
}

#[cfg(test)]
mod tests {
  use crate::Client;
  use base64::Config;
  use crate::config;

  #[test]
  fn test_kv_get() {
    let host = config::CONFIG["consul_addr"];
    let client = Client::new(host, 8500);
    let my_keys = client.kv_get("my-key").unwrap();
    for k in my_keys {
      println!("k: {:?}", k);
    }
  }

  #[test]
  fn test_session_set() {
    let host = config::CONFIG["consul_addr"];
    let client = Client::new(host, 8500);
    let se = client.session_set();
  }
}





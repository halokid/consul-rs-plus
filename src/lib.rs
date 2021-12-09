#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate reqwest;
extern crate serde_json;

pub mod kv;
mod config;
pub mod session;
pub mod pkg;

use self::kv::*;
use std::io::Read;
use crate::pkg::CustomError;
use base64::{decode};

// todo: global use varible here
// pub const PKGX: pkg::Pkg = pkg::Pkg::new(true);

pub struct Client {
  pub debug: bool,

  host: String,
  port: u16,

  kv:   KVPair,
  session: session::Session,
}

impl Client {
  pub fn new<S: Into<String>>(host: S, port: u16) -> Client {
    Client {
      debug: false,
      host: host.into(),
      port: port,
      kv: KVPair::new(),
      session: session::Session::new(),
    }
  }

  pub fn debug_print(&self, s: &str) {
    if self.debug {
      println!("[DEBUG] --- {:?}", s);
    }
  }

  pub fn kv_get<S: Into<String>>(&self, key: S) -> String {
    let keyx = key.into();
    let keyxx = keyx.clone();
    let res = self.kv.get(self, keyx);
    match res {
      Ok(kvs) => {
        let kv = kvs.get(0).unwrap();
        let val = &kv.Value;
        let val_de = decode(val).unwrap();
        let val_de_str = String::from_utf8(val_de).unwrap();
        val_de_str
      }
      Err(err) => {
        log::error!("key {} not exists, err: {}", keyxx, err);
        // err
        "keyNoExists_or_valIsNull".to_string()
      }
    }
  }

  pub fn kv_get_folder<S: Into<String>>(&self, folder: S) -> Result<Vec<String>,
    String> {
    let folder_keys = self.kv.get_folder_keys(self, folder).unwrap();
    serde_json::from_str::<Vec<String>>(&folder_keys).map_err(|e| e.to_string())
  }

  pub fn kv_set<S: Into<String>>(&self, key: S, v: S) -> Result<bool, String> {
    self.kv.set(self, key, v)
  }

  pub fn kv_set_with_session<S: Into<String>>(&self, key: S, v: S, session: S)
                          -> Result<bool, String> {
    self.kv.set_with_session(self, key, v, session)
  }

  pub fn kv_delete<S: Into<String>>(&self, key: S) -> Result<bool, String> {
    self.kv.delete(self, key)
  }

  pub fn kv_delete_both_session<S: Into<String>>(&self, key: S) -> Result<bool, String> {
    self.kv.delete_both_session(self, key)
  }

  /*
  pub async fn kv_folder_watch<S: Into<String>>(&self, folder: S) -> Result<bool, String> {
    let res = self.kv.watch_tree(self, folder).await;

    Ok(true)
  }
   */

  pub fn session_set(&self, lock_delay: String, name: String, node: String,
                     behavior: String, ttl: String) -> String {
    let mut s = session::Session::new();
    s.LockDelay = "0.001s".to_string();
    if name != "".to_string() {
      s.name = name;
    }

    if node != "".to_string() {
      let nodex = Some(node);
      s.node = nodex;
    }
    s.Behavior = behavior;
    s.TTL = ttl;
    self.debug_print(format!("lib session set: {:?}", s).as_str(), );
    self.session.set(self, &s)
  }

  pub fn session_renew(&self, sid: &str) -> Result<(), CustomError> {
    self.session.renew(self, sid)
  }

  pub fn session_delete(&self, sid: &str) -> String {
    self.session.delete(self, sid)
  }

}

#[cfg(test)]
mod tests {
  use crate::Client;
  use base64::Config;
  use tokio::sync::mpsc;
  use crate::config;
  use crate::pkg::CustomError;
  use crate::kv::KVPair;

  /*
  // #[test]
  #[tokio::test]
  async fn test_kv_folder_watch() {
    env_logger::init();
    for i in 0..3 {
      log::info!("i --- {:?}", i);
    }
    log::info!("log输出");
    let host = config::CONFIG["consul_addr"];
    let client = Client::new(host, 8500);
    let res = client.kv_folder_watch("foo").await;
  }
   */

  /*
  #[tokio::test]
  async fn test_kv_folder_watch_getnodes() {
    env_logger::init();
    for i in 0..3 {
      log::info!("i --- {:?}", i);
    }
    log::info!("log输出");
    let host = config::CONFIG["consul_addr"];
    let client = Client::new(host, 8500);
    let mut res = false;
    let (sx, mut rx) = mpsc::channel(1);

    tokio::task::spawn({
      let resx = client.kv_folder_watch("foo").await.unwrap();
      sx.send(resx).await.unwrap();
    });

    while let Some(resx) = rx.recv().await {
      log::info!("resx --- {:?}", resx);
    }
  }
   */

  #[test]
  fn test_kv_delete_both_session() {
    let host = config::CONFIG["consul_addr"];
    let client = Client::new(host, 8500);
    let res = client.kv_delete_both_session("my-key");
    println!("res ---------- {}", res.unwrap());
  }

  #[test]
  fn test_kv_get() {
    // let host = config::CONFIG["consul_addr"];
    let host = "consul_test";
    let client = Client::new(host, 8500);
    let val = client.kv_get("my-key");
    if val.eq("keyNoExists_or_valIsNull") {
      println!("key {} keyNoExists_or_valIsNull", "my-key");
    } else {
      println!("key {} exists", "my-key");
      println!("val ----- {}", val);
    }
  }

  #[test]
  fn test_session_set() {
    // let host = config::CONFIG["consul_addr"];
    let host = "consul_test";
    let client = Client::new(host, 8500);
    let se = client.session_set("15s".to_string(), "my-session".to_string(), "node1".to_string(), "release".to_string(), "10m0s".to_string());
  }

  #[test]
  fn test_kv_set_with_session() {
    let host = config::CONFIG["consul_addr"];
    let client = Client::new(host, 8500);
    let session = client.session_set("15s".to_string(), "my-session".to_string(), "node1".to_string(), "release".to_string(), "10m0s".to_string());

    let res = client.kv_set_with_session("my-key", "my-val", session.as_str()).unwrap();
    println!("res ------- {}", res);
  }

  #[test]
  fn test_session_renew() {
    let host = config::CONFIG["consul_addr"];
    let client = Client::new(host, 8500);
    let ok = client.session_renew("d5663534-82f9-429b-954c-ae63d59d3502");
    match ok {
      Ok(_) => { println!("---ok---");}
      Err(_) => { println!("---err---");}
    }
  }

  #[test]
  fn test_kv_ge_folder() {
    let host = config::CONFIG["consul_addr"];
    let client = Client::new(host, 8500);
    let res = client.kv_get_folder("pomid/Echo");
    let svcs = res.unwrap();
    println!("svcs ------------- {:?}", svcs);
  }

}





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
pub mod service;
pub mod vo;

use std::future::Future;
use self::kv::*;
use self::session::*;
use self::service::*;
use std::io::Read;
use crate::pkg::CustomError;
use base64::{decode};
use env_logger::Env;

#[derive(Debug)]
pub struct Client {
  pub debug: bool,

  host: String,
  port: u16,

  kv:   KVPair,
  session: Session,
  service: Service,
}

impl Client {
  pub fn new<S: Into<String>>(host: S, port: u16) -> Client {

    Client {
      debug: false,
      host: host.into(),
      port: port,
      kv: KVPair::new(),
      session: Session::new(),
      service: Service::new(),
    }
  }

  pub fn debug_print(&self, s: &str) {
    // if self.debug {
    //   println!("[DEBUG] --- {:?}", s);
    // }
    println!("[DEBUG] --- {:?}", s);
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

  pub async fn service_getnodes(&self, service_name: String) -> Vec<String> {
    let node_addrs = self.service.getnodes(self, service_name).await;
    match node_addrs {
      Ok(_) => {
        println!("-->>> service_get real nodes, {:?}", node_addrs);
        node_addrs.unwrap()
      }
      Err(_) => {
        println!("-->>> service_get no nodes");
        vec![]
      }
    }
  }

  pub async fn service_getall(&self, services: String) -> Vec<String> {
    todo!()
  }


}

#[cfg(test)]
mod tests {
  use crate::Client;
  use crate::config;

  #[tokio::test]
  async fn test_service_get() {
    let host = config::CONFIG["consul_addr"];
    let client = Client::new(host, 8500);
    let node_addrs = client.service_getnodes("neon_broker".to_string()).await;
    // println!("node_addrs ---------- {:?}", node_addrs);
  }
}





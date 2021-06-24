#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate reqwest;
extern crate serde_json;

pub mod kv;
use self::kv::*;
use std::io::Read;

pub struct Client {
  host: String,
  port: u16,
}

impl Client {
  pub fn new<S: Into<String>>(host: S, port: u16) -> Client {
    Client {
      host: host.into(),
      port: port,
    }
  }

  pub fn kv_get<S: Into<String>>(&self, key: S) -> Result<Vec<KVPair>, String> {
    let uri_str = format!("http://{}:{}/v1/kv/{}", self.host, self.port, key.into());
    let mut res = reqwest::get(&uri_str).map_err(|e| e.to_string())?;
    let mut body = String::new();
    res.read_to_string(&mut body).map_err(|e| e.to_string())?;
    // todo: success -> return Vec<KVPair>,  fail -> return error string
    serde_json::from_str::<Vec<KVPair>>(&body).map_err(|e| e.to_string())
  }

  pub fn kv_put<S: Into<String>>(&self, key: S, v: S) -> Result<bool, String> {
    let uri_str = format!("http://{}:{}/v1/kv/{}", self.host, self.port, key.into());
    let mut res = reqwest::Client::new()
      .put(&uri_str)
      .body(v.into())
      .send()
      .map_err(|e| e.to_string())?;
    let mut body = String::new();
    res.read_to_string(&mut body).map_err(|e| e.to_string())?;
    return Ok(body.as_str().contains("true"));
  }

  pub fn kv_delete<S: Into<String>>(&self, key: S) -> Result<bool, String> {
    let uri_str = format!("http://{}:{}/v1/kv/{}", self.host, self.port, key.into());
    let mut res = reqwest::Client::new()
      .delete(&uri_str)
      .send()
      .map_err(|e| e.to_string())?;
    let mut body = String::new();
    res.read_to_string(&mut body).map_err(|e| e.to_string())?;
    return Ok(body.as_str().contains("true"));
  }
}

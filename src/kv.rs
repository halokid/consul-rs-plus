use crate::{Client};
use std::io::Read;
use std::{thread, time};
use std::borrow::Borrow;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KVPair {
  pub key: String,
  pub create_index: u64,
  pub modify_index: u64,
  pub lock_index: u64,
  pub flags: u64,
  pub value: String,
  #[serde(default = "default_string")]
  pub session: String,
}

fn default_string() -> String {
  "".to_string()
}

impl KVPair {
  pub fn new() -> Self {
    KVPair {
      key: "".to_string(),
      create_index: 0,
      modify_index: 0,
      lock_index: 0,
      flags: 0,
      value: "".to_string(),
      session: "".to_string(),
    }
  }

  // todo: Into<String> get the ownship for varible?
  pub fn get<S: Into<String>>(&self, c: &Client, key: S) -> Result<Vec<KVPair>, String> {
    let url = format!("http://{}:{}/v1/kv/{}", c.host, c.port, key.into());
    let mut rsp = reqwest::get(&url).map_err(|e| e.to_string())?;
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    // todo: success -> return Vec<KVPair>,  fail -> return error string
    serde_json::from_str::<Vec<KVPair>>(&body).map_err(|e| e.to_string())
  }

  pub fn set<S: Into<String>>(&self, c: &Client, key: S, v: S) -> Result<bool, String> {
    let url = format!("http://{}:{}/v1/kv/{}", c.host, c.port, key.into());
    let mut rsp = reqwest::Client::new()
      .put(&url)
      .body(v.into())
      .send()
      .map_err(|e| e.to_string())?;
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    return Ok(body.as_str().contains("true"));
  }

  pub fn set_with_session<S: Into<String>>(&self, c: &Client, key: S, v: S, session: S)
                                           -> Result<bool, String> {
    let url = format!("http://{}:{}/v1/kv/{}?acquire={}", c.host, c.port,
                      key.into(), session.into());
    let mut vx = v.into();
    let vxx = vx.clone();

    let mut rsp = reqwest::Client::new()
      .put(&url)
      // .body(v.into())
      .body(vx)
      .send()
      .map_err(|e| e.to_string())?;
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    c.debug_print(format!("set_with_session debug: {:?}", body).as_str());
    // return Ok(body.as_str().contains("true"));
    if !body.as_str().contains("true") {
      let mut loop_flag = true;
      let mut loop_num = 0;
      while loop_flag && loop_num < 10 {
        c.debug_print("---loop_flag set_with_session---");
        let mut rsp = reqwest::Client::new()
          .put(&url)
          // todo: loop will get varible own, so use ref
          .body(vxx.as_str().to_string())
          .send()
          .map_err(|e| e.to_string())?;
        let mut body = String::new();
        rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
        c.debug_print(format!("set_with_session loop debug: {:?}", body).as_str());
        thread::sleep(time::Duration::from_secs(1));

        if body.as_str().contains("true") {
          loop_flag = false
        }
        loop_num += 1;
      }
    }
    Ok(true)
  }

  pub fn delete<S: Into<String>>(&self, c: &Client, key: S) -> Result<bool, String> {
    let url = format!("http://{}:{}/v1/kv/{}", c.host, c.port, key.into());
    let mut rsp = reqwest::Client::new()
      .delete(&url)
      .send()
      .map_err(|e| e.to_string())?;
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    return Ok(body.as_str().contains("true"));
  }

  pub fn get_value(&self) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode(&self.value)
  }
}


#[cfg(test)]
mod tests {
  use crate::Client;

  #[test]
  fn unmarshal_kv_pair() {
    let dat = r#"[
        {
            "LockIndex":666,
            "Key":"key/path",
            "Flags":0,
            "Value":"dHJ1ZQ==",
            "CreateIndex":495627,
            "ModifyIndex":495627
        }]"#;
    let v: Vec<super::KVPair> = serde_json::from_str(dat).unwrap();
    assert_eq!(v[0].lock_index, 666);
    assert_eq!(v[0].get_value().unwrap(), "true".as_bytes().to_owned())
  }

  // #[test]
  // fn test_get_kv() {
  //   let client = Client::new("8.8.8.8", 8500);
  //   let my_keys = client.kv_get("my-key").unwrap();
  //   for k in my_keys {
  //     println!("k: {:?}", k);
  //   }
  // }
}






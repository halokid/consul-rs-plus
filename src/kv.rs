use crate::{Client, PKGX};
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
    let vxx = vx.clone();   //  todo: 这里定义 vxx 是不会错误的
    let xx = &vxx.to_string();
    let yy = vxx.as_str().to_string();

    let zz = &vxx;
    let kk = vxx.as_str();

    // todo: 要注意上面几种类型的变化， 简单来说消除 value borrowed here after move 这种错误， 一般都是用引用

    let mut rsp = reqwest::Client::new()
      .put(&url)
      // .body(v.into())
      .body(vx)
      .send()
      .map_err(|e| e.to_string())?;
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    PKGX.debug_print(format!("set_with_session debug: {:?}", body).as_str());
    // return Ok(body.as_str().contains("true"));
    if !body.as_str().contains("true") {
      // let vx = v.borrow();
      let mut loop_flag = true;
      while loop_flag {
        // let x = &vx.to_string();
        // let vxx = xv.into();
        // let vxx = vx.clone();   // todo: 错误 value borrowed here after move 的问题是出现在这里， 因为 while循环的时候， 要每一个执行了 rsp 的封装都会转移 vxx 的所有权
        let mut rsp = reqwest::Client::new()
          .put(&url)
          // .body("xxxx")
          // .body(&vxx.to_string()) // todo: 这样会出错 the trait `From<&std::string::String>` is not implemented for `reqwest::Body`
          .body(vxx.as_str().to_string())
          // .body(vxx.clone())
          .send()
          .map_err(|e| e.to_string())?;
        let mut body = String::new();
        rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
        PKGX.debug_print(format!("set_with_session loop debug: {:?}", body).as_str());
        thread::sleep(time::Duration::from_secs(2));

        if body.as_str().contains("true") {
          loop_flag = false
        }
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






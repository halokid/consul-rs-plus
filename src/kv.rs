use crate::{Client};
use std::io::Read;
use std::{thread, time};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::prelude::v1::Option::Some;
use crate::pkg::CustomError;
use tokio::sync::mpsc;
use std::thread::sleep;
use tokio::time::Duration;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KVPair {
  pub Key: String,
  pub CreateIndex: u64,
  pub ModifyIndex: u64,
  pub LockIndex: u64,
  pub Flags: u64,
  pub Value: String,
  #[serde(default = "default_string")]
  pub Session: String,
}

fn default_string() -> String {
  "".to_string()
}

impl KVPair {
  pub fn new() -> Self {
    KVPair {
      Key: "".to_string(),
      CreateIndex: 0,
      ModifyIndex: 0,
      LockIndex: 0,
      Flags: 0,
      Value: "".to_string(),
      Session: "".to_string(),
    }
  }

  // todo: Into<String> get the ownship for varible?
  pub fn get<S: Into<String>>(&self, c: &Client, key: S) -> Result<Vec<KVPair>, String> {
    let url = format!("http://{}:{}/v1/kv/{}", c.host, c.port, key.into());
    let mut rsp = reqwest::get(&url).map_err(|e| e.to_string())?;
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    // todo: success -> return Vec<KVPair>,  fail -> return error string
    c.debug_print(format!("kv get body --- {}", body).as_str());
    serde_json::from_str::<Vec<KVPair>>(&body).map_err(|e| e.to_string())
  }

  pub fn get_folder_keys<S: Into<String>>(&self, c: &Client, key: S) -> Result<String,
                String> {
    let url = format!("http://{}:{}/v1/kv/{}/?keys", c.host, c.port, key.into());
    let mut rsp = reqwest::get(&url).map_err(|e| e.to_string())?;
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    Ok(body)
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
    c.debug_print(format!("key set_with_session val: {:?}", vx).as_str(), );
    // let vxx = vx.clone();

    let mut rsp = reqwest::Client::new()
      .put(&url)
      // .body(v.into())
      .body(vx)
      .send()
      .map_err(|e| e.to_string())?;
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    c.debug_print(format!("set_with_session debug: {:?}", body).as_str(), );
    return Ok(body.as_str().contains("true"));
    /*
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
        // thread::sleep(time::Duration::from_secs(1));
        if body.as_str().contains("true") {
          loop_flag = false;
          return Ok(true);
        }
        loop_num += 1;
        thread::sleep(time::Duration::from_secs(19))
      }
      return Ok(false);
    }
    Ok(true)
     */
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

  pub fn delete_both_session<S: Into<String>>(&self, c: &Client, key: S) -> Result<bool, String> {
    // get kv session
    let keyx = key.into();
    let keyx_ref = keyx.as_str();
    let kv = self.get(c, keyx_ref.to_string());
    match kv {
      Err(err) => {
        return Err(format!("can not get kv {} {}", keyx_ref.to_string(), err));
      }
      _ => {}
    }
    let kvx = kv.unwrap();
    c.debug_print(format!("kvx ------ {:?}", kvx).as_str());
    let kvx_unwp = kvx.get(0).unwrap();
    let sid = &kvx_unwp.Session;
    c.debug_print(format!("delete_both_session sid: {}", sid).as_str());
    //del session
    c.session_delete(sid);

    let url = format!("http://{}:{}/v1/kv/{}", c.host, c.port, keyx_ref.to_string());
    let mut rsp = reqwest::Client::new()
      .delete(&url)
      .send()
      .map_err(|e| e.to_string())?;
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    return Ok(body.as_str().contains("true"));
  }

  pub fn get_value(&self) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode(&self.Value)
  }

  /*
  pub async fn watch_tree<S: Into<String>>(&self, c: &Client, folder: S,
              mut svc_nodes: HashMap<String, Vec<String>>) -> Result<bool, String> {
    log::info!("watch_tree log输出 1");
    println!("watch_tree log输出 2");
    log::info!("watch_tree log输出 3");
    let hostx = c.host.clone();
    let portx = c.port.clone();
    let url = format!("http://{}:{}/v1/kv/{}", c.host, c.port, folder.into());
    let mut rsp = reqwest::get(&url).map_err(|e| e.to_string())?;
    let header = rsp.headers();
    log::info!("header ----- {:?}, {:?}", header, header.get("x-consul-index").unwrap());
    let origin_index = header.get("x-consul-index").unwrap();
    let (index_check, mut rx) = mpsc::channel(1);

    tokio::task::spawn(async move {
      for i in 0..3 {
        sleep(Duration::from_secs(5));
        let url = format!("http://{}:{}/v1/kv/foo", hostx, portx);
        let mut rspx = reqwest::get(&url).map_err(|e| e.to_string()).unwrap();
        let header = rspx.headers();
        log::info!("header in spawn ----- {:?}, {:?}", header, header.get("x-consul-index").unwrap());
        let check_index = header.get("x-consul-index").unwrap();
        // let check_index = "xx";
        let check_index_owen = check_index.to_owned();
        index_check.send(check_index_owen).await.unwrap();
        log::info!("=== watch tree spawn ===");
      }
    });

    while let Some(check_index) = rx.recv().await {
      log::info!("check --- {:?}", check_index);
      svc_nodes.insert("key".into(), "val".into());
    }

    sleep(Duration::from_secs(30));
    Ok(true)
  }
   */
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
    assert_eq!(v[0].LockIndex, 666);
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






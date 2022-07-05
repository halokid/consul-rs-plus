use crate::{Client};
use std::io::Read;
use std::{thread, time};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::prelude::v1::Option::Some;
use crate::pkg::CustomError;
use tokio::sync::mpsc;
// use std::sync::mpsc;
use std::thread::sleep;
use tokio::time::Duration;
use std::str;

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
  timeout:        u64,
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
      timeout:      600,
    }
  }

  // todo: Into<String> get the ownship for varible?
  pub fn get<S: Into<String>>(&self, c: &Client, key: S) -> Result<Vec<KVPair>, String> {
    let url = format!("http://{}:{}/v1/kv/{}", c.host, c.port, key.into());
    // let mut rsp = reqwest::get(&url).map_err(|e| e.to_string())?;
    let mut rsp = reqwest::Client::builder().timeout(Duration::from_secs(self.timeout)).build().unwrap().get(&url).send().map_err(|e| e.to_string()).unwrap();
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    // todo: success -> return Vec<KVPair>,  fail -> return error string
    c.debug_print(format!("kv get body --- {}", body).as_str());
    serde_json::from_str::<Vec<KVPair>>(&body).map_err(|e| e.to_string())
  }

  pub fn get_folder_keys<S: Into<String>>(&self, c: &Client, key: S) -> Result<String,
    String> {
    let url = format!("http://{}:{}/v1/kv/{}/?keys", c.host, c.port, key.into());
    // let mut rsp = reqwest::get(&url).map_err(|e| e.to_string())?;
    let mut rsp = reqwest::Client::builder().timeout(Duration::from_secs(self.timeout)).build().unwrap().get(&url).send().map_err(|e| e.to_string()).unwrap();
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    Ok(body)
  }

  pub fn set<S: Into<String>>(&self, c: &Client, key: S, v: S) -> Result<bool, String> {
    let url = format!("http://{}:{}/v1/kv/{}", c.host, c.port, key.into());
    // let mut rsp = reqwest::Client::new()
    let mut rsp = reqwest::Client::builder().timeout(Duration::from_secs(self.timeout))
      .build().unwrap()
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
    let keyx = key.into();
    let keyxx = keyx.clone();
    let url = format!("http://{}:{}/v1/kv/{}?acquire={}", c.host, c.port,
                      keyx, session.into());
    let mut vx = v.into();
    c.debug_print(format!("key set_with_session val: {:?}", vx).as_str());
    // let vxx = vx.clone();

    // let mut rsp = reqwest::Client::new()
    let mut rsp = reqwest::Client::builder().timeout(Duration::from_secs(self.timeout))
      .build().unwrap()
      .put(&url)
      // .body(v.into())
      .body(vx)
      .send()
      .map_err(|e| e.to_string())?;
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string())?;
    c.debug_print(format!("{:?} set_with_session debug: {:?}", keyxx, body).as_str());
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
    // let mut rsp = reqwest::Client::new()
    let mut rsp = reqwest::Client::builder().timeout(Duration::from_secs(self.timeout))
      .build().unwrap()
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
    // let mut rsp = reqwest::Client::new()
    let mut rsp = reqwest::Client::builder().timeout(Duration::from_secs(self.timeout))
      .build().unwrap()
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

  fn get_folder_index<S: Into<String>>(&self, c: &Client, folder: S) -> String {
    let url = format!("http://{}:{}/v1/kv/{}/", c.host, c.port, folder.into());
    // let mut rspx = reqwest::get(&url).map_err(|e| e.to_string()).unwrap();
    let mut rspx = reqwest::Client::builder().timeout(Duration::from_secs(self.timeout)).build().unwrap().get(&url).send().map_err(|e| e.to_string()).unwrap();
    let header = rspx.headers();
    let index = header.get("X-Consul-Index").unwrap();
    let index_s = index.to_str().unwrap().to_string();
    index_s
  }

  // if you use cakeRabbit micro-service frmework, it use folder for service nodes
  // you can get all the service nodes use this fn, the consul API: /v1/kv/folder?keys
  fn get_folder_allkeys<S: Into<String>>(&self, c: &Client, folder: S) -> Vec<String> {
    let url = format!("http://{}:{}/v1/kv/{}?keys", c.host, c.port, folder.into());
    // let mut rsp = reqwest::get(&url).map_err(|e| e.to_string()).unwrap();
    let mut rsp = reqwest::Client::builder().timeout(Duration::from_secs(self.timeout)).build().unwrap().get(&url).send().map_err(|e| e.to_string()).unwrap();

    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err(|e| e.to_string());
    // todo: success -> return Vec<KVPair>,  fail -> return error string
    c.debug_print(format!("get_folder_allkeys body --- {}", body).as_str());
    let nodes_v: Vec<String> = serde_json::from_str(&body).unwrap();
    // println!("nodes_v ---------- {:?}", nodes_v);
    nodes_v
  }
}


#[cfg(test)]
mod tests {
  use crate::Client;
  use crate::kv::KVPair;
  use tokio::sync::mpsc as tmpsc;
  use tokio::sync::oneshot as toneshot;
  use std::sync::mpsc as smpsc;
  use std::{thread, time};
  use log;
  use chrono::prelude::*;

  #[test]
  fn test_get_folder_allkeys() {
    let kv = KVPair::new();
    let client = Client::new("consul_test", 8500);
    let nodes_v = kv.get_folder_allkeys(&client, "mytest".to_string());
    println!("node_v -------- {:?}", nodes_v);
    println!("node_v 0 -------- {:?}", &nodes_v[0..1]);
  }

  #[test]
  fn test_get_folder_index() {
    let kv = KVPair::new();
    let client = Client::new("consul_test", 8500);
    let index = kv.get_folder_index(&client, "mytest".to_string());
    println!("index ------ {}", index);
  }

  // todo: for micro service, service client should watch the service tree, load the services
  // todo: cache, dont need to request the register center every call time.
  // todo: this fn should be running in main thread. while get sx.send, then reload services cache.
  #[tokio::test]
  async fn test_watch_folder_tree_tmpsc() {
    env_logger::init();
    let folder = "mytest".to_string();
    let mut nodes_service: Vec<String> = Vec::new();     // service cache
    // todo: if the index change, send the new nodes services between coroutine
    let (sx, mut rx) = tmpsc::channel(1);
    // let (mut sx, mut rx) = toneshot::channel();
    // let (sx, mut rx) = smpsc::channel();
    let kv = KVPair::new();
    let client = Client::new("consul_test", 8500);
    let mut index = kv.get_folder_index(&client, &folder);
    log::info!("index orgin ------- {}", index);

    tokio::task::spawn(async move {
      // loop {
      for i in 0..10 {
        thread::sleep(time::Duration::from_secs(5));
        // /*
        let mut index_ck = kv.get_folder_index(&client, &folder);
        log::info!("index_ck ------- {}", index_ck);
        if !index_ck.eq(index.as_str()) {
          log::info!("=== get newest nodes service, send coroutine ===");
          let nodes_v = kv.get_folder_allkeys(&client, &folder);
          let nodes_v_cl = nodes_v.clone();
          log::info!("[send] === in spawn nodes_v_cl: {:?}", nodes_v_cl);
          sx.send(nodes_v_cl).await.unwrap();    // todo: just make the channel full!
          // sx.send(nodes_v_cl).await.unwrap();
          index = index_ck;
        } else {
          log::info!("=== nodes_service no change ===");
          // sx.send(vec![]).await.unwrap();
        }
        // */

        // log::info!("=== get newest nodes service, send coroutine ===");
        // let nodes_v = kv.get_folder_allkeys(&client, &folder);
        // let nodes_v_cl = nodes_v.clone();
        // log::info!("[send] === in spawn nodes_v_cl: {:?}", nodes_v_cl);
        // sx.send(nodes_v_cl).await.unwrap();    // todo: just make the channel full!

        // &sx.send(nodes_v_cl).unwrap();
      }
    });

    // /*
    while let Some(nodes_v) = rx.recv().await {
      log::info!("=== [一次recv开始] ===");
      log::info!("[recv 1] === recv nodes_v --- {:?}", nodes_v);
      nodes_service = nodes_v;
      log::info!("[recv 2] === reload nodes_service --- {:?}", nodes_service);
      log::info!("=== [一次recv结束] ===");
      // return nodes_v;
    }
    // */

    // todo: if channle is oneshot
    // match rx.await {
    //   Ok(v) => { println!("v --- {:?}", v); }
    //   Err(e) => { println!("err ---- {}", e); }
    // }

    // todo: will not run!!!
    log::info!("nodes_service now is --- {:?}", nodes_service)
    // return vec![];
  }

  #[tokio::test]
  async fn test_tokio_channel() {
    println!("====================================================================");
    // loop send ----------------------------------------
    let (sx, mut rx) = tmpsc::channel(1);

    tokio::task::spawn(async move {
      for i in 0..9 {
        thread::sleep(std::time::Duration::from_secs(5));
        println!("=== send by tokio spawn --- {}, {}", i, Local::now());
        sx.send(i).await.unwrap();
      }
    });

    while let Some(i) = rx.recv().await {
      // println!("5 now time is {:?}", Local::now());
      println!("=== recv from tokio spawn --- {}, {}", i, Local::now());
    }

    // sleep(Duration::from_secs(10));
    // println!("6 now time is {:?}", Local::now());
  }

  #[tokio::test]
  async fn test_watch_folder_tree_smpsc() {
    env_logger::init();
    let mut nodes_service: Vec<String> = Vec::new();     // service cache
    // todo: if the index change, send the new nodes services between coroutine
    let (sx, mut rx) = smpsc::channel();
    let kv = KVPair::new();
    let client = Client::new("consul_test", 8500);
    let mut index = kv.get_folder_index(&client, "mytest".to_string());
    log::info!("index orgin ------- {}", index);

    tokio::task::spawn(async move {
      for i in 0..90 {
        thread::sleep(time::Duration::from_secs(5));
        let mut index_ck = kv.get_folder_index(&client, "mytest".to_string());
        log::info!("index_ck ------- {}", index_ck);
        if !index_ck.eq(index.as_str()) {
          log::info!("=== get newest nodes service, send coroutine ===");
          let nodes_v = kv.get_folder_allkeys(&client, "mytest".to_string());
          sx.send(nodes_v).unwrap();
          index = index_ck;
        } else {
          log::info!("=== nodes_service no change ===");
          // sx.send(vec![]).await.unwrap();
        }
      }
    }).await;   // todo: this is await will loop run the tokio::spawn, so cannot run next process

    // todo: will not run forever!!!!!
    let nodes_v = rx.recv().unwrap();
    nodes_service = nodes_v;
    log::info!("reload nodes_service --- {:?}", nodes_service);
    // return nodes_v;

    // return vec![];
  }

  /*
  #[tokio::test]
  async fn test_get_newest_folder_tree() {
    env_logger::init();
    let nodes_v = test_watch_folder_tree().await;
    log::info!("nodes_v ---------- {:?}", nodes_v);
  }
   */


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



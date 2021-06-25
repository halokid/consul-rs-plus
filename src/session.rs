use crate::Client;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Session {
  pub id:       String,
  pub name:     String,
  pub node:     String,
  pub lock_delay: String,
  pub behavior:  String,
  pub ttl:      String,
  pub node_checks:  Vec<String>,
  service_checks: Option<String>,
  create_index:   u32,
  modify_index:   u32,
}

impl Session {

  pub fn new() -> Self {
    Session {
      id: "".to_string(),
      name: "".to_string(),
      node: "".to_string(),
      lock_delay: "".to_string(),
      behavior: "".to_string(),
      ttl: "".to_string(),
      node_checks: vec![],
      service_checks: None,
      create_index: 0,
      modify_index: 0
    }
  }

  pub fn set(&self, c: &Client, s: &Session) -> String {
    let url = format!("http://{}:{}/v1/session/create", c.host, c.port);
    let payload = serde_json::to_string(s).unwrap();
    println!("payload ------ {}", payload);
    let mut rsp = reqwest::Client::new()
      .put(&url)
      .body(payload)
      .send()
      .map_err( |e| e.to_string() ).unwrap();
    let mut body = String::new();
    rsp.read_to_string(&mut body).map_err( |e| e.to_string());
    println!("body --------- {}", body);
    body
    // "".to_string()
  }
}






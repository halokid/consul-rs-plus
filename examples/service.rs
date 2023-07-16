use serde_json::{json, Value};
use consul_rs_plus::Client;
use consul_rs_plus::service::Service;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  // env_logger::init();
  println!("-->>> service examples");

  let client = Client::new("localhost", 8500);
  let service_name = "neon_broker";
  let s = Service::new(client, service_name);
  // let s = Service::new(client, service_name.to_string());
  let nodes = s._get_nodes().await;
  log::info!("nodes -->>> {:?}", nodes);

  let nodes_health = s._get_health().await;
  log::info!("nodes_health -->>> {:?}", nodes_health);


  let mut service_addrs = Vec::new();
  for node_health_key in nodes_health {
    // let nodex = node_health_key.clone();
    // service_addrs.push(nodes[nodex]);
    // service_addrs.push(nodes.get(nodex.as_str()));
    service_addrs.push(nodes.get(node_health_key.as_str()).unwrap());
  }
  log::info!("service_addrs -->>> {:?}", service_addrs);


  // let js = json!(rsp);
  // println!("ServiceID -->>> {:?}", js);
  // let v: Value = serde_json::from_str(rsp.as_str()).unwrap();
  // println!("ServiceID -->>> {:?}", v[0]["ServiceID"].as_str().unwrap());
  // log::info!("ServiceID -->>> {:?}", v[0]["ServiceID"].as_str().unwrap());

  // for dynamic json
  // let url = "http://localhost:8500/v1/catalog/service/neon_broker";
  // let rsp = reqwest::Client::new().get(url).send().await.unwrap();
  // let rsp_json: serde_json::Value = rsp.json();
  // println!("{:#?}", rsp_json);

  /*
  let echo_json: serde_json::Value = reqwest::Client::new()
    .post("https://jsonplaceholder.typicode.com/posts")
    .json(&serde_json::json!({
            "title": "Reqwest.rs",
            "body": "https://docs.rs/reqwest",
            "userId": 1
        }))
    .send()
    .await?
    .json()
    .await?;

  println!("{:#?}", echo_json);
   */

  Ok(())
}


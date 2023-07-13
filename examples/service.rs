use serde_json::{json, Value};
use consul_rs_plus::Client;
use consul_rs_plus::service::Service;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  // env_logger::init();
  println!("-->>> service examples");

  let client = Client::new("localhost", 8500);
  let s = Service::new();
  let rsp = s._get_nodes(&client, "neon_broker").await;
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


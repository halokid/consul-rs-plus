use consul_rs_plus::Client;
use consul_rs_plus::service::Service;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  println!("-->>> service examples");

  let client = Client::new("localhost", 8500);
  let s = Service::new();
  s._get_nodes(&client, "neon_broker").await;

  // for dynamic json
  let url = "http://localhost:8500/v1/catalog/service/neon_broker";

  let rsp = reqwest::Client::new().get(url).send().await.unwrap();
  let rsp_json: serde_json::Value = rsp.json();
  println!("{:#?}", rsp_json);

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


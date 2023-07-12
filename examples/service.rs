use consul_rs_plus::Client;
use consul_rs_plus::service::Service;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  let client = Client::new("consul_test", 8500);
  let s = Service::new();
  s._get_nodes(&client, "neon_broker").await;

  println!("-->>> service examples");
  Ok(())
}


use std::error::Error;
use crate::Client;
use crate::pkg::CustomError;

pub struct Service {
  name: String
}

impl Service {

  pub fn new() -> Self {
    Service{
      name: "".to_string()
    }
  }

  /// Get the health services, base on below API url
  /// http://localhost:8500/v1/catalog/service/neon_broker
  /// http://localhost:8500/v1/health/checks/neon_broker
  pub fn get(&self, c: &Client, service_name: &str) -> Result<Vec<String>, CustomError> {
    let url = format!("http://{}:{}/v1/health/checks/{}", c.host, c.port, service_name);

    Ok(vec![])
  }

  /// return []String service_id
  pub async fn _get_nodes(&self, c: &Client, service_name: &str) {
    let url = format!("http://{}:{}/v1/catalog/service/{}", c.host, c.port, service_name);
    println!("Fetching {:?}...", url);

    // reqwest::get() is a convenience function.
    //
    // In most cases, you should create/build a reqwest::Client and reuse
    // it for all requests.
    let rsp = reqwest::get(url).await;

    let res = rsp.unwrap();
    println!("Response: {:?} {}", res.version(), res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = res.text().await;

    println!("{:?}", body);
  }

  /// return `service_id: staus` hashmap
  pub fn _get_health() {

  }
}

#[cfg(test)]
mod tests {
  use crate::Client;
  use crate::service::Service;

  #[test]
  fn test_get_nodes() {
    let client = Client::new("consul_test", 8500);
    let s = Service::new();
    s._get_nodes(&client, "neon_broker");

  }

}













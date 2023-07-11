use std::error::Error;
use crate::Client;

pub struct Service {
  name: String
}

impl Service {

  /// Get the health services, base on below API url
  /// http://localhost:8500/v1/catalog/service/neon_broker
  /// http://localhost:8500/v1/health/checks/neon_broker
  pub fn get(&self, c: &Client, service_name: &str) -> Result<Vec<String>, dyn Error> {
    let url = format!("http://{}:{}/v1/health/checks/{}", c.host, c.port, service_name);

    Ok(vec![])
  }


}
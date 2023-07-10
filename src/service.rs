use std::error::Error;
use crate::Client;

pub struct Service {
  name: String
}

impl Service {

  /// Get the health services, url `/v1/health/checks/:service_name`
  pub fn get(&self, c: &Client, service_name: &str) -> Result<Vec<String>, Error> {
    let url = format!("http://{}:{}/v1/health/checks/{}", c.host, c.port, service_name);

    Ok(vec![])
  }


}
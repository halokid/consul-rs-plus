use std::collections::HashMap;
use std::error::Error;
use log::log;
use serde_json::Value;
use crate::Client;
use crate::pkg::CustomError;

pub struct Service {
  // consul_client: Client,
  // name: String,
}

impl Service {
  /*
  pub fn new<S: Into<String>>(c: Client, name: S) -> Self {
    Service {
      consul_client: c,
      name: name.into(),
    }
  }
   */
  pub fn new() -> Self {
    Service {
      // consul_client: c,
    }
  }

  /// Get the health services, base on below API url
  /// http://localhost:8500/v1/catalog/service/neon_broker
  /// http://localhost:8500/v1/health/checks/neon_broker
  pub async fn get<S: Into<String>>(&self, c: &Client, service_name: S)
    -> Result<Vec<String>, CustomError> {
    let service_name = service_name.into();
    let nodes = self._get_nodes(c, service_name.as_str()).await;
    let nodes_health = self._get_health(c, service_name.as_str()).await;
    let mut service_addrs = Vec::new();
    for health_key in nodes_health {
      let v = nodes.get(health_key.as_str()).unwrap().to_string();
      service_addrs.push(v);
    }
    Ok(service_addrs)
    // Ok(vec![])
  }

  /// return []String service_id
  pub async fn _get_nodes(&self, c: &Client, service_name: &str) -> HashMap<String, String> {
    let url = format!("http://{}:{}/v1/catalog/service/{}", c.host, c.port, service_name);
    // println!("Fetching {:?}...", url);
    log::info!("Fetching {:?}...", url);

    // reqwest::get() is a convenience function.
    // In most cases, you should create/build a reqwest::Client and reuse
    // it for all requests.
    let rsp = reqwest::get(url).await;
    let res = rsp.unwrap();
    // println!("Response: {:?} {}", res.version(), res.status());
    // println!("Headers: {:#?}\n", res.headers());
    let body = res.text().await;
    // println!("{:?}", body);
    // body.unwrap()
    let mut nodes = HashMap::new();
    let body_js: Value = serde_json::from_str(body.unwrap().as_str()).unwrap();
    // log::info!("body_js -->>> {:?}", body_js);
    // let body_js_arr = body_js.as_array();
    match body_js.as_array() {
      None => {}

      Some(_) => {
        for service in body_js.as_array().unwrap() {
          log::debug!("ServiceID -->>> {:?}, {}, {}", service["ServiceID"], service["ServiceID"],
          service["ServiceID"].as_str().unwrap().to_string());
          let node_addr = format!("{}:{}",
                                  service["ServiceAddress"].as_str().unwrap().to_string(),
                                  service["ServicePort"].as_u64().unwrap().to_string());
          log::info!("node_addr -->>> {}", node_addr);
          nodes.insert(service["ServiceID"].as_str().unwrap().to_string(), node_addr);
        }
      }
    }
    nodes
  }

  /// return `service_id: staus` hashmap
  pub async fn _get_health(&self, c: &Client, service_name: &str) -> Vec<String> {
    let url = format!("http://{}:{}/v1/health/checks/{}", c.host, c.port, service_name);
    log::info!("Fetching {:?}...", url);
    let rsp = reqwest::get(url).await;
    let res = rsp.unwrap();
    let body = res.text().await;
    // let mut node_health = HashMap::new();
    let mut nodes_health: Vec<String> = Vec::new();
    let body_js: Value = serde_json::from_str(body.unwrap().as_str()).unwrap();
    // log::info!("body_js -->>> {:?}", body_js);
    match body_js.as_array() {
      None => {}

      Some(body_js_arr) => {
        for service in body_js_arr {
          let status = service["Status"].as_str().unwrap();
          log::debug!("{} status -->>> {:?}", service["ServiceID"], status);
          if status == "passing" {
            nodes_health.push(service["ServiceID"].as_str().unwrap().to_string());
          }
        }
      }
    }
    nodes_health
  }
}

#[cfg(test)]
mod tests {
  use crate::Client;
  use crate::service::Service;

  // #[test]
  // fn test_get_nodes() {
  //   let client = Client::new("consul_test", 8500);
  //   let s = Service::new();
  //   s._get_nodes(&client, "neon_broker");
  // }
}













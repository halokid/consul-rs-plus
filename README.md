# consul-rs-plus


A plus consul client package for Rust, more functions for microservice.

- [x] key/value operation
- [x] session operation
- [x] key/value & session combination operation
- [x] key/value folder values(for micro service nodes info)
- [x] watch keys tree change function(usual use in microservice)


## install
set in Cargo dependencies
```toml
[dependencies]
consul-rs-plus = "0.1.7"
```

## Usage
```rust
extern crate consul_rs_plus;
use consul_rs_plus::Client;

fn main() {
    let mut c = Client::new("localhost", 8500);
    // debug enable
    c.debug = true;

    let ok = c.kv_set("test-key", "test_value").unwrap();
    assert_eq!(ok, true);

    let kvpairs = c.kv_get("test-key").unwrap();
    let kvpair = &kvpairs[0];
    let v = kvpair.get_value().unwrap();
    assert_eq!(b"test_value"[..].to_vec(), v);
  
    let ok = c.kv_delete("test-key").unwrap();
    assert_eq!(ok, true);
}
```

## Test
the test case write in function code file or tests folder, all nromal operation test in lib.rs.
```rust
#[cfg(test)]
mod tests {
  use crate::Client;
  use base64::Config;
  use crate::config;

  #[test]
  fn test_kv_get() {
    let host = config::CONFIG["consul_addr"];
    let client = Client::new(host, 8500);
    let my_keys = client.kv_get("my-key").unwrap();
    for k in my_keys {
      println!("k: {:?}", k);
    }
  }
  
}
```

## Micro service nodes watch
if you write the micro service framework, the service register like [rpcx-plus](https://github.com/halokid/rpcx-plus), folder tree style, for example， Echo service in folder(key, value) is 
`` /mytest/Echo/tcp@8.8.8.8:999 ``, you can find the sample code for watch service change below,
they are in code repo. you can cache the service info when the service change(the folder tree change).
```rust

#[tokio::test]
  async fn test_watch_folder_tree_tmpsc() {
  env_logger::init();
  let folder = "mytest".to_string();
  let mut nodes_service: Vec<String> = Vec::new();     // service cache
  let (sx, mut rx) = tmpsc::channel(1);
  let kv = KVPair::new();
  let client = Client::new("consul_test", 8500);
  let mut index = kv.get_folder_index(&client, &folder);
  log::info!("index orgin ------- {}", index);
  tokio::task::spawn(async move {
    loop {
      thread::sleep(time::Duration::from_secs(5));
      let mut index_ck = kv.get_folder_index(&client, &folder);
      log::info!("index_ck ------- {}", index_ck);
      if !index_ck.eq(index.as_str()) {
        log::info!("=== get newest nodes service, send coroutine ===");
        let nodes_v = kv.get_folder_allkeys(&client, &folder);
        let nodes_v_cl = nodes_v.clone();
        log::info!("[send] === in spawn nodes_v_cl: {:?}", nodes_v_cl);
        sx.send(nodes_v_cl).await.unwrap();    // todo: just make the channel full!
        index = index_ck;
      } else {
        log::info!("=== nodes_service no change ===");
      }
    }
  });
}

```
you can run this test case in code repo.




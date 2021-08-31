# consul-rs-plus


A plus consul client package for Rust, more functions for microservice.

- [x] key/value operation
- [x] session operation
- [x] key/value & session combination operation
- [x] key/value folder values(for micro service nodes info)
- [ ] watch keys tree change function(usual use in microservice)


## install
set in Cargo dependencies
```toml
[dependencies]
consul-rs-plus = "0.1.6"
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





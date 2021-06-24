extern crate consul_rs_plus;

use consul_rs_plus::Client;

#[test]
fn kv() {
    let c = Client::new("localhost", 8500);
    let ok = c.kv_put("test-key", "test_value").unwrap();
    assert_eq!(ok, true);
    let pairs = c.kv_get("test-key").unwrap();
    let pair = &pairs[0];
    let v = pair.get_value().unwrap();
    assert_eq!(b"test_value"[..].to_vec(), v);
    let ok = c.kv_delete("test-key").unwrap();
    assert_eq!(ok, true);
}

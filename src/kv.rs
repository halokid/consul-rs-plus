#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct KVPair {
  pub key: String,
  pub create_index: u64,
  pub modify_index: u64,
  pub lock_index: u64,
  pub flags: u64,
  pub value: String,
  #[serde(default = "default_string")]
  pub session: String,
}

fn default_string() -> String {
  "".to_string()
}

impl KVPair {
  pub fn get_value(&self) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode(&self.value)
  }
}


#[cfg(test)]
mod tests {
  use crate::Client;

  #[test]
  fn unmarshal_kv_pair() {
    let dat = r#"[
        {
            "LockIndex":666,
            "Key":"key/path",
            "Flags":0,
            "Value":"dHJ1ZQ==",
            "CreateIndex":495627,
            "ModifyIndex":495627
        }]"#;
    let v: Vec<super::KVPair> = serde_json::from_str(dat).unwrap();
    assert_eq!(v[0].lock_index, 666);
    assert_eq!(v[0].get_value().unwrap(), "true".as_bytes().to_owned())
  }

  #[test]
  fn test_get_kv() {
    let client = Client::new("8.8.8.8", 8500);
    let my_keys = client.kv_get("my-key").unwrap();
    for k in my_keys {
      println!("k: {:?}", k);
    }
  }
}






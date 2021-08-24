
use lazy_static::*;
use std::collections::HashMap;
use std::env;

lazy_static! {
  pub static ref CONFIG: HashMap<&'static str, &'static str> = {
    let mut config = HashMap::new();

    let cmd_args: Vec<_> = env::args().collect();
    let mut run_env = "";
    if cmd_args.len() > 1 {
      run_env = &cmd_args[1];
    } else {
      run_env = "test"
    }

    run_env = "test";   // just for test
    match run_env {
      "dev" => {
        config.insert("consul_addr", "127.0.0.1");
        config.insert("consul_port", "8500");
      },
      "test" => {
        config.insert("consul_addr", "8.8.8.8");
        config.insert("consul_port", "8500");
      },
      "prd" => {
        config.insert("consul_addr", "8.8.8.8");
        config.insert("consul_port", "8500");
      },
      _ => {},
    }

    println!("runtime config ----- {}: {:?}", run_env, config);
    config
  };
}

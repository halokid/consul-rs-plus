use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct CustomError (pub String);

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "CustomErr => {}", self.0)
  }
}

pub struct Pkg{
  debug:    bool,         // set the debug model
}

impl Pkg {
  pub const fn new(debug: bool) -> Self {
    Pkg{
      debug: debug,
    }
  }

  pub fn debug_print(&self, s: &str) {
    if self.debug {
      println!("[DEBUG] --- {:?}", s);
    }
  }
}


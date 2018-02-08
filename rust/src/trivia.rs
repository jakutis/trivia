#[macro_use]
extern crate serde_derive;

pub mod game;

#[derive(Debug, Serialize, Deserialize)]
pub struct Execution {
  pub seed: usize,
  pub output: String,
}

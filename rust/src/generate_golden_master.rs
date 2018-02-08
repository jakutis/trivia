#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct Execution {
  seed: usize,
  output: String
}

fn main() {
  for seed in 0..10 {
    let output = Command::new("cargo")
      .args(&["run", "--bin", "trivia", &format!("{}", &seed)])
      .output()
      .unwrap()
      .stdout;

    let execution = Execution {
      seed,
      output: String::from_utf8(output).unwrap()
    };
    let json = serde_json::to_string(&execution).unwrap();

    println!("{}", json);
  }
}

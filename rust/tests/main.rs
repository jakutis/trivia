#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::process::Command;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Execution {
  seed: usize,
  output: String
}

#[test]
fn it_works() {
  let filename = "./tests/master.log";
  let f = File::open(filename).unwrap();
  let mut reader = BufReader::new(f);
  for line in reader.lines() {
    let execution = serde_json::from_str::<Execution>(&line.unwrap()).unwrap();
    let expected_output = execution.output;
    let stdout = Command::new("cargo")
      .args(&["run", "--bin", "trivia", &format!("{}", &execution.seed)])
      .output()
      .unwrap()
      .stdout;
    let actual_output = String::from_utf8(stdout).unwrap();

    print!("Seed is {}", execution.seed);
    assert_eq!(expected_output, actual_output);
  }
}

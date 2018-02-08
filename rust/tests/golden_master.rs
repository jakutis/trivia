extern crate serde_json;
extern crate trivia;

use std::process::Command;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use trivia::Execution;

#[test]
fn golden_master() {
  let filename = "./tests/master.log";
  let f = File::open(filename).unwrap();
  let reader = BufReader::new(f);
  for line in reader.lines() {
    let execution = serde_json::from_str::<Execution>(&line.unwrap()).unwrap();
    let expected_output = execution.output;
    let stdout = Command::new("target/debug/trivia")
      .arg(&format!("{}", &execution.seed))
      .output()
      .unwrap()
      .stdout;
    let actual_output = String::from_utf8(stdout).unwrap();

    print!("Seed is {}", execution.seed);
    assert_eq!(expected_output, actual_output);
  }
}

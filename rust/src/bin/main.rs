extern crate rand;
extern crate trivia;

use std::env;
use rand::{StdRng, SeedableRng, Rng};
use trivia::game::Game;

fn main() {
  let args: Vec<String> = env::args().collect();
  let seed = args[1].parse::<usize>().unwrap();
  let mut rng: StdRng = SeedableRng::from_seed(&[seed] as &[_]);
  let mut not_a_winner: bool;
  let mut game = Game::default(Box::new(|line: String| {println!("{}", line);}));
  game.add("Chet".to_string());
  game.add("Pat".to_string());
  game.add("Sue".to_string());
  while {
    game.roll(rng.gen::<i32>() % 5 + 1);
    if rng.gen::<i32>() % 9 == 7 {
      not_a_winner = game.wrong_answer();
    } else {
      not_a_winner = game.was_correctly_answered();
    }
    not_a_winner != false
  } {}
}

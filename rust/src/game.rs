pub struct Game {
  logg: Box<Fn(String) -> ()>,
  players: Vec<String>,
  places: [i32; 6],
  purses: [i32; 6],
  in_penaltybox: [bool; 6],
  current_player: i32,
  is_getting_out_of_penaltybox: bool,

  pop_questions: Vec<String>,
  science_questions: Vec<String>,
  sports_questions: Vec<String>,
  rock_questions: Vec<String>,
}

impl Game {
  pub fn default(log: Box<Fn(String) -> ()>) -> Game {
    let mut game = Game {
      logg: log,
      players: vec![],
      places: [0; 6],
      purses: [0; 6],
      in_penaltybox: [false; 6],
      current_player: 0,
      is_getting_out_of_penaltybox: false,
      pop_questions: Vec::new(),
      science_questions: Vec::new(),
      sports_questions: Vec::new(),
      rock_questions: Vec::new(),
    };
    for x in 0..50 {
      let pop_qu = "Pop Question ".to_string() + &x.to_string();
      game.pop_questions.push(pop_qu);
      let sci_qu = "Science Question ".to_string() + &x.to_string();
      game.science_questions.push(sci_qu);
      let spo_qu = "Sports Question ".to_string() + &x.to_string();
      game.sports_questions.push(spo_qu);
      let rock_qu = game.create_rock_question(x);
      game.rock_questions.push(rock_qu);
    }
    game
  }
}

impl Game {
  fn log(&self, line: String) {
    (*self.logg)(line);
  }

  fn how_many_players(&self) -> usize {
    self.players.len()
  }

  fn did_player_win(&self) -> bool {
    !(self.purses[self.current_player as usize] == 6)
  }

  fn current_category(&self) -> &'static str {
    match self.places[self.current_player as usize] {
      0 | 4 | 8 => "Pop",
      1 | 5 | 9 => "Science",
      2 | 6 | 10 => "Sports",
      _ => "Rock"
    }
  }

  fn create_rock_question(&self, index: i32) -> String {
    "Rock Question ".to_string() + &index.to_string()
  }

  pub fn add(&mut self, player_name: String) -> bool {
    let l_player = player_name.clone();
    self.players.push(player_name);
    self.places[self.how_many_players()] = 0;
    self.purses[self.how_many_players()] = 0;
    self.in_penaltybox[self.how_many_players()] = false;
    self.log(format!("{} was added", l_player));
    self.log(format!("They are player number {}", self.players.len()));
    true
  }

  pub fn wrong_answer(&mut self) -> bool {
    self.log(format!("Question was incorrectly answered"));
    self.log(format!("{} was sent to the penalty box", self.players[self.current_player as usize]));
    self.in_penaltybox[self.current_player as usize] = true;
    self.current_player += 1;
    if self.current_player == self.players.len() as i32 {
      self.current_player = 0;
    }
    true
  }
}

impl Game {
  fn ask_question(&mut self) {
    match self.current_category() {
      "Pop" => {
        let top = self.pop_questions.pop();
        self.log(format!("{:?}", top.unwrap()));
      }
      "Science" => {
        let top = self.science_questions.pop();
        self.log(format!("{:?}", top.unwrap()));
      }
      "Sports" => {
        let top = self.sports_questions.pop();
        self.log(format!("{:?}", top.unwrap()));
      }
      "Rock" => {
        let top = self.rock_questions.pop();
        self.log(format!("{:?}", top.unwrap()));
      }
      _ => {
        self.log(format!("Unexpected case"));
      }
    }
  }
}

impl Game {
  pub fn roll(&mut self, roll: i32) {
    self.log(format!("{} is current player", self.players[self.current_player as usize]));
    self.log(format!("They have rolled a {}", roll));
    if self.in_penaltybox[self.current_player as usize] {
      if roll % 2 != 0 {
        self.is_getting_out_of_penaltybox = true;
        self.log(format!("{} is getting out of the penalty box", self.players[self.current_player as usize]));
        self.places[self.current_player as usize] += roll;
        if self.places[self.current_player as usize] > 11 {
          self.places[self.current_player as usize] -= 12;
        }
        self.log(format!("{0} 's new location is {1}", self.players[self.current_player as usize], self.places[self.current_player as usize]));
        self.log(format!("The category is {}", self.current_category()));
        self.ask_question();
      } else {
        self.log(format!("{} is not getting out of the penalty box", self.players[self.current_player as usize]));
        self.is_getting_out_of_penaltybox = false;
      }
    } else {
      self.places[self.current_player as usize] += roll;
      if self.places[self.current_player as usize] > 11 {
        self.places[self.current_player as usize] -= 12;
      }
      self.log(format!("{0} 's new location is {1}", self.players[self.current_player as usize], self.places[self.current_player as usize]));
      self.log(format!("The category is {}", self.current_category()));
      self.ask_question();
    }
  }
}

impl Game {
  pub fn was_correctly_answered(&mut self) -> bool {
    if self.in_penaltybox[self.current_player as usize] {
      if self.is_getting_out_of_penaltybox {
        self.log(format!("Answer was correct!!!!"));
        self.purses[self.current_player as usize] += 1;
        self.log(format!("{0} now has {1} Gold Coins.", self.players[self.current_player as usize], self.purses[self.current_player as usize]));
        let winner: bool = self.did_player_win();
        self.current_player += 1;
        if self.current_player == self.players.len() as i32 {
          self.current_player = 0;
        }
        winner
      } else {
        self.current_player += 1;
        if self.current_player == self.players.len() as i32 {
          self.current_player = 0;
        }
        true
      }
    } else {
      self.log(format!("Answer was correct!!!!"));
      self.purses[self.current_player as usize] += 1;
      self.log(format!("{0} now has {1} Gold Coins.", self.players[self.current_player as usize], self.purses[self.current_player as usize]));
      let winner: bool = self.did_player_win();
      self.current_player += 1;
      if self.current_player == self.players.len() as i32 {
        self.current_player = 0;
      }
      winner
    }
  }
}

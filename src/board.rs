use food::Food;
use snake::Snake;

#[derive(Debug)]
pub struct Board {
  pub width: u32,
  pub height: u32,
  food: Food,
  snake: Snake,
  game_is_running: bool
}

impl Board {
  pub fn new(width: u32, height: u32, food: Food, snake: Snake) -> Board {
      Board { width, height, food, snake, game_is_running: true }
  }

  pub fn set_snake(self, snake: Snake) -> Board {
      Board {
          snake,
          ..self
      }
  }

  pub fn get_snake(&self) -> &Snake {
      &self.snake
  }

  pub fn set_food(self, food: Food) -> Board {
      Board {
          food,
          ..self
      }
  }

  pub fn get_food(&self) -> &Food {
      &self.food
  }

  pub fn set_game_is_running(self, game_is_running: bool) -> Board {
      Board {
          game_is_running,
          ..self
      }
  }

  pub fn is_game_running(&self) -> bool {
      self.game_is_running
  }
}

// TODO add tests

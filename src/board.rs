use food::Food;
use snake::Snake;

#[derive(Debug)]
pub struct Board {
  pub width: u32,
  pub height: u32,
  food: Food,
  snake: Snake,
}

impl Board {
  pub fn new(width: u32, height: u32, food: Food, snake: Snake) -> Board {
      Board { width, height, food, snake }
  }

  pub fn set_snake(&mut self, snake: Snake) {
      self.snake = snake;
  }

  pub fn get_snake(&self) -> &Snake {
      &self.snake
  }

  pub fn set_food(&mut self, food: Food) {
      self.food = food;
  }
}

// TODO add tests

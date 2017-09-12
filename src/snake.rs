use direction::Direction;

#[derive(Debug, PartialEq)]
pub struct Snake {
  pub x: f64,
  pub y: f64,
  pub direction: Direction
}

const SNAKE_STEP: f64 = 0.2;

impl Snake {
  pub fn new(x: f64, y: f64, direction: Direction) -> Snake {
    Snake { x, y, direction }
  }

  pub fn advance(&self) -> Snake {
      let (x, y) = match self.direction {
        Direction::Up    => (self.x, self.y - SNAKE_STEP),
        Direction::Down  => (self.x, self.y + SNAKE_STEP),
        Direction::Left  => (self.x - SNAKE_STEP, self.y),
        Direction::Right => (self.x + SNAKE_STEP, self.y),
      };
      Snake::new(x, y, self.direction)
  }

  pub fn change_direction(&self, direction: Direction) -> Snake {
    let direction = if self.direction.get_opposite_direction() == direction {
      // can't change direction to opposite
      self.direction
    } else {
      direction
    };
    Snake::new(self.x, self.y, direction)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn should_create_new_snake() {
      let x = 5.0;
      let y = 3.5;
      let direction = Direction::Right;

      let expected = Snake { x, y, direction};
      let actual = Snake::new(x, y, direction);

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_decrease_on_y_axis_when_advance_up() {
      let x = 5.0;
      let y = 3.5;
      let direction = Direction::Up;
      let snake = Snake::new(x, y, direction);

      let expected = Snake { x, y: y - SNAKE_STEP, direction };
      let actual = snake.advance();

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_advance_on_y_axis_when_advance_down() {
      let x = 5.0;
      let y = 3.5;
      let direction = Direction::Down;
      let snake = Snake::new(x, y, direction);

      let expected = Snake { x, y: y + SNAKE_STEP, direction };
      let actual = snake.advance();

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_change_direction_from_left_to_up() {
      let x = SNAKE_STEP;
      let y = SNAKE_STEP;
      let direction = Direction::Left;
      let snake = Snake::new(x, y, direction);

      let expected = Snake { x, y, direction: Direction::Up };
      let actual = snake.change_direction(Direction::Up);

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_not_change_direction_if_new_direction_is_opposite_to_current() {
      let x = SNAKE_STEP;
      let y = SNAKE_STEP;
      let direction = Direction::Left;
      let snake = Snake::new(x, y, direction);

      let expected = Snake { x, y, direction: Direction::Left };
      let actual = snake.change_direction(Direction::Right);

      assert_eq!(expected, actual);
  }
}

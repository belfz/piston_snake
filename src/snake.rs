use direction::Direction;

#[derive(Debug, PartialEq)]
pub struct Snake {
  pub segments: Vec<(i32, i32)>,
  pub direction: Direction
}

pub const SNAKE_STEP: u32 = 10;
pub const SNAKE_SEGMENT_WIDTH: u32 = 10;
pub const START_SEGMENTS: u32 = 3;

impl Snake {
  pub fn new(x: i32, y: i32, direction: Direction) -> Snake {
    let mut segments = vec![];
    for i in 0..START_SEGMENTS {
      segments.push((x - (SNAKE_SEGMENT_WIDTH * i) as i32, y));
    }
    Snake { direction, segments }
  }

  pub fn advance(&self) -> Snake {
      let mut segments = self.segments.clone();
      segments.pop();
      let &(x, y) = segments.first().unwrap();
      let (head_x, head_y) = match self.direction {
        Direction::Up    => (x, y - SNAKE_STEP as i32),
        Direction::Down  => (x, y + SNAKE_STEP as i32),
        Direction::Left  => (x - SNAKE_STEP as i32, y),
        Direction::Right => (x + SNAKE_STEP as i32, y)
      };
      segments.insert(0, (head_x, head_y));
      Snake { segments, direction: self.direction }
  }

  pub fn change_direction(&self, direction: Direction) -> Snake {
    let direction = if self.direction.get_opposite_direction() == direction {
      // can't change direction to opposite
      self.direction
    } else {
      direction
    };

    Snake { segments: self.segments.clone(), direction }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn should_create_new_snake() {
      let x = 25;
      let y = 23;
      let direction = Direction::Right;

      let segments = vec![(x, y), (x - SNAKE_SEGMENT_WIDTH, y), (x - SNAKE_SEGMENT_WIDTH * 2, y)];
      let expected = Snake { segments, direction};
      let actual = Snake::new(x, y, direction);

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_decrease_new_segment_on_y_axis_when_advance_up() {
      let x = 25;
      let y = 23;
      let direction = Direction::Up;
      let snake = Snake::new(x, y, direction);

      let segments = vec![(x, y - SNAKE_STEP), (x, y), (x - SNAKE_SEGMENT_WIDTH, y)];
      let expected = Snake { segments, direction };
      let actual = snake.advance();

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_increase_new_segment_on_y_axis_when_advance_down() {
      let x = 25;
      let y = 23;
      let direction = Direction::Down;
      let snake = Snake::new(x, y, direction);

      let segments = vec![(x, y + SNAKE_STEP), (x, y), (x - SNAKE_SEGMENT_WIDTH, y)];
      let expected = Snake { segments , direction };
      let actual = snake.advance();

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_change_direction_from_left_to_up() {
      let x = 25;
      let y = 23;
      let snake = Snake::new(x, y, Direction::Left);

      let segments = vec![(x, y), (x - SNAKE_SEGMENT_WIDTH, y), (x - SNAKE_SEGMENT_WIDTH * 2, y)];
      let expected = Snake { segments, direction: Direction::Up };
      let actual = snake.change_direction(Direction::Up);

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_not_change_direction_if_new_direction_is_opposite_to_current() {
      let x = 25;
      let y = 23;
      let snake = Snake::new(x, y, Direction::Left);

      let segments = vec![(x, y), (x - SNAKE_SEGMENT_WIDTH, y), (x - SNAKE_SEGMENT_WIDTH * 2, y)];
      let expected = Snake { segments, direction: Direction::Left };
      let actual = snake.change_direction(Direction::Right);

      assert_eq!(expected, actual);
  }
}

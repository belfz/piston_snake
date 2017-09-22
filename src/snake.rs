use direction::Direction;
use food::Food;

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

  fn add_segment(&self, mut segments: Vec<(i32, i32)>, board_width: i32, board_height: i32, head_x: i32, head_y: i32) -> Snake {
      let (new_x, new_y) = match self.direction {
        Direction::Up    => (head_x, head_y - SNAKE_STEP as i32),
        Direction::Down  => (head_x, head_y + SNAKE_STEP as i32),
        Direction::Left  => (head_x - SNAKE_STEP as i32, head_y),
        Direction::Right => (head_x + SNAKE_STEP as i32, head_y)
      };
      segments.insert(0, (new_x, new_y));
      segments = segments
        .iter()
        .map(|&(x, y)| {
          let x = if x < 0 { board_width } else if x > board_width { 0 } else { x };
          let y = if y < 0 { board_height } else if y > board_height { 0 } else { y };
          (x, y)
        })
        .collect();
      Snake { segments, ..*self }   
  }

  pub fn advance(&self, board_width: i32, board_height: i32) -> Snake {
      let mut segments = self.segments.clone();
      segments.pop();
      let &(x, y) = segments.first().unwrap();
      self.add_segment(segments, board_width, board_height, x, y)
  }

  pub fn eat_food(&self, food: &Food, board_width: i32, board_height: i32) -> Snake {
    let segments = self.segments.clone();
    self.add_segment(segments, board_width, board_height, food.x as i32, food.y as i32)
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

  pub fn has_collision(&self) -> bool {
    let (head_x, head_y) = self.segments[0];
    self.segments.iter().skip(1).any(|&(x, y)| x == head_x && y == head_y)
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

      let segments = vec![(x, y), (x - SNAKE_SEGMENT_WIDTH as i32, y), (x - SNAKE_SEGMENT_WIDTH as i32 * 2, y)];
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

      let segments = vec![(x, y - SNAKE_STEP as i32), (x, y), (x - SNAKE_SEGMENT_WIDTH as i32, y)];
      let expected = Snake { segments, direction };
      let actual = snake.advance(160, 160);

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_increase_new_segment_on_y_axis_when_advance_down() {
      let x = 25;
      let y = 23;
      let direction = Direction::Down;
      let snake = Snake::new(x, y, direction);

      let segments = vec![(x, y + SNAKE_STEP as i32), (x, y), (x - SNAKE_SEGMENT_WIDTH as i32, y)];
      let expected = Snake { segments , direction };
      let actual = snake.advance(160, 160);

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_flip_to_other_side_when_advance_down_over_the_boundary() {
      let x = 30;
      let y = 30;
      let direction = Direction::Down;
      let snake = Snake::new(x, y, direction);

      let segments = vec![(x, 0), (x, y), (x - SNAKE_SEGMENT_WIDTH as i32, y)];
      let expected = Snake { segments , direction };
      let actual = snake.advance(30, 30);

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_flip_to_other_side_when_advance_up_over_the_boundary() {
      let x = 30;
      let y = 0;
      let direction = Direction::Up;
      let snake = Snake::new(x, y, direction);

      let segments = vec![(x, 30), (x, y), (x - SNAKE_SEGMENT_WIDTH as i32, y)];
      let expected = Snake { segments , direction };
      let actual = snake.advance(30, 30);

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_flip_to_other_side_when_advance_right_over_the_boundary() {
      let x = 30;
      let y = 20;
      let direction = Direction::Right;
      let snake = Snake::new(x, y, direction);

      let segments = vec![(0, y), (x, y), (x - SNAKE_SEGMENT_WIDTH as i32, y)];
      let expected = Snake { segments , direction };
      let actual = snake.advance(30, 30);

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_flip_to_other_side_when_advance_left_over_the_boundary() {
      let x = 0;
      let y = 20;
      let direction = Direction::Left;
      let snake = Snake { segments: vec![(x, y), (x + SNAKE_SEGMENT_WIDTH as i32, y), (x + 2 * SNAKE_SEGMENT_WIDTH as i32, y)], direction };

      let segments = vec![(30, y), (0, y), (10, y)];
      let expected = Snake { segments , direction };
      let actual = snake.advance(30, 30);

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_change_direction_from_left_to_up() {
      let x = 25;
      let y = 23;
      let snake = Snake::new(x, y, Direction::Left);

      let segments = vec![(x, y), (x - SNAKE_SEGMENT_WIDTH as i32, y), (x - SNAKE_SEGMENT_WIDTH as i32 * 2, y)];
      let expected = Snake { segments, direction: Direction::Up };
      let actual = snake.change_direction(Direction::Up);

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_not_change_direction_if_new_direction_is_opposite_to_current() {
      let x = 25;
      let y = 23;
      let snake = Snake::new(x, y, Direction::Left);

      let segments = vec![(x, y), (x - SNAKE_SEGMENT_WIDTH as i32, y), (x - SNAKE_SEGMENT_WIDTH as i32 * 2, y)];
      let expected = Snake { segments, direction: Direction::Left };
      let actual = snake.change_direction(Direction::Right);

      assert_eq!(expected, actual);
  }

  #[test]
  fn eat_food_should_add_head_and_advance() {
      let x = 25;
      let y = 23;
      let food = Food { x: x as u32, y: y as u32 };
      let snake = Snake::new(x, y, Direction::Right);

      let segments = vec![(x + SNAKE_SEGMENT_WIDTH as i32, y), (x, y), (x - SNAKE_SEGMENT_WIDTH as i32, y), (x - SNAKE_SEGMENT_WIDTH as i32 * 2, y)];
      let expected = Snake { segments, direction: Direction::Right };
      let actual = snake.eat_food(&food, 50, 50);

      assert_eq!(expected, actual);
  }

  #[test]
  fn has_collision_should_return_true_when_head_collides_with_at_least_one_segment() {
      let segments = vec![(1, 1), (2, 1), (3, 1), (1, 1), (4, 1)];
      let snake = Snake { segments, direction: Direction::Left };

      assert!(snake.has_collision());
  }

  #[test]
  fn has_collision_should_return_false_when_head_does_not_collide() {
      let segments = vec![(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)];
      let snake = Snake { segments, direction: Direction::Left };

      assert!(!snake.has_collision());
  }
}

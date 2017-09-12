#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right
}

impl Direction {
  pub fn get_opposite_direction(self) -> Direction {
    use self::Direction::*;
    match self {
      Up    => Down,
      Down  => Up,
      Left  => Right,
      Right => Left,
    }    
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn should_return_down_as_opposite_to_up() {
      let up = Direction::Up;
      
      let expected = Direction::Down;
      let actual = up.get_opposite_direction();

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_return_up_as_opposite_to_down() {
      let up = Direction::Down;
      
      let expected = Direction::Up;
      let actual = up.get_opposite_direction();

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_return_left_as_opposite_to_right() {
      let up = Direction::Right;
      
      let expected = Direction::Left;
      let actual = up.get_opposite_direction();

      assert_eq!(expected, actual);
  }

  #[test]
  fn should_return_right_as_opposite_to_left() {
      let up = Direction::Left;
      
      let expected = Direction::Right;
      let actual = up.get_opposite_direction();

      assert_eq!(expected, actual);
  }
}

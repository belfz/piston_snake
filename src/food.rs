
use rand::distributions::{IndependentSample, Range};
use rand::thread_rng;

#[derive(Clone, Debug)]
pub struct Food {
  pub x: u32,
  pub y: u32,
}

fn round_to_tens(num: u32) -> u32 {
    (num / 10) * 10
}

impl Food {
  pub fn next_rand_food(upper_bound_x: u32, upper_bound_y: u32) -> Food {
    let between_x = Range::new(0, upper_bound_x);
    let between_y = Range::new(0, upper_bound_y);
    let mut rng = thread_rng();
    let x = round_to_tens(between_x.ind_sample(&mut rng));
    let y = round_to_tens(between_y.ind_sample(&mut rng));
    Food { x, y }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn next_rand_food_should_return_food_with_position_within_bounds() {
    let upper_bound_x = 50;
    let upper_bound_y = 30;

    let food = Food::next_rand_food(upper_bound_x, upper_bound_y);

    assert!(food.x <= upper_bound_x);
    assert!(food.y <= upper_bound_y);
  }
}

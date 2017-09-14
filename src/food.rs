
use rand::distributions::{IndependentSample, Range};
use rand::thread_rng;

#[derive(Debug)]
pub struct Food {
  pub x: u32,
  pub y: u32,
}

impl Food {
  pub fn next_rand_food(upper_bound_x: u32, upper_bound_y: u32) -> Food {
    let between_x = Range::new(0, upper_bound_x);
    let between_y = Range::new(0, upper_bound_y);
    let mut rng = thread_rng();
    let x = between_x.ind_sample(&mut rng);
    let y = between_y.ind_sample(&mut rng);
    Food { x, y }
  }
}

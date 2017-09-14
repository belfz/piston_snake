extern crate piston_window;
extern crate rand;

mod board;
mod direction;
mod food;
mod snake;

use board::Board;
use direction::Direction;
use food::Food;
use snake::Snake;

use piston_window::*;
use std::time::Instant;

const ONE_HUNDRED_MS: u32 = 100000000;
static TITLE: &'static str = "Rusnake!";

fn render(window: &mut PistonWindow, event: &piston_window::Event, game_board: &Board) {
  window.draw_2d(event, |context, graphics| {
    let &Snake { ref segments, .. } = game_board.get_snake();
    let &Food { x, y } = game_board.get_food();
    clear([1.0; 4], graphics);

    // render each segment of the snake
    for &(x, y) in segments.iter() {
      rectangle([0.0, 0.0, 0.0, 1.0],
                [x, y, snake::SNAKE_SEGMENT_WIDTH, snake::SNAKE_SEGMENT_WIDTH],
                context.transform,
                graphics);
    }

    // render food
    rectangle([0.0, 0.0, 0.0, 1.0],
              [x as f64, y as f64, snake::SNAKE_SEGMENT_WIDTH, snake::SNAKE_SEGMENT_WIDTH],
              context.transform,
              graphics);
  });
}

pub fn run(width: u32, height: u32) {
  let mut last_position_update_timestamp = Instant::now();
  let mut next_direction = Direction::Right;
  let mut game_board = Board::new(width, height, Food::next_rand_food(width, height), Snake::new(50.0, 50.0, next_direction));
  let mut window: PistonWindow = WindowSettings::new(TITLE, [game_board.width, game_board.height])
    .exit_on_esc(true).build().unwrap();

  while let Some(event) = window.next() {
    if let Some(Button::Keyboard(key)) = event.press_args() {
        next_direction = match key {
            Key::Right => Direction::Right,
            Key::Left  => Direction::Left,
            Key::Up    => Direction::Up,
            Key::Down  => Direction::Down,
            _          => next_direction,
        };
    }

    render(&mut window, &event, &game_board);

    if Instant::now().duration_since(last_position_update_timestamp).subsec_nanos() > ONE_HUNDRED_MS {
      let mut snake = game_board.get_snake().advance();
      snake = snake.change_direction(next_direction);
      game_board.set_snake(snake);
      last_position_update_timestamp = Instant::now();
    } else {
      continue;
    }

    // TODO consume food logic
    // TODO game_board.set_food(food);
  }
}

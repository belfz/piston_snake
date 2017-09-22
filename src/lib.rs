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
    let width = snake::SNAKE_SEGMENT_WIDTH as f64;
    clear([1.0; 4], graphics);

    // render each segment of the snake
    for &(x, y) in segments.iter() {
      rectangle([0.0, 0.0, 0.0, 1.0],
                [x as f64, y as f64, width, width],
                context.transform,
                graphics);
    }

    // render food
    rectangle([0.0, 0.0, 0.0, 1.0],
              [x as f64, y as f64, width, width],
              context.transform,
              graphics);
  });
}

pub fn run(width: u32, height: u32) {
  let mut last_position_update_timestamp = Instant::now();
  let mut next_direction = Direction::Right;
  let mut game_board = Board::new(width, height, Food::next_rand_food(width, height), Snake::new(50, 50, next_direction));
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

    if Instant::now().duration_since(last_position_update_timestamp).subsec_nanos() > ONE_HUNDRED_MS && game_board.is_game_running() {
      let mut snake = game_board.get_snake().advance(game_board.width as i32, game_board.height as i32);
      if snake.has_collision() {
        println!("collision!");
        game_board = game_board.set_game_is_running(false);
        continue;
      }
      snake = snake.change_direction(next_direction);

      let (snake_head_x, snake_head_y) = game_board.get_snake().segments[0];
      if snake_head_x == game_board.get_food().x as i32 && snake_head_y == game_board.get_food().y as i32 {
        snake = snake.eat_food(game_board.get_food(), game_board.width as i32, game_board.height as i32);

        let new_food = Food::next_rand_food(width, height);
        game_board = game_board.set_food(new_food);
      }
      
      game_board = game_board.set_snake(snake);
      last_position_update_timestamp = Instant::now();
    } else {
      continue;
    }
  }
}

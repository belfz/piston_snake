extern crate piston_window;

mod board;
mod direction;
mod food;
mod snake;

use board::Board;
use direction::Direction;
use food::Food;
use snake::Snake;

use piston_window::*;

fn render(window: &mut PistonWindow, event: &piston_window::Event, game_board: &Board) {
  window.draw_2d(event, |context, graphics| {
    let &Snake { x, y, .. } = game_board.get_snake();
    clear([1.0; 4], graphics);

    // render snake
    rectangle([0.0, 0.0, 0.0, 1.0],
              [x, y, 10.0, 10.0],
              context.transform,
              graphics);

    // TODO render food
  });
}

pub fn run(width: u32, height: u32) {
  let mut game_board = Board::new(width, height, Food, Snake::new(50.0, 50.0, Direction::Right));
  let mut window: PistonWindow = WindowSettings::new("Rusnake!", [game_board.width, game_board.height])
    .exit_on_esc(true).build().unwrap();

  while let Some(event) = window.next() {
    render(&mut window, &event, &game_board);

    let mut snake = game_board.get_snake().advance();
    if let Some(Button::Keyboard(key)) = event.press_args() {
        snake = match key {
            Key::Right => snake.change_direction(Direction::Right),
            Key::Left  => snake.change_direction(Direction::Left),
            Key::Up    => snake.change_direction(Direction::Up),
            Key::Down  => snake.change_direction(Direction::Down),
            _          => snake,
        };
    }

    // TODO consume food logic
    // TODO game_board.set_food(food);

    game_board.set_snake(snake);
  }
}

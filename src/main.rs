mod constants;
mod snake;
mod window;

use constants::*;
use macroquad::prelude::*;
use snake::*;
use window::config;

#[macroquad::main(config)]
async fn main() {
    let mut snake = Snake::default();
    let mut game_over = false;
    let mut last_time = get_time();

    loop {
        if game_over {
            clear_background(S_GREEN);
            let game_over_text = "Game over. Press { SPACE } to restart.";
            let measure = measure_text(game_over_text, None, 32, 1.0);
            draw_text(
                game_over_text,
                (screen_width() - measure.width) / 2.0,
                (screen_height() - measure.height) / 2.0,
                32.0,
                BLACK,
            );

            if is_key_down(KeyCode::Space) {
                snake = Snake::default();
                game_over = false;
            }
            next_frame().await
        } else {
            let delta_time = get_time() - last_time;

            clear_background(S_GREEN);

            if is_key_down(KeyCode::Right) {
                snake.turn_right();
            }
            if is_key_down(KeyCode::Left) {
                snake.turn_left();
            }
            if is_key_down(KeyCode::Down) {
                snake.turn_down();
            }
            if is_key_down(KeyCode::Up) {
                snake.turn_up();
            }

            if delta_time > (10.0 / (snake.velocity + 10.0)) as f64 {
                last_time = get_time();
                if !snake.move_snake() {
                    game_over = true;
                }
            }

            draw_snake(&snake);
            next_frame().await
        }
    }
}

fn draw_snake(snake: &snake::Snake) {
    let pixel_width = screen_width() / GRID_SIZE as f32;
    let pixel_height = screen_height() / GRID_SIZE as f32;

    draw_rectangle(
        snake.head.x as f32 * pixel_width,
        snake.head.y as f32 * pixel_height,
        pixel_width,
        pixel_height,
        BLACK,
    );
}

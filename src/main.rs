mod constants;
mod snake;
mod window;

use constants::*;
use macroquad::prelude::*;
use window::config;

static GRID_SIZE: u8 = 30;

#[macroquad::main(config)]
async fn main() {
    let mut snake = snake::Snake::default();
    let mut last_time = get_time();

    loop {
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

        let time = format!("{}", delta_time);
        draw_text(&time, 10.0, screen_height() - 100.0, 26.0, BLACK);

        if delta_time > 1.0 {
            last_time = get_time();
            snake.move_snake();
        }

        draw_snake(&snake);
        draw_debug();
        next_frame().await
    }
}

fn draw_debug() {
    let window_size = format!("Window size: {}, {}", screen_width(), screen_height());
    let fps = format!("FPS: {}", get_fps());
    let time = format!("{}", get_time());

    draw_text(&time, 10.0, screen_height() - 70.0, 26.0, BLACK);
    draw_text(&window_size, 10.0, screen_height() - 50.0, 26.0, BLACK);
    draw_text(&fps, 10.0, screen_height() - 20.0, 26.0, BLACK);
}

fn draw_snake(snake: &snake::Snake) {
    let pixel_width = screen_width() / GRID_SIZE as f32;
    let pixel_height = screen_height() / GRID_SIZE as f32;

    draw_rectangle(
        snake.head.x as f32 * 100.0,
        snake.head.y as f32 * 100.0,
        pixel_width,
        pixel_height,
        BLACK,
    );
}

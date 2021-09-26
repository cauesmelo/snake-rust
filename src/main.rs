use macroquad::prelude::*;
mod colors;
mod snake;

static GRID_SIZE: i16 = 30;

#[macroquad::main("Snake")]
async fn main() {
    let snake = snake::Snake { head: (0, 0) };
    let pixel_size: f32 = 10.0;

    loop {
        clear_background(colors::GREEN);

        draw_rectangle(
            snake.head.0 as f32 * pixel_size,
            snake.head.1 as f32 * pixel_size,
            20.0,
            20.0,
            colors::BLACK,
        );

        draw_debug();

        next_frame().await
    }
}

fn draw_debug() {
    let window_size = format!("Window size: {}, {}", screen_width(), screen_height());
    let fps = format!("FPS: {}", get_fps());

    draw_text(&window_size, 10.0, screen_height() - 50.0, 26.0, BLACK);
    draw_text(&fps, 10.0, screen_height() - 20.0, 26.0, BLACK);
}

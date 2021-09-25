use macroquad::prelude::*;

type Point = (i16, i16);

struct Snake {
    pos: Point,
}

#[macroquad::main("Snake")]
async fn main() {
    let mut snake = Snake { pos: (0, 0) };

    loop {
        clear_background(GREEN);

        draw_rectangle(0.0, 0.0, 20.0, 20.0, BLACK);

        next_frame().await
    }
}

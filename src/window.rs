use macroquad::prelude::*;

pub fn config() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        fullscreen: false,
        window_height: 600,
        window_width: 600,
        window_resizable: false,
        ..Default::default()
    }
}

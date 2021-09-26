use std::collections::LinkedList;

pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Snake {
    pub head: Point,
    direction: Direction,
    body: LinkedList<Point>,
}

impl Snake {
    pub fn turn_right(&mut self) {
        self.direction = Direction::Right
    }

    pub fn turn_left(&mut self) {
        self.direction = Direction::Left
    }

    pub fn turn_down(&mut self) {
        self.direction = Direction::Down
    }

    pub fn turn_up(&mut self) {
        self.direction = Direction::Up
    }

    pub fn move_snake(&mut self) {
        match self.direction {
            Direction::Right => {
                self.head.x += 1;
            }

            Direction::Left => {
                self.head.x -= 1;
            }

            Direction::Down => {
                self.head.y += 1;
            }

            Direction::Up => {
                self.head.y -= 1;
            }
        }
    }
}

impl Default for Snake {
    fn default() -> Snake {
        Snake {
            head: Point { x: 0, y: 0 },
            direction: Direction::Right,
            body: LinkedList::new(),
        }
    }
}

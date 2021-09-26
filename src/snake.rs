// use std::collections::LinkedList;

use crate::constants::GRID_SIZE;

pub struct Point {
    pub x: u8,
    pub y: u8,
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
    pub velocity: f32,
    // body: LinkedList<Point>,
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

    pub fn move_snake(&mut self) -> bool {
        match self.direction {
            Direction::Right => {
                if self.head.x == GRID_SIZE - 1 {
                    false
                } else {
                    self.head.x += 1;
                    true
                }
            }

            Direction::Left => {
                if self.head.x == 0 {
                    false
                } else {
                    self.head.x -= 1;
                    true
                }
            }

            Direction::Down => {
                if self.head.y == GRID_SIZE - 1 {
                    false
                } else {
                    self.head.y += 1;
                    true
                }
            }

            Direction::Up => {
                if self.head.y == 0 {
                    false
                } else {
                    self.head.y -= 1;
                    true
                }
            }
        }
    }
}

impl Default for Snake {
    fn default() -> Snake {
        Snake {
            head: Point { x: 0, y: 0 },
            direction: Direction::Right,
            // body: LinkedList::new(),
            velocity: 10.0,
        }
    }
}

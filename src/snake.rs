use std::{collections::VecDeque, u16};

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub type SnakeScore = u32;

pub struct SnakeState {
    pub direction: Direction,
    pub score: SnakeScore,
    pub body: VecDeque<Point>,
}

fn new_direction(old: Direction, new: Direction) -> Option<Direction> {
    match (old, new) {
        (Direction::Up, Direction::Left) => Some(Direction::Left),
        (Direction::Up, Direction::Right) => Some(Direction::Right),
        (Direction::Down, Direction::Left) => Some(Direction::Left),
        (Direction::Down, Direction::Right) => Some(Direction::Right),
        (Direction::Left, Direction::Up) => Some(Direction::Up),
        (Direction::Left, Direction::Down) => Some(Direction::Down),
        (Direction::Right, Direction::Up) => Some(Direction::Up),
        (Direction::Right, Direction::Down) => Some(Direction::Down),
        _ => None,
    }
}

impl SnakeState {
    pub fn new(x: u16, y: u16) -> Self {
        let center = Point { x: x / 2, y: y / 2 };
        // let mut init_body: VecDeque<Point> = VecDeque::with_capacity(10);
        let mut init_body: VecDeque<Point> = [
            Point {
                x: center.x,
                y: center.y,
            },
            Point {
                x: center.x + 1,
                y: center.y,
            },
            Point {
                x: center.x + 2,
                y: center.y,
            },
        ]
        .into();
        init_body.reserve(20);
        // init_body.push_back(Point{x: center.x, y: center.y});
        // init_body.push_back(Point{x: center.x+1, y:center.y});
        // init_body.push_back(Point{x: center.x+2, y:center.y});
        Self {
            direction: Direction::Left,
            score: 0,
            body: init_body,
        }
    }


    pub fn change_direction(&mut self, direction: Direction) {
        if let Some(dir) = new_direction(self.direction, direction) {
            self.direction = dir;
        }

        // match new_direction(state.direction, direction) {
        //     Some(dir) => state.direction = dir,
        //     None => (),
        // }
    }

    // fn check_for_collision(&self) -> bool {
    //     let head = self.body.front().unwrap(); // TODO: handle this ??
    //     match self.direction {
    //         Direction::Up => head.y - 1 == 0,
    //         Direction::Down => head.y + 1 == Y_SIZE - 1,
    //         Direction::Left => head.x - 1 == 0,
    //         Direction::Right => head.x + 1 == X_SIZE - 1,
    //     }
    // }
}

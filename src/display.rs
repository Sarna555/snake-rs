use std::{io::stdout, time::Duration};

// use futures::{future::FutureExt, select, StreamExt};
// use futures_timer::Delay;

use crossterm::{
    cursor,
    event::{
        /* DisableMouseCapture, EnableMouseCapture, */ Event, EventStream, KeyCode, KeyEvent,
        KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::snake::{Point, SnakeScore, SnakeState};

pub trait Display {
    fn draw(&mut self, state: &SnakeState, tail: Point);
    fn start_game(&mut self);
    fn update_score(&mut self, score: SnakeScore);
}

pub struct TerminalDisplay {
    stdout: std::io::Stdout,
    max_x: u16,
    max_y: u16,
}

impl TerminalDisplay {
    pub fn new(max_x: u16, max_y: u16) -> TerminalDisplay {
        TerminalDisplay {
            stdout: stdout(),
            max_x,
            max_y,
        }
    }

    pub fn init(&mut self) -> std::io::Result<()> {
        enable_raw_mode()?;
        execute!(self.stdout, EnterAlternateScreen)
    }

    pub fn deinit(&mut self) -> std::io::Result<()> {
        execute!(self.stdout, LeaveAlternateScreen)?;
        disable_raw_mode()
    }
}
impl Display for TerminalDisplay {
    fn draw(&mut self, state: &SnakeState, tail: Point) {
        for p in state.body.iter() {
            execute!(self.stdout, cursor::MoveTo(p.x, p.y)).unwrap();
            print!("X");
        }
        execute!(self.stdout, cursor::MoveTo(tail.x, tail.y)).unwrap();
        print!(" ");
    }

    fn start_game(&mut self) {
        for x in 0..self.max_x {
            execute!(self.stdout, cursor::MoveTo(x, 0)).unwrap();
            print!("_");
        }
        for x in 1..self.max_x {
            execute!(self.stdout, cursor::MoveTo(x, self.max_y - 1)).unwrap();
            print!("-");
        }
        for y in 1..self.max_y {
            execute!(self.stdout, cursor::MoveTo(0, y)).unwrap();
            print!("|");
            execute!(self.stdout, cursor::MoveTo(self.max_x - 1, y)).unwrap();
            print!("|");
        }

        self.update_score(15000);
    }

    fn update_score(&mut self, score: SnakeScore) {
        let score_str = score.to_string();
        let chars_count = score_str.chars().count() as u16;
        execute!(self.stdout, cursor::MoveTo(self.max_x - 1 - chars_count, 0)).unwrap();
        print!("{}", score_str);
    }
}

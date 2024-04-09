//! Demonstrates how to read events asynchronously with tokio.
//!
//! cargo run --features="event-stream" --example event-stream-tokio

use std::{io::stdout, time::Duration};

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::event::{poll, read};
use crossterm::{
    cursor,
    event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::collections::VecDeque;

mod display;
mod snake;

use crate::display::{Display, TerminalDisplay};
use crate::snake::{Direction, Point, SnakeState};

// const HELP: &str = r#"EventStream based on futures_util::Stream with tokio
//  - Keyboard, mouse and terminal resize events enabled
//  - Prints "." every second if there's no event
//  - Hit "c" to print current cursor position
//  - Use Esc to quit
// "#;

static FRAME_PER_SEC: u64 = 5;
static REFRESH_RATE_MS: u64 = 1_000 / FRAME_PER_SEC;

static X_SIZE: u16 = 60;
static Y_SIZE: u16 = 30;

fn update_snake(state: &mut SnakeState) {
    let head = state.body.front().unwrap();
    match state.direction {
        Direction::Up => state.body.push_front(Point {
            x: head.x,
            y: head.y - 1,
        }),
        Direction::Down => state.body.push_front(Point {
            x: head.x,
            y: head.y + 1,
        }),
        Direction::Left => state.body.push_front(Point {
            x: head.x - 1,
            y: head.y,
        }),
        Direction::Right => state.body.push_front(Point {
            x: head.x + 1,
            y: head.y,
        }),
    }
    state.body.pop_back();
}

fn check_for_collision(state: &SnakeState) -> bool {
    let head = state.body.front().unwrap(); // TODO: handle this ??
    match state.direction {
        Direction::Up => head.y - 1 == 0,
        Direction::Down => head.y + 1 == Y_SIZE - 1,
        Direction::Left => head.x - 1 == 0,
        Direction::Right => head.x + 1 == X_SIZE - 1,
    }
}

async fn print_events(display: &mut dyn Display) -> std::io::Result<()> {
    // let mut reader = EventStream::new();
    let mut state = SnakeState::new(X_SIZE, Y_SIZE);
    display.draw(&state, state.body.back().unwrap().clone());

    loop {
        // let mut delay = Delay::new(Duration::from_millis(REFRESH_RATE_MS)).fuse();
        // let mut event = reader.next().fuse();
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: _,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: _,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => state.change_direction(Direction::Up),
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: _,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => state.change_direction(Direction::Down),
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: _,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => state.change_direction(Direction::Left),
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: _,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => state.change_direction(Direction::Right),
                // Event::Key(KeyEvent{code: KeyCode::Char('c'), modifiers: _, kind: KeyEventKind::Press, state: _}) => println!("Cursor position: {:?}\r", cursor::position()),
                // Event::Key(KeyEvent{code, modifiers: _, kind: KeyEventKind::Press, state: _}) => println!("lolz: {:?}", code),
                _ => continue,
            }
        } else {
            if check_for_collision(&state) {
                println!("YOU DIE!!!");
                break;
            }
            update_snake(&mut state);
            display.draw(&state, state.body.back().unwrap().clone());
        }

        // select! {
        //     _ = delay => {
        //         if check_for_collision(&state) {
        //             println!("YOU DIE!!!");
        //             break;
        //         }
        //         update_snake(&mut state);
        //         display.draw(&state, state.body.back().unwrap().clone());
        //     },
        //     maybe_event = event => {
        //         match maybe_event {
        //             Some(Ok(event)) => {
        //                 match event {
        //                     Event::Key(KeyEvent{code: KeyCode::Esc, modifiers: _, kind: KeyEventKind::Press, state: _}) => break,
        //                     Event::Key(KeyEvent{code: KeyCode::Up, modifiers: _, kind: KeyEventKind::Press, state: _}) => change_direction(&mut state, Direction::Up),
        //                     Event::Key(KeyEvent{code: KeyCode::Down, modifiers: _, kind: KeyEventKind::Press, state: _}) => change_direction(&mut state, Direction::Down),
        //                     Event::Key(KeyEvent{code: KeyCode::Left, modifiers: _, kind: KeyEventKind::Press, state: _}) => change_direction(&mut state, Direction::Left),
        //                     Event::Key(KeyEvent{code: KeyCode::Right, modifiers: _, kind: KeyEventKind::Press, state: _}) => change_direction(&mut state, Direction::Right),
        //                     // Event::Key(KeyEvent{code: KeyCode::Char('c'), modifiers: _, kind: KeyEventKind::Press, state: _}) => println!("Cursor position: {:?}\r", cursor::position()),
        //                     // Event::Key(KeyEvent{code, modifiers: _, kind: KeyEventKind::Press, state: _}) => println!("lolz: {:?}", code),
        //                     _ => continue,
        //                 }
        //             }
        //             Some(Err(e)) => println!("Error: {:?}\r", e),
        //             None => break,
        //         }
        //     }
        // };
    }

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // enable_raw_mode()?;
    // let mut stdout = stdout();

    let mut d = TerminalDisplay::new(X_SIZE, Y_SIZE);
    d.init()?;
    d.start_game();

    // execute!(stdout, EnterAlternateScreen)?;
    // execute!(stdout, EnableMouseCapture)?;

    // println!("{}", HELP);
    let _ = print_events(&mut d).await;

    d.deinit()

    // execute!(stdout, DisableMouseCapture)?;
    // execute!(stdout, LeaveAlternateScreen)?;

    // disable_raw_mode()
}

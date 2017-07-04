// Life-like cellular automaton
extern crate termion;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::cursor::{Goto, Hide, Show};
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, color};


use std::io::{Write, stdout};

mod grid;
use grid::Grid;
mod ruleset;
use ruleset::Ruleset;




//------------------------------------------------------------------------------
// Main loop
//------------------------------------------------------------------------------
fn main() {
    main_loop(Ruleset{born: vec![3], survive: vec![2, 3]});
}

fn main_loop(ruleset: Ruleset) {
    let mut grid = Grid::new(30, 30, 2);
    let mut evolve = false;
    let mut event_stream = async_stdin().events();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    loop {
        write!(stdout, "{}{}{}", clear::All, Goto(1, 1), Hide).unwrap();
        if evolve {
            write!(stdout, "{}", color::Fg(color::Green)).unwrap();
            grid = grid.nextgen(&ruleset);
        } else {
            write!(stdout, "{}", color::Fg(color::Red)).unwrap();
        }
        if let Some(c) = event_stream.next() {
            match c.unwrap() {
                Event::Key(Key::Char('q')) => {
                    write!(stdout, "{}\r\n", Show).unwrap();
                    break;
                },
                Event::Key(Key::Char(' ')) => {
                    evolve = !evolve
                },
                Event::Mouse(MouseEvent::Press(_, x, y)) => {
                    let (i, j) = (y as usize, x as usize);
                    grid.cells[i][j] = !grid.cells[i][j];
                }
                _ => (),
            }
        }
        write!(stdout, "{}", grid).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

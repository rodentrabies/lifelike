use grid::Grid;
use ruleset::Ruleset;
use std::io::{Write, Read, stdout};
use std::time;
use std::thread;
use termion::event::Event::{Key, Mouse};
use termion::event::Key::Char;
use termion::event::MouseEvent::Press;
use termion::input::{Events, TermRead, MouseTerminal};
use termion::cursor::{Goto, Hide, Show};
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, color};

type State = (Grid, Ruleset, bool);
type Action = Fn(State) -> Option<State>;

fn get_action<R> (estream: &mut Events<R>) -> Box<Action>
    where R: Read {
    let noop = Box::new(|s: State| -> Option<State> {Some(s)});
    if let Some(event) = estream.next() {
        match event.unwrap() {
            Key(Char('q')) => Box::new(|_: State| -> Option<State> {None}),
            Key(Char(' ')) =>
                Box::new(|(g, r, e): State| -> Option<State> {Some((g, r, !e))}),
            Mouse(Press(_, x, y)) =>
                Box::new(move |(mut g, r, e): State| -> Option<State> {
                    let (i, j) = ((y - 4) as usize, (x - 3) as usize);
                    g.cells[i][j] = !g.cells[i][j];
                    Some((g, r, e))  
                }),
            _ => noop,
        }
    } else {noop}
}

pub fn mainloop(width: usize, height: usize, ruleset: Ruleset) {
    let mut counter = 0;
    let mut es = async_stdin().events();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let mut state = (Grid::new(width, height, 10), ruleset, false);
    while let Some((grid, ruleset, evolving)) = get_action(&mut es)(state) {
        write!(stdout, "{}{}{}", clear::All, Goto(1, 1), Hide).unwrap();
        write!(stdout, "generation #{}\r\nevolving: {}\r\n", counter, evolving).unwrap();
        if evolving {
            counter += 1;
            write!(stdout, "{}", color::Fg(color::Green)).unwrap();
        } else {
            write!(stdout, "{}", color::Fg(color::Red)).unwrap();
        }
        write!(stdout, "{}{}", grid, color::Fg(color::Reset)).unwrap();
        state = if evolving {
            (grid.nextgen(&ruleset), ruleset, evolving)
        } else {
            (grid, ruleset, evolving)
        };
        thread::sleep(time::Duration::from_secs(1));
    }
    write!(stdout, "{}", Show).unwrap();
}

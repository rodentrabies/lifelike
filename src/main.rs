// Life-like cellular automaton
extern crate rand;
extern crate regex;
extern crate termion;

mod grid;
mod ruleset;
mod ui;

use std::str::FromStr;
use ruleset::Ruleset;
use ui::mainloop;

fn main() {
    mainloop(30, 30, Ruleset::from_str("B3/S23").unwrap());
}

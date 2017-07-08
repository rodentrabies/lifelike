// Life-like cellular automaton
extern crate rand;
extern crate termion;

mod grid;
mod ruleset;
mod ui;

use ruleset::Ruleset;
use ui::mainloop;

fn main() {
    mainloop(30, 30, Ruleset{born: vec![3], survive: vec![2, 3]});
}


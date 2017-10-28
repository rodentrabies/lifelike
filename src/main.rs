// Life-like cellular automaton
extern crate getopts;
extern crate rand;
extern crate regex;
extern crate termion;

mod grid;
mod ruleset;
mod ui;

use getopts::Options;
use ruleset::Ruleset;
use std::env;
use std::str::FromStr;
use ui::mainloop;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut opts = Options::new();
    opts.optopt("r", "ruleset", "Ruleset (default: 'B3/S23')", "RULES");
    opts.optflag("h", "help", "Show help menu");
    let matches = opts.parse(&args).unwrap();
    let ruleset = match matches.opt_str("r") {
        Some(s) => s,
        None => String::from("B3/S23"),
    };
    if matches.opt_present("h") && !matches.free.is_empty() {
        let brief = format!("Usage: {} [options]", args[0].clone());
        print!("{}", opts.usage(&brief));
    } else {
        mainloop(30, 30, Ruleset::from_str(&ruleset).unwrap());
    }
}

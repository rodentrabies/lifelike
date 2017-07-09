use regex::Regex;
use std::str::FromStr;


#[derive(Debug)]
pub struct ParseRulesetError;

pub struct Ruleset {
    pub born: Vec<i8>,
    pub survive: Vec<i8>,
}

impl FromStr for Ruleset {
    type Err = ParseRulesetError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(b|B)(?P<b>[1-9]+)/(s|S)(?P<s>[1-9]+)").unwrap();
        let parse_digits = |s: &str| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>()
        };
        match re.captures(s) {
            Some(cap) => Ok(Ruleset {
                born: parse_digits(&cap["b"]),
                survive: parse_digits(&cap["s"]),
            }),
            _ => Err(ParseRulesetError),
        }
    }
}

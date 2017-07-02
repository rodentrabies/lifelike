// Life-like cellular automaton
extern crate termion;
extern crate rand;

use rand::Rng;
use std::cmp::{min, max};

//------------------------------------------------------------------------------
// Main loop
//------------------------------------------------------------------------------
fn main() {
    main_loop(Ruleset{born: vec![3], survive: vec![2, 3]});
}

fn main_loop(ruleset: Ruleset) {
    let mut grid = Grid::new(20, 20, true);
    loop {
        std::process::Command::new("clear").status().unwrap().success();
        println!("{}", grid);
        std::thread::sleep(std::time::Duration::from_secs(1));
        grid = grid.nextgen(&ruleset);
    }
}

//------------------------------------------------------------------------------
// Grid type
//------------------------------------------------------------------------------
#[derive(Clone, Eq, PartialEq)]
enum Cell { Alive, Dead }

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(width: usize, height: usize, randomize: bool) -> Self {
        let mut cells = vec![vec![Cell::Dead; width]; height];
        if randomize {
            for i in 0..height {
                for j in 0..width {
                    let chance = rand::thread_rng().gen_range(0, 15);
                    if chance == 0 {
                        cells[i][j] = Cell::Alive;
                    }
                }
            }
        }
        Grid{width: width, height: height, cells: cells}
    }

    fn neighbours(&self, y: usize, x: usize) -> i8 {
        let mut neighbours_count = 0i8;
        for i in max(0, y as i32 - 1) as usize..min(self.height, y + 1) {
            for j in max(0, x as i32 - 1) as usize..min(self.width, x + 1) {
                if self.cells[i][j] == Cell::Alive {
                    neighbours_count += 1;
                }
            }
        }
        neighbours_count
    }

    fn nextgen(&self, ruleset: &Ruleset) -> Self {
        let mut new_grid = Grid::new(self.width, self.height, false);
        for i in 0..self.height {
            for j in 0..self.width {
                let n_count = self.neighbours(i, j);
                match self.cells[i][j] {
                    Cell::Alive if ruleset.survive.contains(&n_count)
                        => new_grid.cells[i][j] = Cell::Alive,
                    Cell::Dead if ruleset.born.contains(&n_count)
                        => new_grid.cells[i][j] = Cell::Alive,
                    _ => new_grid.cells[i][j] = Cell::Dead,
                }
            }
        }
        new_grid
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.cells.as_slice() {
            for cell in line.as_slice() {
                match *cell {
                    Cell::Alive => write!(f, "*").unwrap(),
                    Cell::Dead => write!(f, " ").unwrap(),
                }
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}


//------------------------------------------------------------------------------
// Ruleset type
//------------------------------------------------------------------------------
struct Ruleset {
    born: Vec<i8>,
    survive: Vec<i8>,
}

// impl std::str::FromStr for Ruleset {
//     type Err = std::string::ParseError;
//     fn from_str(s: str) -> Result<Self, Self::Err> {
//         match s.split('/').collect().as_slice() {
//             [born, survive] => {
                
//             },
//             [_] => Err,
//             [] => Err,
//         }
//     }
// }

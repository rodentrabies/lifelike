use rand::Rng;
use rand;
use ruleset::Ruleset;
use std::cmp::{max, min};
use std::fmt::{Display, Formatter, Result};
use std::ops::Not;


#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Cell { Alive, Dead }

impl Not for Cell {
    type Output = Cell;
    fn not(self) -> Cell {
        match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive
        }
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: usize, height: usize, r: i8) -> Self {
        let mut cells = vec![vec![Cell::Dead; width]; height];
        if r != 0 {
            for i in 0..height {
                for j in 0..width {
                    let chance = rand::thread_rng().gen_range(0, 100 / r);
                    if chance == 0 {
                        cells[i][j] = Cell::Alive;
                    }
                }
            }
        }
        Grid{width: width, height: height, cells: cells}
    }

    pub fn neighbours(&self, y: usize, x: usize) -> i8 {
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

    pub fn nextgen(self, ruleset: &Ruleset) -> Grid {
        let mut new_grid = Grid::new(self.width, self.height, 0);
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

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for line in self.cells.as_slice() {
            for cell in line.as_slice() {
                match *cell {
                    Cell::Alive => write!(f, "\u{2022}").unwrap(),
                    Cell::Dead => write!(f, " ").unwrap(),
                }
            }
            write!(f, "\r\n").unwrap();
        }
        Ok(())
    }
}

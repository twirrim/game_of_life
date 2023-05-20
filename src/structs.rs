use std::fmt;

use bilge::prelude::*;

use ansi_term::Colour::{Green, Red};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long,  value_parser = clap::value_parser!(i32).range(2..16384))]
    pub width: i32,
    #[arg(short, long,  value_parser = clap::value_parser!(i32).range(2..16384))]
    pub height: i32,
    #[arg(short, long)]
    pub frames: usize,
    #[arg(short, long)]
    pub output: String,
}

#[bitsize(4)]
#[derive(DebugBits, FromBits, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct State {
    pub alive: bool,
    pub neighbours: u3,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alive = match &self.alive() {
            false => Red.paint("\u{274C}"),
            true => Green.paint("\u{2705}"),
        };
        write!(f, "{alive},{}", &self.neighbours())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Colony {
    pub cells: Vec<Vec<State>>,
}

impl Colony {
    pub fn new(width: isize, height: isize) -> Colony {
        // Really don't like the way rustfmt is formatting this!
        // It's: vec![vec![State { alive: false, neighbours: 0 }; height]; width]
        Colony {
            cells: vec![vec![State::new(false, u3::new(0b000)); height as usize]; width as usize],
        }
    }

    fn set_target_state(&mut self, x: isize, y: isize, state: bool) {
        if self.cells[x as usize][y as usize].alive() == state {
            return;
        };
        // Make it live/die
        self.cells[x as usize][y as usize].set_alive(state);

        let offsets = vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ];

        // Update the neighbour counts
        for (x, y) in offsets {
            // Argh more fun with type casting
            if x >= self.cells.len() as isize
                || x < 0
                || y < 0
                || y >= self.cells[x as usize].len() as isize
            {
                continue;
            };
            let neighbours = self.cells[x as usize][y as usize].neighbours();
            if state {
                self.cells[x as usize][y as usize].set_neighbours(neighbours + u3::new(0b001));
            } else {
                self.cells[x as usize][y as usize].set_neighbours(neighbours - u3::new(0b001));
            };
        }
    }

    pub fn make_alive(&mut self, x: usize, y: usize) {
        self.set_target_state(x as isize, y as isize, true);
    }

    pub fn make_dead(&mut self, x: usize, y: usize) {
        self.set_target_state(x as isize, y as isize, false);
    }

    pub fn print(&self) {
        for row in &self.cells {
            println!("{:?}", row);
        }
    }
}

impl fmt::Display for Colony {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = vec![];
        for row in &self.cells {
            let mut row_out = vec![];
            for cell in row {
                let alive = match cell.alive() {
                    false => Red.paint("\u{274C}"),
                    true => Green.paint("\u{2705}"),
                };
                row_out.push(format!("{alive},{}", cell.neighbours()));
            }
            row_out.push(String::from("\n"));
            output.push(row_out.join("|"));
        }
        write!(f, "{}", output.join(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_alive_on_small() {
        let mut colony = Colony::new(3, 3);
        colony.make_alive(1, 1);
        println!("Checking aliveness");
        // Check aliveness
        println!("{colony}");
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                if x == 1 && y == 1 {
                    assert!(colony.cells[x][y].alive());
                } else {
                    assert!(!colony.cells[x][y].alive());
                };
            }
        }
        // Check counts
        println!("Checking counts");
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                if x == 1 && y == 1 {
                    assert_eq!(colony.cells[x][y].neighbours(), u3::new(0));
                } else {
                    assert_eq!(colony.cells[x][y].neighbours(), u3::new(1));
                };
            }
        }
    }

    #[test]
    fn test_kill_on_small() {
        let mut colony = Colony::new(3, 3);
        colony.make_alive(1, 1);
        colony.make_dead(1, 1);
        colony.print();
        println!("Checking dead");
        // Check nothing lives
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                assert!(!colony.cells[x][y].alive())
            }
        }
        println!("Checking zero neighbours");
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                assert_eq!(colony.cells[x][y].neighbours(), u3::new(0))
            }
        }
    }

    #[test]
    fn make_alive_on_small_edge() {
        let mut colony = Colony::new(3, 3);
        colony.make_alive(0, 0);
        colony.print();
        println!("Checking aliveness");
        // Check aliveness
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                if x == 0 && y == 0 {
                    assert!(colony.cells[x][y].alive());
                } else {
                    assert!(!colony.cells[x][y].alive());
                };
            }
        }
        // Check counts
        println!("Checking counts");
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                if (x == 0 && y == 0) || x == 2 || y == 2 {
                    assert_eq!(colony.cells[x][y].neighbours(), u3::new(0));
                } else {
                    assert_eq!(colony.cells[x][y].neighbours(), u3::new(1));
                };
            }
        }
    }

    #[test]
    fn test_kill_on_small_edge() {
        let mut colony = Colony::new(3, 3);
        colony.make_alive(0, 0);
        colony.make_dead(0, 0);
        // Check nothing lives
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                assert!(!colony.cells[x][y].alive())
            }
        }
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                assert_eq!(colony.cells[x][y].neighbours(), u3::new(0))
            }
        }
    }

    #[test]
    fn make_alive_on_larger() {
        let mut colony = Colony::new(30, 30);
        colony.make_alive(14, 14);
        // Check aliveness
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                if x == 14 && y == 14 {
                    assert!(colony.cells[x][y].alive());
                } else {
                    assert!(!colony.cells[x][y].alive());
                };
            }
        }
        // Check counts
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                if x >= 13 && x <= 15 && y >= 13 && y <= 15 {
                    if x == 14 && y == 14 {
                        assert_eq!(colony.cells[x][y].neighbours(), u3::new(0));
                    } else {
                        assert_eq!(colony.cells[x][y].neighbours(), u3::new(1));
                    };
                } else {
                    assert_eq!(colony.cells[x][y].neighbours(), u3::new(0));
                };
            }
        }
    }

    #[test]
    fn test_kill_on_larger() {
        let mut colony = Colony::new(30, 30);
        colony.make_alive(14, 14);
        colony.make_dead(14, 14);
        // Check nothing lives
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                assert!(!colony.cells[x][y].alive())
            }
        }
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                assert_eq!(colony.cells[x][y].neighbours(), u3::new(0))
            }
        }
    }

    #[test]
    fn make_alive_twice_on_small() {
        let mut colony = Colony::new(3, 3);
        colony.make_alive(1, 1);
        colony.make_alive(1, 1);
        colony.print();
        println!("Checking aliveness");
        // Check aliveness
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                if x == 1 && y == 1 {
                    assert!(colony.cells[x][y].alive());
                } else {
                    assert!(!colony.cells[x][y].alive());
                };
            }
        }
        // Check counts
        println!("Checking counts");
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                if x == 1 && y == 1 {
                    assert_eq!(colony.cells[x][y].neighbours(), u3::new(0));
                } else {
                    assert_eq!(colony.cells[x][y].neighbours(), u3::new(1));
                };
            }
        }
    }

    #[test]
    fn make_dead_twice_on_small() {
        let mut colony = Colony::new(3, 3);
        colony.make_alive(1, 1);
        colony.make_dead(1, 1);
        colony.make_dead(1, 1);
        colony.print();
        println!("Checking aliveness");
        // Check aliveness
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                assert!(!colony.cells[x][y].alive());
            }
        }
        // Check counts
        println!("Checking counts");
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                assert_eq!(colony.cells[x][y].neighbours(), u3::new(0));
            }
        }
    }
}

use std::fmt;

use get_size::GetSize;
use get_size_derive::*;

use ansi_term::Colour::{Green, Red};

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy, GetSize)]
pub struct State {
    pub life_left: u8,
    pub neighbours: usize,
}

impl State {
    pub fn is_alive(self) -> bool {
        self.life_left == 255
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alive = match &self.is_alive() {
            false => Red.paint("\u{274C}"),
            true => Green.paint("\u{2705}"),
        };
        write!(f, "{alive},{},{:03}", &self.neighbours, &self.life_left)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, GetSize)]
pub struct Colony {
    pub cells: Vec<Vec<State>>,
}

impl Colony {
    pub fn new(width: usize, height: usize) -> Colony {
        // Really don't like the way rustfmt is formatting this!
        // It's: vec![vec![State { alive: false, neighbours: 0 }; height]; width]
        Colony {
            cells: vec![
                vec![
                    State {
                        life_left: 0,
                        neighbours: 0
                    };
                    height
                ];
                width
            ],
        }
    }

    fn set_target_state(&mut self, x: i32, y: i32, state: bool) {
        if self.cells[x as usize][y as usize].is_alive() == state {
            return;
        };
        // Make it live/die
        if state {
            self.cells[x as usize][y as usize].life_left = 255
        } else {
            self.cells[x as usize][y as usize].life_left = 254
        };

        // Update the neighbour counts
        for (off_x, off_y) in OFFSETS {
            // Argh more fun with type casting
            if ((x >= self.cells.len() as i32 - 1) && off_x == 1)
                || ((y >= self.cells[x as usize].len() as i32 - 1) && off_y == 1)
                || (x == 0 && off_x == -1)
                || (y == 0 && off_y == -1)
            {
                continue;
            };
            if state {
                self.cells[(x + off_x) as usize][(y + off_y) as usize].neighbours += 1;
            } else {
                self.cells[(x + off_x) as usize][(y + off_y) as usize].neighbours -= 1;
            };
        }
    }

    pub fn make_alive(&mut self, x: usize, y: usize) {
        self.set_target_state(x as i32, y as i32, true);
    }

    pub fn make_dead(&mut self, x: usize, y: usize) {
        self.set_target_state(x as i32, y as i32, false);
    }

    pub fn reduce_life(&mut self, x: usize, y: usize) {
        if self.cells[x][y].life_left >= 20 {
            self.cells[x][y].life_left -= 20;
        } else {
            self.cells[x][y].life_left = 0;
        };
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
                let alive = match cell.is_alive() {
                    false => Red.paint("\u{274C}"),
                    true => Green.paint("\u{2705}"),
                };
                row_out.push(format!("{alive},{},{:03}", cell.neighbours, cell.life_left))
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
        colony.print();
        println!("Checking aliveness");
        // Check aliveness
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                if x == 1 && y == 1 {
                    assert!(colony.cells[x][y].is_alive());
                    assert_eq!(colony.cells[x][y].life_left, 255);
                } else {
                    assert!(!colony.cells[x][y].is_alive());
                };
            }
        }
        // Check counts
        println!("Checking counts");
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                if x == 1 && y == 1 {
                    assert_eq!(colony.cells[x][y].neighbours, 0);
                } else {
                    assert_eq!(colony.cells[x][y].neighbours, 1);
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
                assert!(!colony.cells[x][y].is_alive())
            }
        }
        println!("Checking zero neighbours");
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                assert_eq!(colony.cells[x][y].neighbours, 0)
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
                    assert!(colony.cells[x][y].is_alive());
                } else {
                    assert!(!colony.cells[x][y].is_alive());
                };
            }
        }
        // Check counts
        println!("Checking counts");
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                if (x == 0 && y == 0) || x == 2 || y == 2 {
                    assert_eq!(colony.cells[x][y].neighbours, 0);
                } else {
                    assert_eq!(colony.cells[x][y].neighbours, 1);
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
                assert!(!colony.cells[x][y].is_alive())
            }
        }
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                assert_eq!(colony.cells[x][y].neighbours, 0)
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
                    assert!(colony.cells[x][y].is_alive());
                } else {
                    assert!(!colony.cells[x][y].is_alive());
                };
            }
        }
        // Check counts
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                if x >= 13 && x <= 15 && y >= 13 && y <= 15 {
                    if x == 14 && y == 14 {
                        assert_eq!(colony.cells[x][y].neighbours, 0);
                    } else {
                        assert_eq!(colony.cells[x][y].neighbours, 1);
                    };
                } else {
                    assert_eq!(colony.cells[x][y].neighbours, 0);
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
                assert!(!colony.cells[x][y].is_alive())
            }
        }
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                assert_eq!(colony.cells[x][y].neighbours, 0)
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
                    assert!(colony.cells[x][y].is_alive());
                } else {
                    assert!(!colony.cells[x][y].is_alive());
                };
            }
        }
        // Check counts
        println!("Checking counts");
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                println!("{x},{y}");
                if x == 1 && y == 1 {
                    assert_eq!(colony.cells[x][y].neighbours, 0);
                } else {
                    assert_eq!(colony.cells[x][y].neighbours, 1);
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
                assert!(!colony.cells[x][y].is_alive());
            }
        }
        // Check counts
        println!("Checking counts");
        for x in 0..colony.cells.len() {
            for y in 0..colony.cells[x].len() {
                assert_eq!(colony.cells[x][y].neighbours, 0);
            }
        }
    }
}

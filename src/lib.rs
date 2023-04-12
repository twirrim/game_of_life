pub mod consts;
pub mod structs;

extern crate lazy_static;

use rand::Rng;
// use rayon::prelude::*;

use crate::consts::{HEIGHT, WIDTH};
use crate::structs::{Colony, State};

// I'm in type hell here with integers.  Need to do some serious refectoring

pub fn initialise(starting_cells: u32) -> Colony {
    // Define a colony with every cell dead and no neighbours
    let mut colony = vec![
        vec![
            State {
                alive: false,
                neighbour_count: 0
            };
            HEIGHT as usize
        ];
        WIDTH as usize
    ];
    let mut rng = rand::thread_rng();
    for _ in 0..starting_cells {
        let x = rng.gen_range(0..WIDTH) as usize;
        let y = rng.gen_range(0..HEIGHT) as usize;

        // Set the cells as active
        colony[x][y] = State {
            alive: true,
            neighbour_count: 0,
        };
        // Find their neighbours and increment neighbour_count!
        let neighbours = produce_neighbours(x as i32, y as i32);
        for (x, y) in neighbours {
            colony[x as usize][y as usize].neighbour_count += 1;
        }
    }
    colony
}

pub fn produce_neighbours(x: i32, y: i32) -> Vec<(i32, i32)> {
    let offsets: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut neighbours = vec![];
    for (x_off, y_off) in offsets {
        if
        // Catch overflows and underflows, and off-the-edge-of-map
        ((x == i32::MAX || x >= WIDTH - 1) && x_off == 1)
            || ((y == i32::MAX || y >= HEIGHT - 1) && y_off == 1)
            || (x == 0 && x_off == -1)
            || (y == 0 && y_off == -1)
        {
            continue;
        }
        neighbours.push((x + x_off, y + y_off));
    }
    neighbours
}

pub fn process_frame(active_cells: &Colony) -> Colony {
    let mut clone = active_cells.clone();
    for x_index in 0..active_cells.len() {
        for y_index in 0..active_cells[x_index].len() {
            let cell = active_cells[x_index][y_index];
            // Only conditions under which a cell lives
            if cell.neighbour_count == 3 || (cell.neighbour_count == 2 && !cell.alive) {
                // More integer type madness!
                let neighbours = produce_neighbours(x_index as i32, y_index as i32);
                for (col_x, col_y) in neighbours {
                    clone[col_x as usize][col_y as usize].neighbour_count += 1;
                }
                clone[x_index][y_index].alive = true;
            } else if cell.alive {
                let neighbours = produce_neighbours(x_index as i32, y_index as i32);
                for (col_x, col_y) in neighbours {
                    clone[col_x as usize][col_y as usize].neighbour_count -= 1;
                }
                clone[x_index][y_index].alive = false;
            }
        }
    }
    clone
}

#[cfg(test)]
mod tests {

    // use super::*;

    //#[test]
    //fn blinks() {
    //    // Simple blinker
    //    let mut give = FxHashSet::default();
    //    give.insert(Cell { x: 1, y: 2 });
    //    give.insert(Cell { x: 2, y: 2 });
    //    give.insert(Cell { x: 3, y: 2 });

    //    let mut want = FxHashSet::default();
    //    want.insert(Cell { x: 2, y: 1 });
    //    want.insert(Cell { x: 2, y: 2 });
    //    want.insert(Cell { x: 2, y: 3 });

    //    let got = process_frame(&give);
    //    assert_eq!(got, want);
    //}

    //#[test]
    //fn still() {
    //    let mut give = FxHashSet::default();
    //    give.insert(Cell { x: 1, y: 1 });
    //    give.insert(Cell { x: 1, y: 2 });
    //    give.insert(Cell { x: 2, y: 1 });
    //    give.insert(Cell { x: 2, y: 2 });

    //    let got = process_frame(&give);
    //    assert_eq!(got, give);
    //}

    //#[test]
    //fn test_produce_neighbours_simple() {
    //    let give = Cell { x: 2, y: 2 };
    //    let want = vec![
    //        Cell { x: 1, y: 1 },
    //        Cell { x: 1, y: 2 },
    //        Cell { x: 1, y: 3 },
    //        Cell { x: 2, y: 1 },
    //        Cell { x: 2, y: 3 },
    //        Cell { x: 3, y: 1 },
    //        Cell { x: 3, y: 2 },
    //        Cell { x: 3, y: 3 },
    //    ];
    //    let got = produce_neighbours(&give);
    //    assert_eq!(want, got);
    //}

    //#[test]
    //fn test_produce_neighbours_edges() {
    //    let give = Cell { x: 0, y: 0 };
    //    // Should get [(0,1), ((1,0), (1, 1)]
    //    let want = vec![
    //        Cell { x: 0, y: 1 },
    //        Cell { x: 1, y: 0 },
    //        Cell { x: 1, y: 1 },
    //    ];
    //    let got = produce_neighbours(&give);
    //    assert_eq!(want, got);

    //    // Overflow check!
    //    let give = Cell {
    //        x: i32::MAX,
    //        y: i32::MAX,
    //    };
    //    // Should get [(i32::MAX - 1, i32::MAX -1), (i32::MAX -1, i32::MAX), (i32::MAX, i32::MAX -1)]
    //    let want = vec![
    //        Cell {
    //            x: i32::MAX - 1,
    //            y: i32::MAX - 1,
    //        },
    //        Cell {
    //            x: i32::MAX - 1,
    //            y: i32::MAX,
    //        },
    //        Cell {
    //            x: i32::MAX,
    //            y: i32::MAX - 1,
    //        },
    //    ];
    //    let got = produce_neighbours(&give);
    //    assert_eq!(want, got);
    //}

    //#[test]
    //fn test_get_neighbour_counts_basic() {
    //    let mut give = FxHashSet::default();
    //    give.insert(Cell { x: 2, y: 2 });
    //    give.insert(Cell { x: 2, y: 3 });

    //    let mut want = FxHashMap::default();
    //    want.insert(Cell { x: 1, y: 1 }, 1);
    //    want.insert(Cell { x: 1, y: 2 }, 2);
    //    want.insert(Cell { x: 1, y: 3 }, 2);
    //    want.insert(Cell { x: 1, y: 4 }, 1);
    //    want.insert(Cell { x: 2, y: 1 }, 1);
    //    want.insert(Cell { x: 2, y: 2 }, 1);
    //    want.insert(Cell { x: 2, y: 3 }, 1);
    //    want.insert(Cell { x: 2, y: 4 }, 1);
    //    want.insert(Cell { x: 3, y: 1 }, 1);
    //    want.insert(Cell { x: 3, y: 2 }, 2);
    //    want.insert(Cell { x: 3, y: 3 }, 2);
    //    want.insert(Cell { x: 3, y: 4 }, 1);

    //    let got = get_neighbour_counts(&give);

    //    assert_eq!(want, got);
    //}
}

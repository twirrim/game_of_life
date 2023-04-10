pub mod consts;
pub mod structs;

extern crate lazy_static;

use std::cmp;
use std::thread::available_parallelism;

use rayon::prelude::*;

use rustc_hash::{FxHashMap, FxHashSet};

use crate::consts::{HEIGHT, WIDTH};
use crate::structs::Cell;

fn produce_neighbours(cell: &Cell) -> Vec<Cell> {
    let offsets = vec![
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
    for (x, y) in offsets {
        if
        // Catch overflows and underflows, and off-the-edge-of-map
        ((cell.x == i32::MAX || cell.x >= WIDTH - 1) && x == 1)
            || ((cell.y == i32::MAX || cell.y >= HEIGHT - 1) && y == 1)
            || (cell.x == 0 && x == -1)
            || (cell.y == 0 && y == -1)
        {
            continue;
        }
        neighbours.push(Cell {
            x: cell.x + x,
            y: cell.y + y,
        });
    }
    neighbours
}

fn batch_produce_neighbours(cells: Vec<Cell>) -> FxHashMap<Cell, u32> {
    let mut neighbour_counts = FxHashMap::default();
    for cell in cells {
        for neighbour in produce_neighbours(&cell) {
            *neighbour_counts.entry(neighbour).or_insert(0) += 1;
        }
    }
    neighbour_counts
}

fn get_neighbour_counts(active_cells: &FxHashSet<Cell>) -> FxHashMap<Cell, u32> {
    let mut neighbour_counts = FxHashMap::default();
    let chunk_size: usize = cmp::max(
        (active_cells.len() as f32 / available_parallelism().unwrap().get() as f32) as usize,
        10,
    );
    let collection: Vec<FxHashMap<Cell, u32>> = active_cells
        .clone()
        .into_iter()
        .collect::<Vec<Cell>>()
        .into_par_iter()
        .chunks(chunk_size)
        .map(batch_produce_neighbours)
        .collect();
    for map in collection {
        for (key, value) in map {
            *neighbour_counts.entry(key).or_insert(0) += value;
        }
    }
    neighbour_counts
}

pub fn process_frame(active_cells: &FxHashSet<Cell>) -> FxHashSet<Cell> {
    let neighbour_counts = get_neighbour_counts(&active_cells);

    neighbour_counts
        .into_iter()
        .filter_map(
            |(cell, count)| match (count, active_cells.contains(&cell)) {
                // If there's 2 neighbours on an active cell, or three neighbours regardless of
                // state, cell is live
                (2, true) | (3, ..) => Some(cell),
                // Otherwise, cell dies, or remains, dead.
                _ => None,
            },
        )
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn blinks() {
        // Simple blinker
        let mut give = FxHashSet::default();
        give.insert(Cell { x: 1, y: 2 });
        give.insert(Cell { x: 2, y: 2 });
        give.insert(Cell { x: 3, y: 2 });

        let mut want = FxHashSet::default();
        want.insert(Cell { x: 2, y: 1 });
        want.insert(Cell { x: 2, y: 2 });
        want.insert(Cell { x: 2, y: 3 });

        let got = process_frame(&give);
        assert_eq!(got, want);
    }

    #[test]
    fn still() {
        let mut give = FxHashSet::default();
        give.insert(Cell { x: 1, y: 1 });
        give.insert(Cell { x: 1, y: 2 });
        give.insert(Cell { x: 2, y: 1 });
        give.insert(Cell { x: 2, y: 2 });

        let got = process_frame(&give);
        assert_eq!(got, give);
    }

    #[test]
    fn test_produce_neighbours_simple() {
        let give = Cell { x: 2, y: 2 };
        let want = vec![
            Cell { x: 1, y: 1 },
            Cell { x: 1, y: 2 },
            Cell { x: 1, y: 3 },
            Cell { x: 2, y: 1 },
            Cell { x: 2, y: 3 },
            Cell { x: 3, y: 1 },
            Cell { x: 3, y: 2 },
            Cell { x: 3, y: 3 },
        ];
        let got = produce_neighbours(&give);
        assert_eq!(want, got);
    }

    #[test]
    fn test_produce_neighbours_edges() {
        let give = Cell { x: 0, y: 0 };
        // Should get [(0,1), ((1,0), (1, 1)]
        let want = vec![
            Cell { x: 0, y: 1 },
            Cell { x: 1, y: 0 },
            Cell { x: 1, y: 1 },
        ];
        let got = produce_neighbours(&give);
        assert_eq!(want, got);

        // Overflow check!
        let give = Cell {
            x: i32::MAX,
            y: i32::MAX,
        };
        // Should get [(i32::MAX - 1, i32::MAX -1), (i32::MAX -1, i32::MAX), (i32::MAX, i32::MAX -1)]
        let want = vec![
            Cell {
                x: i32::MAX - 1,
                y: i32::MAX - 1,
            },
            Cell {
                x: i32::MAX - 1,
                y: i32::MAX,
            },
            Cell {
                x: i32::MAX,
                y: i32::MAX - 1,
            },
        ];
        let got = produce_neighbours(&give);
        assert_eq!(want, got);
    }

    #[test]
    fn test_get_neighbour_counts_basic() {
        let mut give = FxHashSet::default();
        give.insert(Cell { x: 2, y: 2 });
        give.insert(Cell { x: 2, y: 3 });

        let mut want = FxHashMap::default();
        want.insert(Cell { x: 1, y: 1 }, 1);
        want.insert(Cell { x: 1, y: 2 }, 2);
        want.insert(Cell { x: 1, y: 3 }, 2);
        want.insert(Cell { x: 1, y: 4 }, 1);
        want.insert(Cell { x: 2, y: 1 }, 1);
        want.insert(Cell { x: 2, y: 2 }, 1);
        want.insert(Cell { x: 2, y: 3 }, 1);
        want.insert(Cell { x: 2, y: 4 }, 1);
        want.insert(Cell { x: 3, y: 1 }, 1);
        want.insert(Cell { x: 3, y: 2 }, 2);
        want.insert(Cell { x: 3, y: 3 }, 2);
        want.insert(Cell { x: 3, y: 4 }, 1);

        let got = get_neighbour_counts(&give);

        assert_eq!(want, got);
    }
}

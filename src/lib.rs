pub mod consts;
pub mod structs;

extern crate lazy_static;

use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::consts::{HEIGHT, WIDTH};
use crate::structs::Cell;

fn produce_neighbours(cell: &Cell) -> Vec<Cell> {
    let neighbours = vec![
        // Left column
        Cell {
            x: cell.x - 1,
            y: cell.y - 1,
        },
        Cell {
            x: cell.x - 1,
            y: cell.y,
        },
        Cell {
            x: cell.x - 1,
            y: cell.y + 1,
        },
        // Central column
        Cell {
            x: cell.x,
            y: cell.y - 1,
        },
        Cell {
            x: cell.x,
            y: cell.y + 1,
        },
        // Right column
        Cell {
            x: cell.x + 1,
            y: cell.y - 1,
        },
        Cell {
            x: cell.x + 1,
            y: cell.y,
        },
        Cell {
            x: cell.x + 1,
            y: cell.y + 1,
        },
    ];

    neighbours
        .into_iter()
        .filter(|cell| !(cell.x < 0 || cell.x >= WIDTH || cell.y < 0 || cell.y >= HEIGHT))
        .collect()
}

fn get_neighbour_counts(active_cells: &FxHashSet<Cell>) -> FxHashMap<Cell, u32> {
    let mut neighbour_counts = FxHashMap::default();
    for cell in active_cells.into_iter().flat_map(produce_neighbours) {
        *neighbour_counts.entry(cell).or_insert(0) += 1;
    }
    neighbour_counts
}

pub fn process_frame(active_cells: &FxHashSet<Cell>) -> FxHashSet<Cell> {
    let neighbour_counts = get_neighbour_counts(&active_cells);

    neighbour_counts
        .into_par_iter()
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
}

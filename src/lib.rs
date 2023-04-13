pub mod consts;
pub mod structs;

extern crate lazy_static;

use rand::Rng;

use crate::structs::Colony;

pub fn initialise(starting_cells: u32, width: usize, height: usize) -> Colony {
    // Make a colony of the specified size, all dead
    let mut colony = Colony::new(width, height);
    let mut rng = rand::thread_rng();
    for _ in 0..starting_cells {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        colony.make_alive(x, y);
    }
    colony
}

pub fn process_frame(colony: &mut Colony) {
    // TODO: Parallelise this?  Could maybe use a parallelised filter to find the cells that need to be updated,
    // then do that in a sequential loop?  This also helps avoid a clone of the colony
    // In fact... let's start out towards that way...
    let mut make_alive: Vec<(usize, usize)> = vec![];
    let mut make_dead: Vec<(usize, usize)> = vec![];
    for (x, row) in colony.cells.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if (cell.alive && cell.neighbours == 2) || (cell.neighbours == 3) {
                make_alive.push((x, y));
            } else if cell.alive {
                make_dead.push((x, y));
            };
        }
    }
    for (x, y) in make_alive {
        colony.make_alive(x, y);
    }
    for (x, y) in make_dead {
        colony.make_dead(x, y);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn blinks() {
        let mut starter = Colony::new(5, 5);
        starter.make_alive(1, 2);
        starter.make_alive(2, 2);
        starter.make_alive(3, 2);

        let mut want = Colony::new(5, 5);
        want.make_alive(2, 1);
        want.make_alive(2, 2);
        want.make_alive(3, 3);

        let mut got = starter.clone();
        process_frame(&mut got);
        assert_eq!(got, want);
    }
    //
    //    #[test]
    //    fn still() {
    //        let mut give = FxHashSet::default();
    //        give.insert(Cell { x: 1, y: 1 });
    //        give.insert(Cell { x: 1, y: 2 });
    //        give.insert(Cell { x: 2, y: 1 });
    //        give.insert(Cell { x: 2, y: 2 });
    //
    //        let got = process_frame(&give);
    //        assert_eq!(got, give);
    //    }
    //
    //    #[test]
    //    fn test_produce_neighbours_simple() {
    //        let give = Cell { x: 2, y: 2 };
    //        let want = vec![
    //            Cell { x: 1, y: 1 },
    //            Cell { x: 1, y: 2 },
    //            Cell { x: 1, y: 3 },
    //            Cell { x: 2, y: 1 },
    //            Cell { x: 2, y: 3 },
    //            Cell { x: 3, y: 1 },
    //            Cell { x: 3, y: 2 },
    //            Cell { x: 3, y: 3 },
    //        ];
    //        let got = produce_neighbours(&give);
    //        assert_eq!(want, got);
    //    }
    //
    //    #[test]
    //    fn test_produce_neighbours_edges() {
    //        let give = Cell { x: 0, y: 0 };
    //        // Should get [(0,1), ((1,0), (1, 1)]
    //        let want = vec![
    //            Cell { x: 0, y: 1 },
    //            Cell { x: 1, y: 0 },
    //            Cell { x: 1, y: 1 },
    //        ];
    //        let got = produce_neighbours(&give);
    //        assert_eq!(want, got);
    //
    //        // Overflow check!
    //        let give = Cell {
    //            x: i32::MAX,
    //            y: i32::MAX,
    //        };
    //        // Should get [(i32::MAX - 1, i32::MAX -1), (i32::MAX -1, i32::MAX), (i32::MAX, i32::MAX -1)]
    //        let want = vec![
    //            Cell {
    //                x: i32::MAX - 1,
    //                y: i32::MAX - 1,
    //            },
    //            Cell {
    //                x: i32::MAX - 1,
    //                y: i32::MAX,
    //            },
    //            Cell {
    //                x: i32::MAX,
    //                y: i32::MAX - 1,
    //            },
    //        ];
    //        let got = produce_neighbours(&give);
    //        assert_eq!(want, got);
    //    }
    //
    //    #[test]
    //    fn test_get_neighbour_counts_basic() {
    //        let mut give = FxHashSet::default();
    //        give.insert(Cell { x: 2, y: 2 });
    //        give.insert(Cell { x: 2, y: 3 });
    //
    //        let mut want = FxHashMap::default();
    //        want.insert(Cell { x: 1, y: 1 }, 1);
    //        want.insert(Cell { x: 1, y: 2 }, 2);
    //        want.insert(Cell { x: 1, y: 3 }, 2);
    //        want.insert(Cell { x: 1, y: 4 }, 1);
    //        want.insert(Cell { x: 2, y: 1 }, 1);
    //        want.insert(Cell { x: 2, y: 2 }, 1);
    //        want.insert(Cell { x: 2, y: 3 }, 1);
    //        want.insert(Cell { x: 2, y: 4 }, 1);
    //        want.insert(Cell { x: 3, y: 1 }, 1);
    //        want.insert(Cell { x: 3, y: 2 }, 2);
    //        want.insert(Cell { x: 3, y: 3 }, 2);
    //        want.insert(Cell { x: 3, y: 4 }, 1);
    //
    //        let got = get_neighbour_counts(&give);
    //
    //        assert_eq!(want, got);
    //    }
}

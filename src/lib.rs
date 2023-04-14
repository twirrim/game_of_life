pub mod consts;
pub mod structs;

use rand::Rng;

use crate::structs::Colony;

pub fn initialise(starting_cells: u32, width: usize, height: usize) -> Colony {
    // Make a colony of the specified size, all dead
    let mut colony = Colony::new(width, height);
    let mut rng = rand::thread_rng();
    // for _ in 0..starting_cells {
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
            if (cell.alive && cell.neighbours == 2) || cell.neighbours == 3 {
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
        println!("{starter}");

        let mut want = Colony::new(5, 5);
        want.make_alive(2, 1);
        want.make_alive(2, 2);
        want.make_alive(2, 3);

        let mut got = starter.clone();
        process_frame(&mut got);
        println!("Want:\n{want}");
        println!("Got:\n{got}");
        assert_eq!(got, want);
    }

    #[test]
    fn still() {
        let mut want = Colony::new(5, 5);
        want.make_alive(1, 1);
        want.make_alive(1, 2);
        want.make_alive(2, 1);
        want.make_alive(2, 2);

        let mut got = want.clone();
        process_frame(&mut got);
        println!("Want:\n{want}");
        println!("Got:\n{got}");
        assert_eq!(got, want);
    }
}

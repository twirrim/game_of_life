pub mod consts;
pub mod structs;

extern crate ansi_term;
extern crate crossbeam_channel;

use crossbeam_channel::unbounded;
use rand::Rng;
use rayon::prelude::*;

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
    // Make some channels
    let (tx_alive, rx_alive) = unbounded();
    let (tx_dead, rx_dead) = unbounded();
    let (tx_reduce, rx_reduce) = unbounded();

    colony.cells.iter().enumerate().for_each(|(x, row)| {
        for (y, cell) in row.iter().enumerate() {
            if (cell.alive && cell.neighbours == 2) || cell.neighbours == 3 {
                tx_alive.send((x, y)).unwrap();
            } else if cell.alive {
                tx_dead.send((x, y)).unwrap();
            } else if !cell.alive && cell.life_left > 0 {
                tx_reduce.send((x, y)).unwrap();
            };
        }
    });
    drop(tx_alive);
    drop(tx_dead);
    drop(tx_reduce);

    // Update the cell state
    while let Ok((x, y)) = rx_reduce.recv() {
        colony.reduce_life(x, y);
    }
    while let Ok((x, y)) = rx_alive.recv() {
        colony.make_alive(x, y);
    }
    while let Ok((x, y)) = rx_dead.recv() {
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

        // Easiest way to deal with the life/decay stuff.  Walk through the process!
        let mut want = starter.clone();
        want.make_dead(1, 2);
        want.make_dead(3, 2);
        want.make_alive(2, 1);
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

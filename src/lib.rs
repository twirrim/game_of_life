pub mod consts;
pub mod structs;

extern crate ansi_term;
extern crate get_size;
extern crate get_size_derive;

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
    let mut to_live = vec![];
    let mut to_die = vec![];
    let mut to_reduce = vec![];

    colony.cells.iter().enumerate().for_each(|(x, row)| {
        for (y, cell) in row.iter().enumerate() {
            if (cell.alive && cell.neighbours == 2) || cell.neighbours == 3 {
                to_live.push((x, y));
            } else if cell.alive {
                to_die.push((x, y));
            } else if !cell.alive && cell.life_left > 0 {
                to_reduce.push((x, y));
            };
        }
    });
    // Update the cell state
    for (x, y) in to_reduce {
        colony.reduce_life(x, y);
    }
    for (x, y) in to_live {
        colony.make_alive(x, y);
    }
    for (x, y) in to_die {
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

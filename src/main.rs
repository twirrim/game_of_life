#[macro_use]
extern crate lazy_static;

use std::fs;
use std::sync::mpsc::sync_channel;
use std::thread;

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use ril::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

//3840x2160
const WIDTH: i32 = 3840;
const HEIGHT: i32 = 2160;
const OUTPUT_PATH: &str = "./output";
const FRAMES: u32 = 9000;

lazy_static! {
    static ref STARTING_CELLS: u32 = ((WIDTH as f32 * HEIGHT as f32) * 0.5) as u32;
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Cell {
    x: i32,
    y: i32,
}

fn main() {
    let (tx, rx) = sync_channel(100);

    let simulation = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        let starting_cells = ((WIDTH as f32 * HEIGHT as f32) * 0.5) as u32;

        println!("Starting with {:} cells", starting_cells);
        println!("Randomising starting cells");

        let mut active_cells = FxHashSet::default();
        for _ in (0..starting_cells).progress() {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            active_cells.insert(Cell { x: x, y: y });
        }

        let mut cells: FxHashSet<Cell> = active_cells.into_par_iter().collect();
        for frame in 0..FRAMES {
            cells = process_frame(&cells);
            tx.send((frame, cells.clone())).unwrap();
        }
    });

    let style = ProgressStyle::with_template(
        "[{elapsed_precise} / {eta_precise}] {wide_bar:40.cyan/blue} {pos:>7}/{len:7} {per_sec} {msg}",
    )
    .unwrap();

    fs::create_dir_all(OUTPUT_PATH).unwrap();
    let pb = ProgressBar::new(FRAMES as u64);
    pb.set_style(style.clone());

    for (frame, cells) in rx {
        pb.set_message(format!("Live cells: {}", cells.len()));

        let mut current_image = Image::new(WIDTH as u32, HEIGHT as u32, Rgb::black());
        for cell in &cells {
            current_image.set_pixel(cell.x as u32, cell.y as u32, Rgb::white());
        }
        current_image
            .save_inferred(format!("{OUTPUT_PATH}/{:08}.png", frame))
            .unwrap();
        pb.inc(1);
    }
    simulation.join().unwrap();
}

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

fn process_frame(active_cells: &FxHashSet<Cell>) -> FxHashSet<Cell> {
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

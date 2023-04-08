use rustc_hash::FxHashSet;

use std::fs;
use std::sync::mpsc::sync_channel;
use std::thread;

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use ril::prelude::*;

use gol::consts::{FRAMES, HEIGHT, OUTPUT_PATH, WIDTH};
use gol::process_frame;
use gol::structs::Cell;

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

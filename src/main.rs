use std::fs;
use std::sync::mpsc::sync_channel;
use std::thread;

use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use ril::prelude::*;

use gol::consts::{FRAMES, HEIGHT, OUTPUT_PATH, WIDTH};
use gol::{initialise, process_frame};

fn main() {
    let (tx, rx) = sync_channel(100);

    let simulation = thread::spawn(move || {
        let starting_cells = ((WIDTH as f32 * HEIGHT as f32) * 0.5) as u32;
        println!("Starting with {:} cells", starting_cells);
        println!("Randomising starting cells");

        let mut cells = initialise(starting_cells, WIDTH, HEIGHT);
        for frame in 0..FRAMES {
            process_frame(&mut cells);
            tx.send((frame, cells.clone())).unwrap();
        }
    });

    let style = ProgressStyle::with_template(
        "[{elapsed_precise} / {eta_precise}] {wide_bar:40.cyan/blue} {pos:>7}/{len:7} {per_sec} {msg}",
    )
    .unwrap();

    fs::create_dir_all(OUTPUT_PATH).unwrap();
    let pb = ProgressBar::new(FRAMES as u64);
    pb.set_style(style);

    // let mut output: Vec<&'static str> = rx.into_iter().par_bridge().collect();

    let collector = rx.into_iter().par_bridge();

    collector.for_each(|(frame, colony)| {
        let mut current_image = Image::new(WIDTH as u32, HEIGHT as u32, Rgb::black());
        for (x, row) in colony.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if cell.alive {
                    current_image.set_pixel(x as u32, y as u32, Rgb::white());
                }
            }
        }
        current_image
            .save_inferred(format!("{OUTPUT_PATH}/{:08}.png", frame))
            .unwrap();
        pb.inc(1);
    });

    /* for (frame, colony) in rx {
           let mut live_cells = 0;
           let mut current_image = Image::new(WIDTH as u32, HEIGHT as u32, Rgb::black());
           // Again, should be able to paralellise finding the live cells.
           for (x, row) in colony.cells.iter().enumerate() {
               for (y, cell) in row.iter().enumerate() {
                   if cell.alive {
                       live_cells += 1;
                       current_image.set_pixel(x as u32, y as u32, Rgb::white());
                   };
               }
           }

           current_image
               .save_inferred(format!("{OUTPUT_PATH}/{:08}.png", frame))
               .unwrap();
           pb.inc(1);
           pb.set_message(format!("Live cells: {}", live_cells));
       }
    */
    simulation.join().unwrap();
}

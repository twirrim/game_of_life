use std::fs;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::thread::available_parallelism;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use ril::prelude::*;

use gol::{initialise, process_frame};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    width: isize,
    #[arg(short, long)]
    height: isize,
    #[arg(short, long)]
    frames: usize,
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Cli::parse();
    let (tx, rx) = sync_channel(available_parallelism().unwrap().get());

    let simulation = thread::spawn(move || {
        println!("Creating initial colony");
        let starting_cells = ((args.width as f32 * args.height as f32) * 0.5) as usize;
        println!("Starting with {:} cells", starting_cells);
        println!("Randomising starting cells");

        let mut cells = initialise(starting_cells, args.width, args.height);
        println!("Colony initialised");
        for frame in 0..args.frames {
            process_frame(&mut cells);
            tx.send((frame, cells.clone())).unwrap();
        }
        drop(tx);
    });

    let style = ProgressStyle::with_template(
        "[{elapsed_precise} / {eta_precise}] {wide_bar:40.cyan/blue} {pos:>7}/{len:7} {per_sec} {msg}",
    )
    .unwrap();

    fs::create_dir_all(&args.output).unwrap();
    let pb = ProgressBar::new(args.frames as u64);
    pb.set_style(style);

    let collector = rx.into_iter().par_bridge();

    collector.for_each(|(frame, colony)| {
        let mut current_image = Image::new(args.width as u32, args.height as u32, Rgb::black());
        let mut live_cells = 0;
        for (x, row) in colony.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if cell.alive {
                    live_cells += 1;
                    current_image.set_pixel(x as u32, y as u32, Rgb::white());
                };
            }
        }
        current_image
            .save_inferred(format!("{}/{:08}.png", &args.output, frame))
            .unwrap();
        pb.inc(1);
        pb.set_message(format!("Live cells: {}", live_cells));
    });
    simulation.join().unwrap();
}

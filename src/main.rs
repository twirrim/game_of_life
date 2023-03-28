#[macro_use]
extern crate lazy_static;

use std::fs;

use indicatif::{
    MultiProgress, ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle,
};
use itertools::Itertools;
use rand::Rng;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use ril::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const OUTPUT_PATH: &str = "./output";
const FRAMES: u32 = 1500;

lazy_static! {
    static ref STARTING_CELLS: u32 = ((WIDTH as f32 * HEIGHT as f32) * 0.1) as u32;
}

fn main() {
    let mut rng = rand::thread_rng();

    fs::create_dir_all(OUTPUT_PATH).unwrap();
    println!("Starting with {:} cells", *STARTING_CELLS);
    let mut previous_image = Image::new(WIDTH, HEIGHT, Rgb::white());
    println!("Randomising starting cells");
    for _ in (0..*STARTING_CELLS).progress() {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);
        previous_image.set_pixel(x, y, Rgb::black());
    }
    previous_image
        .save_inferred(format!("{OUTPUT_PATH}/000.png"))
        .unwrap();
    // This is going to be an inefficient approach to start with.
    // For each frame, make a blank image.
    // Check each cell in the original image, and all the cells around it.
    // TODO: Instead of tracking this in the image, have a hashmap that just tracks known alive cells.

    // This is such a dumb idea..
    let cells: Vec<(u32, u32)> = (0..WIDTH)
        .cartesian_product(0..HEIGHT)
        .into_iter()
        .collect();

    let style = ProgressStyle::with_template(
        "[{elapsed_precise} / {eta_precise}] {wide_bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap();
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new(FRAMES as u64));
    pb.set_style(style.clone());
    let pb2 = m.insert_after(&pb, ProgressBar::new(cells.len() as u64));
    pb2.set_style(style.clone());

    m.println("Producing frames").unwrap();
    for frame in 1..FRAMES {
        pb.inc(1);
        let mut current_image = Image::new(WIDTH, HEIGHT, Rgb::white());

        let to_update: Vec<(u32, u32)> = cells
            .clone()
            .into_par_iter()
            .progress_with(pb2.clone())
            .filter(|cell| evaluate_cell(cell.0, cell.1, previous_image.clone()))
            .collect();
        pb2.reset();
        for cell in to_update {
            current_image.set_pixel(cell.0, cell.1, Rgb::black());
        }

        current_image
            .save_inferred(format!("{OUTPUT_PATH}/{:03}.png", frame))
            .unwrap();
        previous_image = current_image.clone();
    }
}

fn evaluate_cell(x: u32, y: u32, image: ril::Image<ril::Rgb>) -> bool {
    let mut neighbour_count = 0;
    let current_cell_lives = match image.get_pixel(x, y) {
        Some(pixel) => pixel.as_rgb() == Rgb::black(),
        None => false,
    };

    for x_offset in -1..=1 {
        for y_offset in -1..=1 {
            if !(x_offset == 0 && y_offset == 0) {
                // Going to assume any invalid pixel is outside of the range of the image
                // and is also dead
                let offset_pixel_alive: bool =
                    match image.get_pixel(x + x_offset as u32, y + y_offset as u32) {
                        Some(pixel) => pixel.as_rgb() == Rgb::black(),
                        None => false,
                    };
                if offset_pixel_alive {
                    neighbour_count += 1;
                };
            };
        }
    }

    // Any live cell with two or three live neighbours survives.
    // Any dead cell with three live neighbours becomes a live cell.
    // All other live cells die in the next generation. Similarly, all other dead cells stay dead.
    if current_cell_lives {
        if neighbour_count == 2 || neighbour_count == 3 {
            return true;
        };
    } else {
        if neighbour_count == 3 {
            return true;
        };
    };
    return false;
}

// #![deny(warnings)]
#![allow(mixed_script_confusables)] // allows unicode characters
// use std::{thread, time::Duration};
use stopwatch::Stopwatch;
mod hydrological_erosion;
mod terrain;

use crate::hydrological_erosion::erode;
use crate::terrain::Terrain;
fn main() {
    println!(">");
    let mut frame = Stopwatch::start_new();
    let mut terrain: Terrain = Terrain::new(11, 256, 256, 0, 2., 0.8);

    frame.stop();
    println!("frame: {:?}", frame.elapsed());
    println!(
        "instructions per cell: {:?}",
        (4.46 * 1_000_000_000.) / (1. / frame.elapsed().as_secs_f64()) / (512. * 512. * 25.)
    );
    terrain.height_to_image();
    terrain.normal_to_image();
    terrain.xnormal_to_image();
    terrain.ynormal_to_image();
    erode(&mut terrain);
    println!("done");
}

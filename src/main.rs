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
    let mut terrain: Terrain = Terrain::new(11, 4096, 4096, 0, 2., 0.8);

    frame.stop();
    println!("frame: {:?}", frame.elapsed());

    terrain.height_to_image();
    terrain.normal_to_image();
    terrain.xnormal_to_image();
    terrain.ynormal_to_image();
    erode(&mut terrain);
}

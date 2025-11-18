// #![deny(warnings)]
#![allow(mixed_script_confusables)] // allows unicode characters
// use std::{thread, time::Duration};
use stopwatch::Stopwatch;
mod terrain;
use crate::terrain::Terrain;

fn main() {
    let mut frame = Stopwatch::start_new();
    frame.stop();
    println!("frame: {:?}", frame.elapsed());
    let terrain: Terrain = Terrain::new(4, 512, 512, 3, 2., 2.1);
    terrain.height_to_image();
    terrain.normal_to_image();
    terrain.xnormal_to_image();
    terrain.ynormal_to_image();
}

// #![deny(warnings)]
#![allow(mixed_script_confusables)]
use nalgebra::ComplexField;
// allows unicode characters
// use std::{thread, time::Duration};
use stopwatch::Stopwatch;
mod hydrological_erosion;
mod terrain;

use crate::hydrological_erosion::erode;
use crate::terrain::Terrain;
fn main() {
    println!(">");
    let mut frame = Stopwatch::start_new();
    let mut terrain: Terrain = Terrain::new(11, 1024, 1024, 0, 2.1, 0.4);

    frame.stop();
    println!("noise generation time: {:?}", frame.elapsed());

    erode(&mut terrain);
}

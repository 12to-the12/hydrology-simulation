use crate::terrain::Terrain;
use rayon::prelude::*;
use std::sync::mpsc; // multiproducer, single consumer
// pub trait Drop {
//     fn drop(&self, _: &mut Terrain) -> ();
// }
#[derive(Debug)]
struct Particle {
    location: (usize, usize),
}

impl Particle {
    fn drop(terrain: &Terrain) -> () {
        let particle = Particle {
            location: terrain.random_location(),
        };
        // println!("{:?}", particle.location);
        // println!("{}", terrain.shape().0);
    }
}
fn fake_workload(terrain: &Terrain, sender: mpsc::Sender<(usize, usize)>) -> () {
    Particle::drop(&terrain);
    for _ in 0..1 {
        // thread::sleep(Duration::from_millis(myval as u64));
        sender.send(terrain.random_location()).unwrap();
    }
}
pub fn erode<'a>(terrain: &'a mut Terrain) {
    println!("eroding...");
    Particle::drop(terrain);
    let (sender, receiver) = mpsc::channel::<(usize, usize)>();

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(10)
        .build()
        .unwrap();
    for _ in 0..65_000 {
        let mysender = sender.clone();
        pool.install(|| {
            fake_workload(&terrain, mysender);
        });
    }
    drop(sender);

    for received in receiver {
        println!("({},{})", received.0, received.1);
        terrain.set_height(0.5, received.0, received.1);
    }
    terrain.height_to_image();
    // let handle = thread::spawn(move || {
    //     fake_workload(sender, 77);
    // });
    // let handle = thread::spawn(move || {
    //     fake_workload(sender1, 88);
    // });
    // for received in receiver {
    //     println!("({},{})", received.0, received.1);
    // }
    // println!("we're done here");
    // handle.join().unwrap()
}

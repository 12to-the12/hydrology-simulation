use crate::terrain::{self, Terrain};
use rand::{SeedableRng, distr::Map};
use rayon::prelude::*;
use std::sync::mpsc;
use stopwatch::Stopwatch; // multiproducer, single consumer
#[derive(Debug)]
struct Particle {
    coor: (f32, f32),
    age: usize,
}
impl Particle {
    pub fn new(coor: Coor) -> Particle {
        Particle {
            coor: (coor.0 as f32, coor.1 as f32),
            age: 0,
        }
    }
    fn drop(terrain: &Terrain, coor: Coor) -> Vec<MapUpdate> {
        let mut particle = Particle::new(coor);
        let mut out = Vec::new();

        while particle.age < 100 && terrain.in_bounds(coor) {
            particle.age()
        }
        out.push((0., particle.coor()));
        return out;
    }
    pub fn age(&mut self) {
        self.age += 1;
    }
    pub fn coor(&self) -> Coor {
        (self.coor.0 as usize, self.coor.1 as usize)
    }
}
type Coor = (usize, usize);
type MapUpdate = (f32, Coor);
fn rain_on_me(terrain: &Terrain, sender: mpsc::Sender<Vec<MapUpdate>>, droplets: i32) -> () {
    let mut threadtimer = Stopwatch::start_new();
    let mut out: Vec<MapUpdate> = Vec::new();
    let mut rng = rand::rngs::SmallRng::from_os_rng();

    for _ in 0..droplets {
        let coor: Coor = terrain.random_location(&mut rng);
        out.extend(Particle::drop(&terrain, coor));
        let update: MapUpdate = (32., coor);
        out.push(update);
    }
    sender.send(out).unwrap();
    threadtimer.stop();

    println!("threadtime: {:?}", threadtimer.elapsed());
}
pub fn erode<'a>(terrain: &'a mut Terrain) {
    println!("eroding...");

    let (sender, receiver) = mpsc::channel::<Vec<MapUpdate>>();
    let mut timer = Stopwatch::start_new();
    println!("making pool...");
    (0..10).into_par_iter().for_each(|_| {
        let mysender = sender.clone();
        rain_on_me(&terrain, mysender, 10_000);
    });
    println!("left scope");
    drop(sender);
    timer.stop();

    println!("allocation: {:?}", timer.elapsed());
    let mut timer = Stopwatch::start_new();

    for received in receiver {
        for (value, coor) in received {
            terrain.set_height(value, coor.0, coor.1);
        }
    }

    timer.stop();

    println!("execution: {:?}", timer.elapsed());

    terrain.height_to_image();
}

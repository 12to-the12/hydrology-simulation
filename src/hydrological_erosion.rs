use crate::terrain::Terrain;
use crossbeam::channel;
use nalgebra::SVector;
use rand::SeedableRng;
use rayon::prelude::*;
use std::thread;
use stopwatch::Stopwatch; // multiproducer, single consumer
type Vec2 = SVector<f32, 2>;

#[derive(Debug)]
struct Particle {
    coor: Vec2,
    starting_coor: Coor,
    age: f32,
    _volume: f32,
    _velocity: Vec2,
    relative_sediment: f32,
}

type Coor = (usize, usize);
type MapUpdate = (f32, Coor);
impl Particle {
    pub fn new(coor: Coor) -> Particle {
        Particle {
            coor: Vec2::new(coor.0 as f32, coor.1 as f32),
            starting_coor: (0, 0),
            age: 0.,
            _volume: 0.,
            _velocity: Vec2::new(0., 0.),
            relative_sediment: 0.,
        }
    }
    fn drop(terrain: &Terrain, coor: Coor) -> Vec<MapUpdate> {
        let mut particle = Particle::new(coor);
        let mut out = Vec::new();
        let mut iterations = 0;
        while particle.age < 5_000. && terrain.in_bounds(particle.coor) {
            let starting_height = terrain.height[particle.coor()];
            particle.save_starting_coor();
            let normal: Vec2 = terrain.get_normal_2D(particle.coor());
            // particle._velocity = normal;
            particle._velocity += normal;
            particle.coor += particle._velocity.normalize();
            if !terrain.in_bounds(particle.coor) {
                break;
            };
            let height = terrain.height[particle.coor()];
            let Δheight = starting_height - height;
            // println!("Δheight: {}", Δheight);
            if Δheight < 0. {
                // println!("iterations: {}", iterations);
                particle._velocity *= 0.1
            }

            out.push((starting_height - (Δheight / 2.), particle.starting_coor));
            // out.push((starting_height - (Δheight / 2.), particle.starting_coor));
            // out.push((height + (Δheight / 2.), particle.coor()));
            // particle.age_up(1.);
            particle.age_up(1. / particle._velocity.magnitude());
            iterations += 1;
        }

        return out;
    }
    pub fn age_up(&mut self, time: f32) {
        self.age += time;
    }
    pub fn coor(&self) -> Coor {
        (self.coor[0] as usize, self.coor[1] as usize)
    }
    pub fn save_starting_coor(&mut self) -> () {
        self.starting_coor = (self.coor[0] as usize, self.coor[1] as usize)
    }
}
fn rain_on_me(terrain: &Terrain, sender: channel::Sender<Vec<MapUpdate>>, droplets: i32) -> () {
    let mut threadtimer = Stopwatch::start_new();
    let mut out: Vec<MapUpdate> = Vec::new();
    let mut rng = rand::rngs::SmallRng::from_os_rng();

    for _ in 0..droplets {
        let coor: Coor = terrain.random_location(&mut rng);
        out.extend(Particle::drop(&terrain, coor));
    }
    sender.send(out).unwrap();
    threadtimer.stop();

    // println!("threadtime: {:?}", threadtimer.elapsed());
}
pub fn erode<'a>(mut terrain: &'a mut Terrain) {
    println!("eroding...");
    loop {
        let mut timer = Stopwatch::start_new();

        // println!("making pool...");
        let workload = 100_000;
        let segments = 32;
        let (sender, receiver) = channel::bounded::<Vec<MapUpdate>>(segments as usize);

        let workload_per_segment = workload / segments;

        // let mut myterrain = terrain.clone();
        // let receiver_thread = thread::spawn(move || {
        //     for received in receiver {
        //         // println!("received");
        //         for (value, coor) in received {
        //             // println!("from {:?},{:?}", terrain.height[coor], coor);
        //             // println!("to {:?},{:?}\n\n", value, coor);

        //             myterrain.height[coor] = value;
        //             myterrain.stream_map[coor] += 1.;
        //         }
        //     }
        // });
        // println!("there");

        (0..segments).into_par_iter().for_each(|_| {
            // println!("started...");
            let mysender = sender.clone();
            rain_on_me(&terrain, mysender, workload_per_segment);
        });
        // terrain.height = myterrain.height;
        // terrain.stream_map = myterrain.stream_map;
        drop(sender);
        // let _ = receiver_thread.join();
        timer.stop();
        //
        println!("allocation time: {:?}", timer.elapsed());
        // println!("{:?},{:?}", segments, timer.elapsed().as_millis());
        let mut timer = Stopwatch::start_new();
        for received in receiver {
            // println!("received");
            for (value, coor) in received {
                // println!("from {:?},{:?}", terrain.height[coor], coor);
                // println!("to {:?},{:?}\n\n", value, coor);
                let Δheight = (terrain.height[coor] - value).abs();
                terrain.set_height(value, coor.0, coor.1);
                terrain.stream_map[coor] += Δheight;
            }
        }

        // let receiver: Vec<Vec<MapUpdate>> = receiver.into_iter().par_bridge().collect();

        timer.stop();

        println!("write time: {:?}", timer.elapsed());
        let mut timer = Stopwatch::start_new();
        // terrain.stream_map *= 0.95;
        terrain.compute_normals();
        terrain.render_all_images();

        timer.stop();

        println!("render time: {:?}", timer.elapsed());
        println!();
    }
}

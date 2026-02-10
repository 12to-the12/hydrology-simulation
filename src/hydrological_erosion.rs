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
    starting_height: f32,
    age: f32,
    volume: f32,
    velocity: Vec2,
    sediment: f32, // sediment carrid in cubic meters
}

type Coor = (usize, usize);
type MapUpdate = (f32, Coor);
impl Particle {
    pub fn new(coor: Coor) -> Particle {
        Particle {
            coor: Vec2::new(coor.0 as f32, coor.1 as f32),
            starting_coor: (0, 0),
            starting_height: 0.,
            age: 0.,
            volume: 10.,
            velocity: Vec2::new(0., 0.),
            sediment: 0.,
        }
    }
    fn drop(terrain: &Terrain, coor: Coor) -> Vec<MapUpdate> {
        let mut particle = Particle::new(coor);
        let mut out = Vec::new();
        let mut iterations = 0;
        particle.save_starting_state(terrain.height[particle.coor()]);

        particle.starting_height = terrain.height[particle.coor()];
        while terrain.in_bounds(particle.coor) {
            let normal: Vec2 = terrain.get_normal_2D(particle.coor());
            particle.velocity += normal;
            let dt = 1. / particle.velocity.magnitude();
            particle.coor += particle.velocity.normalize();
            if !terrain.in_bounds(particle.coor) {
                break;
            };
            let height = terrain.height[particle.coor()];
            // Δheight is negative if it drops
            let Δheight = height - particle.starting_height;
            let intensity = 1.; // 1 is fully equalized

            // full soak saturates?
            let mut carrying_capacity = particle.speed() * -Δheight * particle.volume;
            // println!("{}", -Δheight);


            
            carrying_capacity = carrying_capacity.max(0.);
            let soaking_force = carrying_capacity - particle.sediment;
            if soaking_force > 0. {
                // grab dirt
                let picked_up = dt * soaking_force * 1.0;
                particle.sediment += picked_up;
                out.push((-picked_up, particle.coor()));
            } else {
                // drop dirt
                let dropped = particle.sediment - carrying_capacity;
                particle.sediment -= dropped;
                out.push((dropped, particle.coor()));
            }
            // let mass_transfer = intensity * Δheight * 0.5;

            if Δheight >= 0. {
                break;
            }

            // if iterations == 0 {
            //     out.push((mass_transfer, particle.starting_coor));
            // } else {
            //     let last = out.last_mut().unwrap();
            //     *last = (last.0 + mass_transfer, last.1);
            // }

            // out.push((-mass_transfer, particle.coor()));

            // particle.save_starting_state(height - mass_transfer);

            // particle.age_up(1. / particle.velocity.magnitude());

            particle.volume *= (1_f32 - 0.1).powf(dt);
            particle.velocity *= (1_f32 - 0.1).powf(dt);
            iterations += 1;
            if particle.volume < 1e-1 {
                out.push((particle.sediment, particle.coor()));
                break;
            }
        }

        return out;
    }
    pub fn age_up(&mut self, time: f32) {
        self.age += time;
    }
    pub fn coor(&self) -> Coor {
        (self.coor[0] as usize, self.coor[1] as usize)
    }

    pub fn speed(&self) -> f32 {
        self.velocity.magnitude()
    }
    pub fn save_starting_state(&mut self, height: f32) -> () {
        self.starting_height = height;
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
pub fn erode<'a>(terrain: &'a mut Terrain) {
    println!("eroding...");
    loop {
        println!("max: {}", terrain.height.max());
        println!("min: {}", terrain.height.min());
        for _ in 0..10 {
            let mut timer = Stopwatch::start_new();

            // println!("making pool...");
            let mut get_time = Stopwatch::start_new();
            let workload = 10_000;
            let segments = 1_000;
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
            // println!(">  allocation time: {:?}", timer.elapsed());
            // println!("{:?},{:?}", segments, timer.elapsed().as_millis());
            let mut timer = Stopwatch::start_new();
            let mut total = 0;
            let mut total_change = 0.;
            for received in receiver {
                total += received.len();

                // println!("received");
                for (value, coor) in received {
                    // println!("from {:?},{:?}", terrain.height[coor], coor);
                    // println!("to {:?},{:?}\n\n", value, coor);

                    let Δheight = value;
                    // let Δheight = value - terrain.height[coor];

                    total_change += Δheight;
                    // terrain.height[coor] += Δheight;
                    // terrain.height[coor] = terrain.height[coor] + Δheight;
                    terrain.height[coor] += Δheight;

                    // println!("{:?}, {:?}", coor, Δheight);
                    terrain.Δheight[coor] += Δheight;
                    terrain.stream_map[coor] += 1.;
                    // terrain.stream_map[coor] += Δheight.abs();

                    // if (terrain.height[coor] < 0.) {
                    //     println!("{:?}", coor);
                    //     // println!(
                    //     //     "{:?}, delta: {}, new height: {}",
                    //     //     (value, coor),
                    //     //     Δheight,
                    //     //     terrain.height[coor]
                    //     // );
                    //     // panic!()
                    // }
                }
            }
            // println!("total writes: {}", total);
            // println!("change in height written: {}", total_change);

            // let receiver: Vec<Vec<MapUpdate>> = receiver.into_iter().par_bridge().collect();

            terrain.compute_normals();
            timer.stop();
            // println!(">  write time: {:?}", timer.elapsed());
            get_time.stop();
            // println!(
            //     "droplets per ms: {:?}",
            //     workload as u128 / get_time.elapsed().as_millis()
            // );
        }

        let mut timer = Stopwatch::start_new();
        terrain.stream_map *= 0.95;
        terrain.render_all_images();

        timer.stop();

        println!("render time: {:?}\n\n", timer.elapsed());
        println!();
    }
}

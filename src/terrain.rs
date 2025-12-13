extern crate nalgebra as na;
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use na::DMatrix;
use nalgebra::{Matrix3x1, SVector};
use noise::{NoiseFn, Simplex};
use rand::{Rng, rngs::SmallRng};

use rayon::prelude::*;
pub fn save_image(canvas: ImageBuffer<Rgb<u8>, Vec<u8>>, name: &str) -> () {
    canvas
        .save_with_format("pictures/".to_owned() + name + ".png", ImageFormat::Png)
        .unwrap();
}
type Vec3 = SVector<f32, 3>;
type Vec2 = SVector<f32, 2>;
#[derive(Debug, Clone)]
pub struct Terrain {
    pub height: DMatrix<f32>,
    _surface_water: DMatrix<f32>,
    pub normal: DMatrix<Vec3>,
    pub stream_map: DMatrix<f32>,
    // _temperature: DMatrix<f32>, // Kelvin
    // _oxygen: DMatrix<f32>,
    // _fixed_nitrogen: DMatrix<f32>,
    // _humidity: DMatrix<f32>, // %
    // _wind: DMatrix<Vec2>,
    // _volume: DMatrix<f32>,
    // _volume_acc: DMatrix<f32>,
    // _impact: DMatrix<f32>,
    // _hydraulic_momentum_acc: DMatrix<2Vec>,
    // _hydraulic_momentum: DMatrix<2Vec>,
    // _atmospheric_water: DMatrix<f32>,
    // rng: rand::rngs::SmallRng,
}
impl Default for Terrain {
    fn default() -> Terrain {
        Terrain::brownian_terrain(1, 64, 64, 1, 4., 4.)
    }
}
impl Terrain {
    pub fn new(
        seed: usize,
        rows: usize,
        columns: usize,
        octaves: usize,
        lacunarity: f32,
        persistence: f32,
    ) -> Terrain {
        let mut out =
            Terrain::brownian_terrain(seed, rows, columns, octaves, lacunarity, persistence);
        out.compute_normals();
        return out;
    }
    pub fn set_height(&mut self, value: f32, x: usize, y: usize) {
        self.height[(x, y)] = value
    }
    pub fn get_normal_2D(&self, coor: (usize, usize)) -> Vec2 {
        Vec2::new(self.normal[coor][0], self.normal[coor][1])
    }
    #[allow(nonstandard_style)]
    pub fn compute_normals(&mut self) -> () {
        for i in 1..(self.shape().0 - 1) {
            for j in 1..(self.shape().1 - 1) {
                // println!("{:?}", (i, j));
                let center = self.height[(i, j)];
                let Δleft = self.height[(i - 1, j)] - center;
                let Δright = self.height[(i + 1, j)] - center;
                let Δup = self.height[(i, j - 1)] - center;
                let Δdown = self.height[(i, j + 1)] - center;

                let topleft = Matrix3x1::new(-1., 0., Δleft).cross(&Matrix3x1::new(0., -1., Δup));
                let topright = Matrix3x1::new(0., -1., Δup).cross(&Matrix3x1::new(1., 0., Δright));
                let bottomleft =
                    Matrix3x1::new(0., 1., Δdown).cross(&Matrix3x1::new(-1., 0., Δleft));
                let bottomright =
                    Matrix3x1::new(1., 0., Δright).cross(&Matrix3x1::new(0., 1., Δdown));
                let normal: Vec3 = (topleft.normalize()
                    + topright.normalize()
                    + bottomleft.normalize()
                    + bottomright.normalize())
                .normalize()
                .into();
                // println!("{:?}", normal);
                // topleft = [-1.0, 0.0, left-center].cross([0.0, -1.0, up-center])
                // topright = [0.0, -1.0, up-center].cross([1.0, 0.0, right-center])
                // bottomleft = [0.0, 1.0, down-center].cross([-1.0, 0.0, left-center])
                // bottomright = [1.0, 0.0, right-center].cross([0.0, 1.0, down-center])
                self.normal[(i, j)] = normal;
            }
        }
    }
    fn blank(rows: usize, columns: usize) -> Self {
        Terrain {
            height: DMatrix::<f32>::zeros(rows, columns),
            stream_map: DMatrix::<f32>::zeros(rows, columns),
            _surface_water: DMatrix::<f32>::zeros(rows, columns),
            normal: DMatrix::<Vec3>::from_element(rows, columns, Vec3::new(0., 0., 0.)),
            // rng: rand::rngs::SmallRng::from_os_rng(),
        }
    }
    pub fn shape(&self) -> (usize, usize) {
        return self.height.shape();
    }
    fn width(&self) -> usize {
        self.shape().0
    }
    fn height(&self) -> usize {
        self.shape().1
    }

    #[inline(always)]
    fn brownian_terrain(
        seed: usize,
        rows: usize,
        columns: usize,
        _: usize,
        lacunarity: f32,
        persistence: f32,
    ) -> Terrain {
        // let noisefunc = Simplex::new(seed as u32);

        let mut terrain = Terrain::blank(rows, columns);
        let octaves = (rows as f32).log(lacunarity).ceil() as usize + 1;
        // let octaves = 1;
        // (0..octaves).into_par_iter()
        for octave in 0..octaves {
            let noisefunc = Simplex::new((seed + octave) as u32);
            let frequency = (lacunarity as f32).powf(octave as f32) / (rows as f32);
            let height = DMatrix::<f32>::from_fn(rows, columns, |x, y| {
                noisefunc.get([
                    ((x as f32) * frequency) as f64,
                    ((y as f32) * frequency) as f64,
                ]) as f32
            });
            terrain.height += height * persistence.powf(octave as f32);
        }

        // normalize to [0,1]
        terrain.height -= DMatrix::<f32>::from_element(rows, columns, terrain.height.min());
        terrain.height /= terrain.height.max();

        // println!("{:?}", terrain.height.sum() / ((rows * columns) as f32));
        // terrain.height /= terrain.height.sum();
        terrain.height *= terrain.shape().0 as f32; // usually starts around 0.5

        terrain
    }
    fn value_to_image<F: Fn(u32, u32) -> Rgb<u8> + std::marker::Sync + std::marker::Send>(
        &self,
        value: F,
        name: &str,
    ) -> () {
        // let mut canvas: RgbImage = ImageBuffer::new(self.shape().0 as u32, self.shape().1 as u32);
        // for i in 0..self.shape().0 {
        //     for j in 0..self.shape().1 {
        //         // let pixel = value(i, j) * 256.;
        //         // println!("{:?}", value(i, j));
        //         canvas.put_pixel(i as u32, j as u32, value(i as u32, j as u32));
        //     }
        // }
        let canvas = ImageBuffer::from_par_fn(self.width() as u32, self.height() as u32, value);

        save_image(canvas, name);
    }
    pub fn render_all_images(&self) {
        self.height_to_image();
        self.stream_map_to_image();
        self.normal_to_image();
        self.xnormal_to_image();
        self.ynormal_to_image();
        self.pretty_to_image();
    }
    pub fn height_to_image(&self) -> () {
        self.value_to_image(
            |i, j| {
                Rgb([
                    (self.height[(i as usize, j as usize)] / self.width() as f32 * 255.) as u8,
                    (self.height[(i as usize, j as usize)] / self.width() as f32 * 255.) as u8,
                    (self.height[(i as usize, j as usize)] / self.width() as f32 * 255.) as u8,
                ])
            },
            "height",
        )
    }
    pub fn stream_map_to_image(&self) -> () {
        let mut render_map = self.stream_map.clone();
        let max = render_map.max().ln();
        // render_map /= render_map.max().ln();
        self.value_to_image(
            |i, j| {
                Rgb([
                    (render_map[(i as usize, j as usize)].ln() / max * 255.) as u8,
                    0 as u8,
                    0 as u8,
                ])
            },
            "stream_map",
        );
    }
    pub fn normal_to_image(&self) -> () {
        let sunlight = Vec3::new(1., 0.5, 0.);
        self.value_to_image(
            |i, j| {
                let normal = self.normal[(i as usize, j as usize)];
                let value = (normal.dot(&sunlight) * 256.) as u8;
                Rgb([value, value, value])
            },
            "normal",
        )
    }
    pub fn pretty_to_image(&self) -> () {
        let sunlight = Vec3::new(1., 0.5, 0.);
        let render_map = self.stream_map.clone();
        let max = render_map.max().ln();

        self.value_to_image(
            |i, j| {
                let normal = self.normal[(i as usize, j as usize)];
                let lighting = normal.dot(&sunlight) * 256.;
                let value =
                    lighting * (self.height[(i as usize, j as usize)] / self.shape().0 as f32);
                let water = render_map[(i as usize, j as usize)].ln() / max * 256.;
                Rgb([value as u8, value as u8, value.max(water) as u8])
            },
            "pretty",
        )
    }
    pub fn xnormal_to_image(&self) -> () {
        self.value_to_image(
            |i, j| {
                Rgb([
                    ((self.normal[(i as usize, j as usize)][0]) * 256.) as u8,
                    0,
                    -((self.normal[(i as usize, j as usize)][0]) * 256.) as u8,
                ])
            },
            "x",
        )
    }
    pub fn ynormal_to_image(&self) -> () {
        self.value_to_image(
            |i, j| {
                Rgb([
                    ((self.normal[(i as usize, j as usize)][1]) * 256.) as u8,
                    0,
                    -((self.normal[(i as usize, j as usize)][1]) * 256.) as u8,
                ])
            },
            "y",
        )
    }
    pub fn random_location(&self, rng: &mut SmallRng) -> (usize, usize) {
        (
            rng.random_range(0..self.shape().0),
            rng.random_range(0..self.shape().1),
        )
    }
    pub fn in_bounds(&self, coor: SVector<f32, 2>) -> bool {
        coor[0] >= 0.
            && coor[1] >= 0.
            && coor[0] < self.shape().0 as f32
            && coor[1] < self.shape().1 as f32
    }
}

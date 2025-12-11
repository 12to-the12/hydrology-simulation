extern crate nalgebra as na;
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use na::DMatrix;
use nalgebra::{ComplexField, Matrix3x1, SVector};
use noise::{NoiseFn, Seedable, Simplex};
use rand::Rng;
pub fn save_image(canvas: ImageBuffer<Rgb<u8>, Vec<u8>>, name: &str) -> () {
    canvas
        .save_with_format("pictures/".to_owned() + name + ".png", ImageFormat::Png)
        .unwrap();
}
type Vec3 = SVector<f32, 3>;
#[derive(Debug)]
pub struct Terrain {
    height: DMatrix<f32>,
    _surface_water: DMatrix<f32>,
    normal: DMatrix<Vec3>,
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
    #[allow(nonstandard_style)]
    fn compute_normals(&mut self) -> () {
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
            _surface_water: DMatrix::<f32>::zeros(rows, columns),
            normal: DMatrix::<Vec3>::from_element(rows, columns, Vec3::new(0., 0., 0.)),
        }
    }
    pub fn shape(&self) -> (usize, usize) {
        return self.height.shape();
    }
    fn width(&self) -> usize {
        self.shape().0
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
    fn value_to_image<F: Fn(usize, usize) -> Rgb<u8>>(&self, value: F, name: &str) -> () {
        let mut canvas: RgbImage = ImageBuffer::new(self.shape().0 as u32, self.shape().1 as u32);
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                // let pixel = value(i, j) * 256.;
                // println!("{:?}", value(i, j));
                canvas.put_pixel(i as u32, j as u32, value(i, j));
            }
        }
        save_image(canvas, name);
    }
    pub fn height_to_image(&self) -> () {
        self.value_to_image(
            |i, j| {
                Rgb([
                    (self.height[(i, j)] / self.width() as f32 * 255.) as u8,
                    (self.height[(i, j)] / self.width() as f32 * 255.) as u8,
                    (self.height[(i, j)] / self.width() as f32 * 255.) as u8,
                ])
            },
            "height",
        )
    }
    pub fn normal_to_image(&self) -> () {
        self.value_to_image(
            |i, j| {
                Rgb([
                    (self.normal[(i, j)][0] * 256.) as u8,
                    0,
                    (self.normal[(i, j)][1] * 256.) as u8,
                ])
            },
            "normal",
        )
    }
    pub fn xnormal_to_image(&self) -> () {
        self.value_to_image(
            |i, j| {
                Rgb([
                    ((self.normal[(i, j)][0]) * 256.) as u8,
                    0,
                    -((self.normal[(i, j)][0]) * 256.) as u8,
                ])
            },
            "x",
        )
    }
    pub fn ynormal_to_image(&self) -> () {
        self.value_to_image(
            |i, j| {
                Rgb([
                    ((self.normal[(i, j)][1]) * 256.) as u8,
                    0,
                    -((self.normal[(i, j)][1]) * 256.) as u8,
                ])
            },
            "y",
        )
    }
    pub fn random_location(&self) -> (usize, usize) {
        (
            rand::rng().random_range(0..self.shape().0),
            rand::rng().random_range(0..self.shape().1),
        )
    }
}

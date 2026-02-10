use nalgebra::{DMatrix, SVector};
use rayon::prelude::*;
use std::ops::{Index, IndexMut, MulAssign};
use std::sync::Mutex;
type Vec3 = SVector<f32, 3>;
type Vec2 = SVector<f32, 2>;

type Grid<T> = Vec<Vec<T>>;

#[derive(Debug)]
struct FloatTiles {
    contents: Grid<Mutex<DMatrix<f32>>>,
    matrixshape: (usize, usize),
}
impl FloatTiles {
    fn tileshape(&self) -> (usize, usize) {
        return (self.contents.len(), self.contents.first().unwrap().len());
    }
    fn matrixshape(&self) -> (usize, usize) {
        return self.matrixshape;
    }
    fn shape(&self) -> (usize, usize) {
        let tileshape = self.tileshape();
        let matrixshape = self.matrixshape();
        return (
            tileshape.0 * self.matrixshape().0,
            tileshape.1 * self.matrixshape().1,
        );
    }
    fn matrixindex(&self, index: &(usize, usize)) -> (usize, usize) {
        return (
            index.0 % self.matrixshape().0,
            index.1 % self.matrixshape().1,
        );
    }
    fn tileindex(&self, index: &(usize, usize)) -> (usize, usize) {
        return (
            index.0 / self.matrixshape().0,
            index.1 / self.matrixshape().1,
        );
    }
    fn reserve_tile()
}

#[derive(Debug)]
struct VecTiles(Grid<Mutex<DMatrix<Vec3>>>);

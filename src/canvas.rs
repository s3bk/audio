use std::ops::{Index, IndexMut};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::io::{BufWriter, BufReader};
use std::fs::File;
use rayon::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Canvas {
    data: Vec<f32>,
    width: usize,
    height: usize
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            data: vec![std::f32::NAN; width * height],
            width,
            height
        }
    }
    fn index_of(&self, (x, y): (usize, usize)) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);
        y * self.width + x
    }
    pub fn values(&self) -> impl Iterator<Item=f32> + '_ {
        self.data.iter().cloned()
    }
    pub fn row(&self, y: usize) -> impl Iterator<Item=f32> + '_ {
        let start = y * self.width;
        let end = start + self.width;
        self.data[start .. end].iter().cloned()
    }
    pub fn par_rows_mut(&mut self) -> impl ParallelIterator<Item=&mut [f32]> + IndexedParallelIterator {
        self.data.par_chunks_mut(self.width)
    }
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    pub fn save(&self, path: impl AsRef<Path>) {
        let file = File::create(path).expect("can't create file");
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self).expect("can't serialize");
    }
    pub fn load(path: impl AsRef<Path>) -> Self {
        let file = File::open(path).expect("can't open file");
        let reader = BufReader::new(file);
        bincode::deserialize_from(reader).expect("can't deserialize")
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = f32;
    fn index(&self, idx: (usize, usize)) -> &f32 {
        let idx = self.index_of(idx);
        &self.data[idx]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut f32 {
        let idx = self.index_of(idx);
        &mut self.data[idx]
    }
}

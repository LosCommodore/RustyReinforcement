#![expect(dead_code)]

use anyhow::{Result, bail};
use std::ops::{Index, IndexMut};

pub struct Matrix<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Copy + Default> Matrix<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, y: usize, x: usize) -> Option<&T> {
        let i = y * self.width + x;
        self.data.get(i)
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    pub fn from_vec(data: Vec<T>, width: usize, height: usize) -> Result<Self> {
        let w = data.len() / height;
        let remainder = data.len() % height;
        if w != width || remainder != 0 {
            bail!("wrong dimensions")
        }

        Ok(Self {
            width,
            height,
            data,
        })
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        &mut self.data[y * self.width + x]
    }
}

use std::ops::{Index, IndexMut};

pub struct Array2D<T: Copy> {
    width: usize,
    height: usize,
    buf: Vec<T>,
}

impl<T: Copy> Array2D<T> {
    pub fn new(width: usize, height: usize, initial_value: T) -> Self {
        Self {
            width,
            height,
            buf: vec! [initial_value; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.buf[self.width * y + x])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(&mut self.buf[self.width * y + x])
        } else {
            None
        }
    }
}

impl<T: Copy> Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1).unwrap()
    }
}

impl<T: Copy> IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_array_of_initial_value() {
        let arr = Array2D::new(19, 19, 7i32);

        for x in 0..19 {
            for y in 0..19 {
                assert_eq!(arr.get(x, y), Some(&7i32));
            }
        }
    }
}

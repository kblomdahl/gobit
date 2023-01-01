pub struct Ring<T: Copy + PartialEq, const N: usize> {
    elements: [T; N],
    position: usize,
}

impl<T: Copy + PartialEq, const N: usize> Ring<T, N> {
    pub fn new(initial_value: T) -> Self {
        Self {
            elements: [initial_value; N],
            position: 0,
        }
    }

    pub fn contains(&self, value: T) -> bool {
        self.elements.contains(&value)
    }

    pub fn insert(&mut self, value: T) {
        self.elements[self.position] = value;
        self.position = (self.position + 1) % N;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_keeps_n_elements() {
        let mut ring = Ring::<u32, 3>::new(0);
        ring.insert(1);
        ring.insert(2);
        ring.insert(3);
        ring.insert(4);

        assert!(!ring.contains(1));
        assert!(ring.contains(2));
        assert!(ring.contains(3));
        assert!(ring.contains(4));
    }
}

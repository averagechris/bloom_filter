extern crate bit_vec;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use bit_vec::BitVec;


#[derive(Debug)]
pub struct BloomFilter {
    elements: BitVec,
    size: usize,
}

impl BloomFilter {
    pub fn new(size: usize) -> BloomFilter {
        let elements = BitVec::from_elem(size, false);
        BloomFilter { elements, size }
    }

    pub fn insert<T: Hash>(&mut self, item: &T) {
        let index = self.hash(item);
        self.elements.set(index, true);
    }

    pub fn insert_each<T: Hash>(&mut self, v: &Vec<T>) {
        for item in v.iter() {
            self.insert(item)
        }
    }

    pub fn contains<T: Hash>(&mut self, item: &T) -> bool {
        let index = self.hash(item);
        self.elements.get(index).unwrap_or(false)
    }

    fn hash<T: Hash>(&mut self, item: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        hasher.finish() as usize % self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bf_elements_are_zero_initialized_and_length_of_size_passed_to_constructor() {
        let size = 10;
        let bf = BloomFilter::new(size);

        assert_eq!(bf.elements.len(), size);
        assert_eq!(bf.elements.iter().filter(|&b| b).count(), 0);
    }

    #[test]
    fn hasher_doesnt_suck() {
        let mut bf = BloomFilter::new(5);

        assert_eq!(bf.hash(&12), bf.hash(&12));
    }
}

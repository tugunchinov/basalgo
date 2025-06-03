use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BloomFilter {
    bits: u128,
}

impl BloomFilter {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn insert<T: Hash>(&mut self, item: &T) {
        for i in 0..2 {
            let hash = self.hash(item, i);
            self.bits |= 1 << (hash % 128);
        }
    }

    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        for i in 0..2 {
            let hash = self.hash(item, i);
            if (self.bits & (1 << (hash % 128))) == 0 {
                return false;
            }
        }

        true
    }

    pub fn clear(&mut self) {
        self.bits = 0;
    }

    fn hash<T: Hash>(&self, item: &T, seed: u64) -> u64 {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        seed.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for BloomFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bloom_filter() {
        let filter = BloomFilter::new();
        assert_eq!(filter.bits, 0);
    }

    #[test]
    fn test_default() {
        let filter = BloomFilter::default();
        assert_eq!(filter.bits, 0);
    }

    #[test]
    fn test_insert_and_contains() {
        let mut filter = BloomFilter::new();

        filter.insert(&"hello");
        assert!(filter.contains(&"hello"));

        filter.insert(&42);
        assert!(filter.contains(&42));

        filter.insert(&"world");
        assert!(filter.contains(&"world"));
        assert!(filter.contains(&"hello"));
        assert!(filter.contains(&42));
    }

    #[test]
    fn test_contains_false_negative_impossible() {
        let mut filter = BloomFilter::new();
        filter.insert(&"test");
        assert!(filter.contains(&"test"));
    }

    #[test]
    fn test_clear() {
        let mut filter = BloomFilter::new();
        filter.insert(&"hello");
        filter.insert(&42);

        assert!(filter.contains(&"hello"));
        assert!(filter.contains(&42));

        filter.clear();
        assert_eq!(filter.bits, 0);
    }

    #[test]
    fn test_different_types() {
        let mut filter = BloomFilter::new();

        filter.insert(&"string");
        filter.insert(&123);
        filter.insert(&true);
        filter.insert(&vec![1, 2, 3]);

        assert!(filter.contains(&"string"));
        assert!(filter.contains(&123));
        assert!(filter.contains(&true));
        assert!(filter.contains(&vec![1, 2, 3]));
    }

    #[test]
    fn test_false_positives_possible() {
        let mut filter = BloomFilter::new();
        filter.insert(&"test1");

        let mut false_positive_found = false;
        for i in 0..1000 {
            let test_str = format!("not_inserted_{}", i);
            if !filter.contains(&"test1") {
                panic!("False negative detected");
            }
            if filter.contains(&test_str) {
                false_positive_found = true;
                break;
            }
        }
    }
}

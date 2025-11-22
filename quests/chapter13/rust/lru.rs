// Chapter 13: Crystal Cache - LRU Cache

use std::collections::HashMap;

/// LRUCache represents a Least Recently Used cache with a fixed capacity
pub struct LRUCache {
    capacity: usize,
    // TODO: Add data structures to track cache entries and their recency
    // Hint: Consider using a HashMap for fast lookups and a linked structure
    // to maintain insertion/access order
    cache: HashMap<i32, i32>,
}

impl LRUCache {
    /// Creates a new LRU cache with the given capacity
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            // TODO: Initialize your data structures
            cache: HashMap::new(),
        }
    }

    /// Retrieves the value associated with the key
    /// Returns -1 if the key doesn't exist
    /// When a key is accessed, it should be marked as recently used
    pub fn get(&mut self, key: i32) -> i32 {
        // TODO: Implement get operation
        // 1. Check if key exists in cache
        // 2. If it exists, move it to most recently used position
        // 3. Return the value, or -1 if not found
        -1
    }

    /// Adds or updates a key-value pair in the cache
    /// If the cache is at capacity, evict the least recently used item first
    /// If the key already exists, update its value and mark as recently used
    pub fn put(&mut self, key: i32, value: i32) {
        // TODO: Implement put operation
        // 1. If key exists, update value and move to most recently used
        // 2. If key doesn't exist and cache is at capacity, evict LRU item
        // 3. Add the new key-value pair as most recently used
    }

    /// Returns the current number of items in the cache
    pub fn size(&self) -> usize {
        // TODO: Return the current cache size
        0
    }

    /// Checks if a key exists in the cache
    pub fn has(&self, key: i32) -> bool {
        // TODO: Check if key exists in cache
        false
    }
}

/// Represents a cache operation
#[derive(Debug)]
pub enum Operation {
    Put { key: i32, value: i32 },
    Get { key: i32 },
}

/// Calculates the cache hit rate as a percentage
/// Given a sequence of operations, it simulates them and returns
/// the percentage of successful gets (hits) out of total get operations
pub fn cache_hit_rate(operations: &[Operation]) -> f64 {
    // TODO: Implement cache hit rate calculation
    // 1. Create a cache with reasonable capacity (e.g., 10)
    // 2. Process each operation (put or get)
    // 3. Track hits and total gets
    // 4. Return (hits / totalGets) * 100.0
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.get(1), 10);
    }

    #[test]
    fn test_eviction() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.get(1); // Make 1 recently used
        cache.put(3, 30); // Should evict 2
        assert_eq!(cache.get(2), -1);
    }
}

// Reference solutions for Chapter 13: LRU Cache

use std::collections::HashMap;

/// LRUCache implementation using HashMap and Vec for order tracking
/// This implementation uses a HashMap for O(1) lookups and a Vec to track access order
/// Note: Moving elements in the Vec is O(n), but this is a simple, working implementation
/// A production implementation would use a doubly-linked list for O(1) operations
pub struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    order: Vec<i32>, // Keys in order from least recently used (front) to most recently used (back)
}

impl LRUCache {
    /// Creates a new LRU cache with the given capacity
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            order: Vec::new(),
        }
    }

    /// Retrieves the value associated with the key
    /// Returns -1 if the key doesn't exist
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.cache.get(&key) {
            // Move key to end (most recently used)
            self.move_to_end(key);
            value
        } else {
            -1
        }
    }

    /// Adds or updates a key-value pair in the cache
    pub fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            // Update existing key
            self.cache.insert(key, value);
            self.move_to_end(key);
        } else {
            // Add new key
            if self.cache.len() >= self.capacity {
                // Evict least recently used (first in order)
                if let Some(lru_key) = self.order.first().copied() {
                    self.cache.remove(&lru_key);
                    self.order.remove(0);
                }
            }
            self.cache.insert(key, value);
            self.order.push(key);
        }
    }

    /// Moves a key to the end of the order Vec (most recently used position)
    fn move_to_end(&mut self, key: i32) {
        if let Some(pos) = self.order.iter().position(|&k| k == key) {
            self.order.remove(pos);
            self.order.push(key);
        }
    }

    /// Returns the current number of items in the cache
    pub fn size(&self) -> usize {
        self.cache.len()
    }

    /// Checks if a key exists in the cache
    pub fn has(&self, key: i32) -> bool {
        self.cache.contains_key(&key)
    }
}

/// Represents a cache operation
#[derive(Debug)]
pub enum Operation {
    Put { key: i32, value: i32 },
    Get { key: i32 },
}

/// Calculates the cache hit rate as a percentage
pub fn cache_hit_rate(operations: &[Operation]) -> f64 {
    let mut cache = LRUCache::new(10); // Reasonable default capacity
    let mut hits = 0;
    let mut total_gets = 0;

    for op in operations {
        match op {
            Operation::Put { key, value } => {
                cache.put(*key, *value);
            }
            Operation::Get { key } => {
                total_gets += 1;
                if cache.get(*key) != -1 {
                    hits += 1;
                }
            }
        }
    }

    if total_gets == 0 {
        return 0.0;
    }

    (hits as f64 / total_gets as f64) * 100.0
}

// ============================================================================
// Alternative implementation using indices for a more efficient doubly-linked list
// This avoids the O(n) removal but is more complex
// ============================================================================

type Link = Option<usize>;

struct Node {
    key: i32,
    value: i32,
    prev: Link,
    next: Link,
}

/// More efficient LRU cache using an index-based doubly-linked list
pub struct LRUCacheOptimized {
    capacity: usize,
    cache: HashMap<i32, usize>, // Maps key to node index
    nodes: Vec<Node>,           // Node storage
    head: Link,                 // Most recently used
    tail: Link,                 // Least recently used
    free_list: Vec<usize>,      // Indices of free nodes
}

impl LRUCacheOptimized {
    pub fn new(capacity: usize) -> Self {
        LRUCacheOptimized {
            capacity,
            cache: HashMap::new(),
            nodes: Vec::new(),
            head: None,
            tail: None,
            free_list: Vec::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&node_idx) = self.cache.get(&key) {
            let value = self.nodes[node_idx].value;
            self.move_to_head(node_idx);
            value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&node_idx) = self.cache.get(&key) {
            // Update existing node
            self.nodes[node_idx].value = value;
            self.move_to_head(node_idx);
        } else {
            // Check capacity
            if self.cache.len() >= self.capacity {
                // Evict tail
                if let Some(tail_idx) = self.tail {
                    let tail_key = self.nodes[tail_idx].key;
                    self.remove_node(tail_idx);
                    self.cache.remove(&tail_key);
                    self.free_list.push(tail_idx);
                }
            }

            // Add new node
            let node_idx = if let Some(idx) = self.free_list.pop() {
                self.nodes[idx] = Node {
                    key,
                    value,
                    prev: None,
                    next: self.head,
                };
                idx
            } else {
                let idx = self.nodes.len();
                self.nodes.push(Node {
                    key,
                    value,
                    prev: None,
                    next: self.head,
                });
                idx
            };

            self.cache.insert(key, node_idx);
            self.add_to_head(node_idx);
        }
    }

    fn move_to_head(&mut self, node_idx: usize) {
        if self.head == Some(node_idx) {
            return; // Already at head
        }
        self.remove_node(node_idx);
        self.add_to_head(node_idx);
    }

    fn remove_node(&mut self, node_idx: usize) {
        let node = &self.nodes[node_idx];
        let prev_idx = node.prev;
        let next_idx = node.next;

        if let Some(prev) = prev_idx {
            self.nodes[prev].next = next_idx;
        } else {
            self.head = next_idx;
        }

        if let Some(next) = next_idx {
            self.nodes[next].prev = prev_idx;
        } else {
            self.tail = prev_idx;
        }
    }

    fn add_to_head(&mut self, node_idx: usize) {
        self.nodes[node_idx].prev = None;
        self.nodes[node_idx].next = self.head;

        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(node_idx);
        }

        self.head = Some(node_idx);

        if self.tail.is_none() {
            self.tail = Some(node_idx);
        }
    }

    pub fn size(&self) -> usize {
        self.cache.len()
    }

    pub fn has(&self, key: i32) -> bool {
        self.cache.contains_key(&key)
    }
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
        assert_eq!(cache.size(), 2);
    }

    #[test]
    fn test_eviction() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.get(1); // Make 1 recently used
        cache.put(3, 30); // Should evict 2
        assert_eq!(cache.get(2), -1);
        assert!(cache.has(1));
        assert!(cache.has(3));
    }

    #[test]
    fn test_update() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(1, 100);
        assert_eq!(cache.get(1), 100);
        assert_eq!(cache.size(), 1);
    }

    #[test]
    fn test_optimized_cache() {
        let mut cache = LRUCacheOptimized::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.get(1), 10);
        cache.put(3, 30);
        assert_eq!(cache.get(2), -1);
    }
}

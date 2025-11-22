// Reference solutions for Chapter 15: Core of the Architect - Final Boss
// Distributed Systems: Consistent Hashing and CRDTs

use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// ConsistentHash implements consistent hashing for distributed systems
/// Uses virtual nodes (replicas) to improve key distribution
pub struct ConsistentHash {
    ring: BTreeMap<u64, String>,  // hash -> node mapping (sorted for efficient lookup)
    nodes: Vec<String>,           // list of physical nodes
    replicas: usize,              // number of virtual nodes per physical node
}

impl ConsistentHash {
    /// Creates a new consistent hash ring with the given number of replicas
    pub fn new(replicas: usize) -> Self {
        ConsistentHash {
            ring: BTreeMap::new(),
            nodes: Vec::new(),
            replicas,
        }
    }

    /// Computes a hash for the given key
    fn hash_key(key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    /// Adds a physical node to the hash ring
    /// Creates 'replicas' virtual nodes for better distribution
    pub fn add_node(&mut self, node: &str) {
        // Check if node already exists
        if self.nodes.contains(&node.to_string()) {
            return;
        }

        // Add to nodes list
        self.nodes.push(node.to_string());

        // Create virtual nodes
        for i in 0..self.replicas {
            let virtual_key = format!("{}-{}", node, i);
            let hash = Self::hash_key(&virtual_key);
            self.ring.insert(hash, node.to_string());
        }
    }

    /// Removes a physical node and all its virtual nodes from the ring
    pub fn remove_node(&mut self, node: &str) {
        // Remove from nodes list
        self.nodes.retain(|n| n != node);

        // Remove virtual nodes from ring
        for i in 0..self.replicas {
            let virtual_key = format!("{}-{}", node, i);
            let hash = Self::hash_key(&virtual_key);
            self.ring.remove(&hash);
        }
    }

    /// Finds the node responsible for a given key
    /// Uses clockwise traversal on the hash ring
    pub fn get_node(&self, key: &str) -> Option<&str> {
        if self.ring.is_empty() {
            return None;
        }

        let hash = Self::hash_key(key);

        // Find the first node with hash >= key hash (clockwise)
        // BTreeMap's range gives us efficient lookup
        if let Some((_, node)) = self.ring.range(hash..).next() {
            return Some(node.as_str());
        }

        // Wrap around to the first node
        self.ring.values().next().map(|s| s.as_str())
    }

    /// Returns a copy of all physical nodes
    pub fn get_nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }

    /// Returns the number of physical nodes in the ring
    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}

// ============================================================================
// GCounter - A CRDT (Conflict-free Replicated Data Type) grow-only counter
// Each node maintains its own counter, and counters can only be incremented
// Merging takes the maximum value for each node
// ============================================================================

/// GCounter represents a grow-only counter CRDT
pub struct GCounter {
    node_id: String,              // ID of the local node
    counts: HashMap<String, i64>, // node ID -> count mapping
}

impl GCounter {
    /// Creates a new GCounter for the given node
    pub fn new(node_id: &str) -> Self {
        GCounter {
            node_id: node_id.to_string(),
            counts: HashMap::new(),
        }
    }

    /// Increments the counter for the local node by 1
    pub fn increment(&mut self) {
        *self.counts.entry(self.node_id.clone()).or_insert(0) += 1;
    }

    /// Increments the counter for the local node by a specified amount
    pub fn increment_by(&mut self, amount: i64) {
        if amount > 0 {
            *self.counts.entry(self.node_id.clone()).or_insert(0) += amount;
        }
    }

    /// Returns the total count across all nodes
    pub fn value(&self) -> i64 {
        self.counts.values().sum()
    }

    /// Returns the count for the local node only
    pub fn local_value(&self) -> i64 {
        *self.counts.get(&self.node_id).unwrap_or(&0)
    }

    /// Merges another GCounter into this one
    /// Takes the maximum value for each node (ensures convergence)
    pub fn merge(&mut self, other: &GCounter) {
        for (node_id, &count) in &other.counts {
            let current = self.counts.entry(node_id.clone()).or_insert(0);
            *current = (*current).max(count);
        }
    }

    /// Returns a copy of the internal counts map
    pub fn get_counts(&self) -> HashMap<String, i64> {
        self.counts.clone()
    }
}

// ============================================================================
// PNCounter - A CRDT counter that supports both increment and decrement
// Implemented as two GCounters: one for increments (P) and one for decrements (N)
// ============================================================================

/// PNCounter represents a positive-negative counter CRDT
pub struct PNCounter {
    #[allow(dead_code)]
    node_id: String,  // ID of the local node
    p: GCounter,      // positive counter (increments)
    n: GCounter,      // negative counter (decrements)
}

impl PNCounter {
    /// Creates a new PNCounter for the given node
    pub fn new(node_id: &str) -> Self {
        PNCounter {
            node_id: node_id.to_string(),
            p: GCounter::new(node_id),
            n: GCounter::new(node_id),
        }
    }

    /// Increments the counter by 1
    pub fn increment(&mut self) {
        self.p.increment();
    }

    /// Decrements the counter by 1
    pub fn decrement(&mut self) {
        self.n.increment();
    }

    /// Returns the current value (P - N)
    pub fn value(&self) -> i64 {
        self.p.value() - self.n.value()
    }

    /// Merges another PNCounter into this one
    pub fn merge(&mut self, other: &PNCounter) {
        self.p.merge(&other.p);
        self.n.merge(&other.n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistent_hash_add_nodes() {
        let mut ch = ConsistentHash::new(3);
        ch.add_node("server-1");
        ch.add_node("server-2");
        ch.add_node("server-3");
        assert_eq!(ch.size(), 3);
    }

    #[test]
    fn test_consistent_hash_remove_node() {
        let mut ch = ConsistentHash::new(3);
        ch.add_node("server-1");
        ch.add_node("server-2");
        ch.remove_node("server-1");
        assert_eq!(ch.size(), 1);
    }

    #[test]
    fn test_consistent_hash_get_node() {
        let mut ch = ConsistentHash::new(3);
        ch.add_node("server-1");
        ch.add_node("server-2");

        let node = ch.get_node("some-key");
        assert!(node.is_some());
        assert!(node.unwrap() == "server-1" || node.unwrap() == "server-2");
    }

    #[test]
    fn test_consistent_hash_consistency() {
        let mut ch = ConsistentHash::new(3);
        ch.add_node("server-1");
        ch.add_node("server-2");

        let first_result = ch.get_node("consistent-key");
        for _ in 0..100 {
            assert_eq!(ch.get_node("consistent-key"), first_result);
        }
    }

    #[test]
    fn test_consistent_hash_empty() {
        let ch = ConsistentHash::new(3);
        assert!(ch.get_node("any-key").is_none());
    }

    #[test]
    fn test_gcounter_increment() {
        let mut gc = GCounter::new("node-1");
        gc.increment();
        gc.increment();
        gc.increment();
        assert_eq!(gc.value(), 3);
        assert_eq!(gc.local_value(), 3);
    }

    #[test]
    fn test_gcounter_increment_by() {
        let mut gc = GCounter::new("node-1");
        gc.increment_by(5);
        gc.increment_by(3);
        assert_eq!(gc.value(), 8);
    }

    #[test]
    fn test_gcounter_increment_by_negative() {
        let mut gc = GCounter::new("node-1");
        gc.increment_by(5);
        gc.increment_by(-3); // Should be ignored
        assert_eq!(gc.value(), 5);
    }

    #[test]
    fn test_gcounter_merge() {
        let mut gc1 = GCounter::new("node-1");
        gc1.increment();
        gc1.increment();

        let mut gc2 = GCounter::new("node-2");
        gc2.increment();
        gc2.increment();
        gc2.increment();

        gc1.merge(&gc2);
        assert_eq!(gc1.value(), 5); // 2 from node-1 + 3 from node-2
    }

    #[test]
    fn test_gcounter_merge_idempotent() {
        let mut gc1 = GCounter::new("node-1");
        gc1.increment_by(5);

        let mut gc2 = GCounter::new("node-2");
        gc2.increment_by(3);

        gc1.merge(&gc2);
        assert_eq!(gc1.value(), 8);

        gc1.merge(&gc2); // Merge again
        assert_eq!(gc1.value(), 8); // Should remain the same
    }

    #[test]
    fn test_gcounter_merge_commutative() {
        let mut gc_a = GCounter::new("A");
        gc_a.increment_by(5);

        let mut gc_b = GCounter::new("B");
        gc_b.increment_by(3);

        // Merge A into copy1, then B
        let mut copy1 = GCounter::new("copy1");
        copy1.merge(&gc_a);
        copy1.merge(&gc_b);

        // Merge B into copy2, then A
        let mut copy2 = GCounter::new("copy2");
        copy2.merge(&gc_b);
        copy2.merge(&gc_a);

        assert_eq!(copy1.value(), copy2.value());
    }

    #[test]
    fn test_pncounter_increment() {
        let mut pn = PNCounter::new("node-1");
        pn.increment();
        pn.increment();
        pn.increment();
        assert_eq!(pn.value(), 3);
    }

    #[test]
    fn test_pncounter_decrement() {
        let mut pn = PNCounter::new("node-1");
        pn.increment();
        pn.increment();
        pn.decrement();
        assert_eq!(pn.value(), 1);
    }

    #[test]
    fn test_pncounter_negative() {
        let mut pn = PNCounter::new("node-1");
        pn.increment();
        pn.decrement();
        pn.decrement();
        assert_eq!(pn.value(), -1);
    }

    #[test]
    fn test_pncounter_merge() {
        let mut pn1 = PNCounter::new("node-1");
        pn1.increment();
        pn1.increment();
        pn1.decrement();

        let mut pn2 = PNCounter::new("node-2");
        pn2.increment();
        pn2.decrement();
        pn2.decrement();

        pn1.merge(&pn2);
        // pn1: node-1 has +2-1=1, node-2 has +1-2=-1, total = 0
        assert_eq!(pn1.value(), 0);
    }
}

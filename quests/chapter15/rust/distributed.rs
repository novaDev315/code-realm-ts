// Chapter 15: Core of the Architect - Final Boss
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
        // TODO: Implement adding a node to the consistent hash ring
        // 1. Check if node already exists in self.nodes
        // 2. Add node to self.nodes
        // 3. For each replica (0 to self.replicas):
        //    - Create virtual node key: format!("{}-{}", node, i)
        //    - Compute hash using Self::hash_key()
        //    - Insert into self.ring: hash -> node.to_string()
    }

    /// Removes a physical node and all its virtual nodes from the ring
    pub fn remove_node(&mut self, node: &str) {
        // TODO: Implement removing a node from the consistent hash ring
        // 1. Find and remove node from self.nodes (use retain or position+remove)
        // 2. For each replica (0 to self.replicas):
        //    - Create virtual node key: format!("{}-{}", node, i)
        //    - Compute hash and remove from self.ring
    }

    /// Finds the node responsible for a given key
    /// Uses clockwise traversal on the hash ring
    pub fn get_node(&self, key: &str) -> Option<&str> {
        // TODO: Implement getting the node for a key
        // 1. If ring is empty, return None
        // 2. Compute hash of the key using Self::hash_key()
        // 3. Use BTreeMap's range() to find the first hash >= key hash
        // 4. If no hash >= key hash found, wrap around to first node
        //    (Hint: use self.ring.iter().next())
        // 5. Return Some(node) with the node associated with that hash
        None
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
        // TODO: Implement incrementing the local counter
        // Use entry().or_insert(0) and increment
        // *self.counts.entry(self.node_id.clone()).or_insert(0) += 1;
    }

    /// Increments the counter for the local node by a specified amount
    pub fn increment_by(&mut self, amount: i64) {
        // TODO: Implement incrementing by a specific amount
        // Only allow positive increments (GCounter can only grow)
        // if amount > 0 { ... }
    }

    /// Returns the total count across all nodes
    pub fn value(&self) -> i64 {
        // TODO: Implement getting the total value
        // Sum all values in self.counts: self.counts.values().sum()
        0
    }

    /// Returns the count for the local node only
    pub fn local_value(&self) -> i64 {
        // TODO: Return the local node's count
        // *self.counts.get(&self.node_id).unwrap_or(&0)
        0
    }

    /// Merges another GCounter into this one
    /// Takes the maximum value for each node (ensures convergence)
    pub fn merge(&mut self, other: &GCounter) {
        // TODO: Implement merging two GCounters
        // For each (node_id, count) in other.counts:
        //   let current = self.counts.entry(node_id.clone()).or_insert(0);
        //   *current = (*current).max(*count);
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
        // TODO: Implement increment using the P counter
        // self.p.increment();
    }

    /// Decrements the counter by 1
    pub fn decrement(&mut self) {
        // TODO: Implement decrement using the N counter
        // self.n.increment();
    }

    /// Returns the current value (P - N)
    pub fn value(&self) -> i64 {
        // TODO: Implement getting the value
        // self.p.value() - self.n.value()
        0
    }

    /// Merges another PNCounter into this one
    pub fn merge(&mut self, other: &PNCounter) {
        // TODO: Implement merging two PNCounters
        // self.p.merge(&other.p);
        // self.n.merge(&other.n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistent_hash_basic() {
        let mut ch = ConsistentHash::new(3);
        ch.add_node("server-1");
        ch.add_node("server-2");
        assert_eq!(ch.size(), 2);
    }

    #[test]
    fn test_consistent_hash_get_node() {
        let mut ch = ConsistentHash::new(3);
        ch.add_node("server-1");
        ch.add_node("server-2");

        let node = ch.get_node("some-key");
        assert!(node.is_some());
    }

    #[test]
    fn test_gcounter_basic() {
        let mut gc = GCounter::new("node-1");
        gc.increment();
        gc.increment();
        assert_eq!(gc.value(), 2);
    }

    #[test]
    fn test_gcounter_merge() {
        let mut gc1 = GCounter::new("node-1");
        gc1.increment();
        gc1.increment();

        let mut gc2 = GCounter::new("node-2");
        gc2.increment();

        gc1.merge(&gc2);
        assert_eq!(gc1.value(), 3);
    }

    #[test]
    fn test_pncounter() {
        let mut pn = PNCounter::new("node-1");
        pn.increment();
        pn.increment();
        pn.decrement();
        assert_eq!(pn.value(), 1);
    }
}

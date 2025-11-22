// Chapter 15: Core of the Architect - Final Boss
// Distributed Systems: Consistent Hashing and CRDTs

package main

import (
	"hash/fnv"
	"sort"
)

// ConsistentHash implements consistent hashing for distributed systems
// Uses virtual nodes (replicas) to improve key distribution
type ConsistentHash struct {
	ring     map[uint32]string // hash -> node mapping
	nodes    []string          // list of physical nodes
	replicas int               // number of virtual nodes per physical node
	keys     []uint32          // sorted hash keys for binary search
}

// NewConsistentHash creates a new consistent hash ring with the given number of replicas
func NewConsistentHash(replicas int) *ConsistentHash {
	return &ConsistentHash{
		ring:     make(map[uint32]string),
		nodes:    make([]string, 0),
		replicas: replicas,
		keys:     make([]uint32, 0),
	}
}

// hashKey computes a hash for the given key
func hashKey(key string) uint32 {
	h := fnv.New32a()
	h.Write([]byte(key))
	return h.Sum32()
}

// AddNode adds a physical node to the hash ring
// Creates 'replicas' virtual nodes for better distribution
func (ch *ConsistentHash) AddNode(node string) {
	// TODO: Implement adding a node to the consistent hash ring
	// 1. Check if node already exists in ch.nodes
	// 2. Add node to ch.nodes slice
	// 3. For each replica (0 to ch.replicas-1):
	//    - Create virtual node key: fmt.Sprintf("%s-%d", node, i)
	//    - Compute hash using hashKey()
	//    - Add to ch.ring mapping hash -> node
	//    - Add hash to ch.keys
	// 4. Sort ch.keys for binary search
}

// RemoveNode removes a physical node and all its virtual nodes from the ring
func (ch *ConsistentHash) RemoveNode(node string) {
	// TODO: Implement removing a node from the consistent hash ring
	// 1. Find and remove node from ch.nodes slice
	// 2. For each replica (0 to ch.replicas-1):
	//    - Create virtual node key: fmt.Sprintf("%s-%d", node, i)
	//    - Compute hash and remove from ch.ring
	// 3. Rebuild ch.keys slice (collect all keys from ch.ring and sort)
}

// GetNode finds the node responsible for a given key
// Uses clockwise traversal on the hash ring
func (ch *ConsistentHash) GetNode(key string) string {
	// TODO: Implement getting the node for a key
	// 1. If ring is empty, return ""
	// 2. Compute hash of the key using hashKey()
	// 3. Use binary search to find the first hash >= key hash
	//    (Hint: use sort.Search)
	// 4. If no hash >= key hash found, wrap around to first node
	// 5. Return the node associated with that hash
	return ""
}

// GetNodes returns the list of all physical nodes
func (ch *ConsistentHash) GetNodes() []string {
	return ch.nodes
}

// Size returns the number of physical nodes in the ring
func (ch *ConsistentHash) Size() int {
	return len(ch.nodes)
}

// ============================================================================
// GCounter - A CRDT (Conflict-free Replicated Data Type) grow-only counter
// Each node maintains its own counter, and counters can only be incremented
// Merging takes the maximum value for each node
// ============================================================================

// GCounter represents a grow-only counter CRDT
type GCounter struct {
	nodeID string         // ID of the local node
	counts map[string]int // node ID -> count mapping
}

// NewGCounter creates a new GCounter for the given node
func NewGCounter(nodeID string) *GCounter {
	return &GCounter{
		nodeID: nodeID,
		counts: make(map[string]int),
	}
}

// Increment increases the counter for the local node by 1
func (gc *GCounter) Increment() {
	// TODO: Implement incrementing the local counter
	// Increment gc.counts[gc.nodeID] by 1
}

// IncrementBy increases the counter for the local node by a specified amount
func (gc *GCounter) IncrementBy(amount int) {
	// TODO: Implement incrementing by a specific amount
	// Only allow positive increments (GCounter can only grow)
	// Increment gc.counts[gc.nodeID] by amount
}

// Value returns the total count across all nodes
func (gc *GCounter) Value() int {
	// TODO: Implement getting the total value
	// Sum all values in gc.counts map
	return 0
}

// LocalValue returns the count for the local node only
func (gc *GCounter) LocalValue() int {
	// TODO: Implement getting local node's count
	return 0
}

// Merge combines another GCounter into this one
// Takes the maximum value for each node (ensures convergence)
func (gc *GCounter) Merge(other *GCounter) {
	// TODO: Implement merging two GCounters
	// For each node in other.counts:
	//   gc.counts[node] = max(gc.counts[node], other.counts[node])
	// This ensures CRDTs converge to the same value
}

// GetCounts returns a copy of the internal counts map
func (gc *GCounter) GetCounts() map[string]int {
	result := make(map[string]int)
	for k, v := range gc.counts {
		result[k] = v
	}
	return result
}

// ============================================================================
// PNCounter - A CRDT counter that supports both increment and decrement
// Implemented as two GCounters: one for increments (P) and one for decrements (N)
// ============================================================================

// PNCounter represents a positive-negative counter CRDT
type PNCounter struct {
	nodeID string    // ID of the local node
	p      *GCounter // positive counter (increments)
	n      *GCounter // negative counter (decrements)
}

// NewPNCounter creates a new PNCounter for the given node
func NewPNCounter(nodeID string) *PNCounter {
	return &PNCounter{
		nodeID: nodeID,
		p:      NewGCounter(nodeID),
		n:      NewGCounter(nodeID),
	}
}

// Increment increases the counter by 1
func (pn *PNCounter) Increment() {
	// TODO: Implement increment using the P counter
}

// Decrement decreases the counter by 1
func (pn *PNCounter) Decrement() {
	// TODO: Implement decrement using the N counter
}

// Value returns the current value (P - N)
func (pn *PNCounter) Value() int {
	// TODO: Implement getting the value
	// Return p.Value() - n.Value()
	return 0
}

// Merge combines another PNCounter into this one
func (pn *PNCounter) Merge(other *PNCounter) {
	// TODO: Implement merging two PNCounters
	// Merge both P and N counters separately
}

// Helper function for max (not in standard library before Go 1.21)
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// Keep sort package usage to satisfy import
var _ = sort.Search

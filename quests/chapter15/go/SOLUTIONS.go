// Reference solutions for Chapter 15: Core of the Architect - Final Boss
// Distributed Systems: Consistent Hashing and CRDTs

package main

import (
	"fmt"
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
	// Check if node already exists
	for _, n := range ch.nodes {
		if n == node {
			return // Node already in ring
		}
	}

	// Add to nodes list
	ch.nodes = append(ch.nodes, node)

	// Create virtual nodes
	for i := 0; i < ch.replicas; i++ {
		virtualKey := fmt.Sprintf("%s-%d", node, i)
		hash := hashKey(virtualKey)
		ch.ring[hash] = node
		ch.keys = append(ch.keys, hash)
	}

	// Sort keys for binary search
	sort.Slice(ch.keys, func(i, j int) bool {
		return ch.keys[i] < ch.keys[j]
	})
}

// RemoveNode removes a physical node and all its virtual nodes from the ring
func (ch *ConsistentHash) RemoveNode(node string) {
	// Find and remove from nodes slice
	idx := -1
	for i, n := range ch.nodes {
		if n == node {
			idx = i
			break
		}
	}
	if idx == -1 {
		return // Node not found
	}

	ch.nodes = append(ch.nodes[:idx], ch.nodes[idx+1:]...)

	// Remove virtual nodes from ring
	for i := 0; i < ch.replicas; i++ {
		virtualKey := fmt.Sprintf("%s-%d", node, i)
		hash := hashKey(virtualKey)
		delete(ch.ring, hash)
	}

	// Rebuild keys slice
	ch.keys = make([]uint32, 0, len(ch.ring))
	for hash := range ch.ring {
		ch.keys = append(ch.keys, hash)
	}
	sort.Slice(ch.keys, func(i, j int) bool {
		return ch.keys[i] < ch.keys[j]
	})
}

// GetNode finds the node responsible for a given key
// Uses clockwise traversal on the hash ring
func (ch *ConsistentHash) GetNode(key string) string {
	if len(ch.keys) == 0 {
		return ""
	}

	hash := hashKey(key)

	// Binary search for the first hash >= key hash
	idx := sort.Search(len(ch.keys), func(i int) bool {
		return ch.keys[i] >= hash
	})

	// Wrap around if we've gone past the end
	if idx >= len(ch.keys) {
		idx = 0
	}

	return ch.ring[ch.keys[idx]]
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
	gc.counts[gc.nodeID]++
}

// IncrementBy increases the counter for the local node by a specified amount
func (gc *GCounter) IncrementBy(amount int) {
	if amount > 0 {
		gc.counts[gc.nodeID] += amount
	}
}

// Value returns the total count across all nodes
func (gc *GCounter) Value() int {
	total := 0
	for _, count := range gc.counts {
		total += count
	}
	return total
}

// LocalValue returns the count for the local node only
func (gc *GCounter) LocalValue() int {
	return gc.counts[gc.nodeID]
}

// Merge combines another GCounter into this one
// Takes the maximum value for each node (ensures convergence)
func (gc *GCounter) Merge(other *GCounter) {
	for nodeID, count := range other.counts {
		if count > gc.counts[nodeID] {
			gc.counts[nodeID] = count
		}
	}
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
	pn.p.Increment()
}

// Decrement decreases the counter by 1
func (pn *PNCounter) Decrement() {
	pn.n.Increment()
}

// Value returns the current value (P - N)
func (pn *PNCounter) Value() int {
	return pn.p.Value() - pn.n.Value()
}

// Merge combines another PNCounter into this one
func (pn *PNCounter) Merge(other *PNCounter) {
	pn.p.Merge(other.p)
	pn.n.Merge(other.n)
}

// Helper function for max (not in standard library before Go 1.21)
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

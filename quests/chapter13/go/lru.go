// Chapter 13: Crystal Cache - LRU Cache

package main

// LRUCache represents a Least Recently Used cache with a fixed capacity
type LRUCache struct {
	capacity int
	// TODO: Add data structures to track cache entries and their recency
	// Hint: Consider using a map for fast lookups and a doubly linked list
	// to maintain insertion/access order
}

// NewLRUCache creates a new LRU cache with the given capacity
func NewLRUCache(capacity int) *LRUCache {
	return &LRUCache{
		capacity: capacity,
		// TODO: Initialize your data structures
	}
}

// Get retrieves the value associated with the key
// Returns -1 if the key doesn't exist
// When a key is accessed, it should be marked as recently used
func (cache *LRUCache) Get(key int) int {
	// TODO: Implement get operation
	// 1. Check if key exists in cache
	// 2. If it exists, move it to most recently used position
	// 3. Return the value, or -1 if not found
	return -1
}

// Put adds or updates a key-value pair in the cache
// If the cache is at capacity, evict the least recently used item first
// If the key already exists, update its value and mark as recently used
func (cache *LRUCache) Put(key int, value int) {
	// TODO: Implement put operation
	// 1. If key exists, update value and move to most recently used
	// 2. If key doesn't exist and cache is at capacity, evict LRU item
	// 3. Add the new key-value pair as most recently used
}

// Size returns the current number of items in the cache
func (cache *LRUCache) Size() int {
	// TODO: Return the current cache size
	return 0
}

// Has checks if a key exists in the cache
func (cache *LRUCache) Has(key int) bool {
	// TODO: Check if key exists in cache
	return false
}

// CacheHitRate calculates the cache hit rate as a percentage
// Given a sequence of operations, it simulates them and returns
// the percentage of successful gets (hits) out of total get operations
func CacheHitRate(operations []map[string]interface{}) float64 {
	// TODO: Implement cache hit rate calculation
	// 1. Create a cache with reasonable capacity (e.g., 10)
	// 2. Process each operation (put or get)
	// 3. Track hits and total gets
	// 4. Return (hits / totalGets) * 100
	return 0.0
}

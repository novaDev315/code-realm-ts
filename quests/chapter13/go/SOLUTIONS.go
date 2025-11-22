// Reference solutions for Chapter 13: LRU Cache

package main

// Node represents a node in the doubly linked list
type node struct {
	key   int
	value int
	prev  *node
	next  *node
}

// NewLRUCache creates a new LRU cache with the given capacity
// This is the SOLUTION implementation
func NewLRUCache(capacity int) *LRUCache {
	cache := &LRUCache{
		capacity: capacity,
		cache:    make(map[int]*node),
		head:     &node{}, // Dummy head
		tail:     &node{}, // Dummy tail
	}
	// Connect head and tail
	cache.head.next = cache.tail
	cache.tail.prev = cache.head
	return cache
}

// LRUCache implements an LRU cache using a map and doubly linked list
// This is the SOLUTION implementation with working methods
type LRUCache struct {
	capacity int
	cache    map[int]*node
	head     *node // Most recently used (dummy head)
	tail     *node // Least recently used (dummy tail)
}

// removeNode removes a node from the doubly linked list
func (c *LRUCache) removeNode(n *node) {
	n.prev.next = n.next
	n.next.prev = n.prev
}

// addToHead adds a node right after the head (most recently used position)
func (c *LRUCache) addToHead(n *node) {
	n.next = c.head.next
	n.prev = c.head
	c.head.next.prev = n
	c.head.next = n
}

// moveToHead moves an existing node to the head (mark as recently used)
func (c *LRUCache) moveToHead(n *node) {
	c.removeNode(n)
	c.addToHead(n)
}

// removeTail removes and returns the least recently used node (before tail)
func (c *LRUCache) removeTail() *node {
	n := c.tail.prev
	c.removeNode(n)
	return n
}

// Get retrieves the value associated with the key
func (c *LRUCache) Get(key int) int {
	if n, exists := c.cache[key]; exists {
		// Move to most recently used
		c.moveToHead(n)
		return n.value
	}
	return -1
}

// Put adds or updates a key-value pair in the cache
func (c *LRUCache) Put(key int, value int) {
	if n, exists := c.cache[key]; exists {
		// Update existing node
		n.value = value
		c.moveToHead(n)
	} else {
		// Create new node
		newNode := &node{key: key, value: value}
		c.cache[key] = newNode
		c.addToHead(newNode)

		// Check capacity
		if len(c.cache) > c.capacity {
			// Evict LRU
			lru := c.removeTail()
			delete(c.cache, lru.key)
		}
	}
}

// Size returns the current number of items in the cache
func (c *LRUCache) Size() int {
	return len(c.cache)
}

// Has checks if a key exists in the cache
func (c *LRUCache) Has(key int) bool {
	_, exists := c.cache[key]
	return exists
}

// CacheHitRate calculates the cache hit rate as a percentage
func CacheHitRate(operations []map[string]interface{}) float64 {
	cache := NewLRUCache(10) // Reasonable default capacity
	hits := 0
	totalGets := 0

	for _, op := range operations {
		opType := op["op"].(string)
		if opType == "put" {
			key := op["key"].(int)
			value := op["value"].(int)
			cache.Put(key, value)
		} else if opType == "get" {
			totalGets++
			key := op["key"].(int)
			result := cache.Get(key)
			if result != -1 {
				hits++
			}
		}
	}

	if totalGets == 0 {
		return 0.0
	}

	return float64(hits) / float64(totalGets) * 100.0
}

// Reference solutions for Chapter 13: LRU Cache

class LRUCache {
  constructor(capacity) {
    this.capacity = capacity;
    this.cache = new Map();
  }

  get(key) {
    // Check if key exists
    if (!this.cache.has(key)) {
      return null;
    }

    // Get the value
    const value = this.cache.get(key);

    // Move to most recently used (delete and re-add to end)
    this.cache.delete(key);
    this.cache.set(key, value);

    return value;
  }

  put(key, value) {
    // If key already exists, remove it first
    if (this.cache.has(key)) {
      this.cache.delete(key);
    } else if (this.cache.size >= this.capacity) {
      // If at capacity, evict least recently used (first item)
      const firstKey = this.cache.keys().next().value;
      this.cache.delete(firstKey);
    }

    // Add/update the key-value pair (added to end = most recently used)
    this.cache.set(key, value);
  }

  size() {
    return this.cache.size;
  }

  has(key) {
    return this.cache.has(key);
  }
}

function cacheHitRate(operations) {
  const cache = new LRUCache(10); // Reasonable default capacity
  let hits = 0;
  let totalGets = 0;

  for (const op of operations) {
    if (op.op === "put") {
      cache.put(op.key, op.value);
    } else if (op.op === "get") {
      totalGets++;
      const result = cache.get(op.key);
      if (result !== null) {
        hits++;
      }
    }
  }

  if (totalGets === 0) {
    return 0;
  }

  return (hits / totalGets) * 100;
}

module.exports = { LRUCache, cacheHitRate };

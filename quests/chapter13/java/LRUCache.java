// Chapter 13: Crystal Cache - LRU Cache

import java.util.*;

public class LRUCache {
    private int capacity;

    public LRUCache(int capacity) {
        this.capacity = capacity;
        // TODO: Initialize your data structures
    }

    public int get(int key) {
        // TODO: Get value and move to most recently used
        // Return -1 if key not found
        return -1;
    }

    public void put(int key, int value) {
        // TODO: Add/update key-value pair
        // Evict least recently used item if at capacity
    }

    public int size() {
        // TODO: Return current cache size
        return 0;
    }

    public boolean has(int key) {
        // TODO: Check if key exists in cache
        return false;
    }
}

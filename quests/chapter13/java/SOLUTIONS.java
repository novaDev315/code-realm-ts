// Reference solutions for Chapter 13: Crystal Cache - LRU Cache

import java.util.*;

/**
 * Solution 1: Using LinkedHashMap with access order
 * This is the most elegant solution in Java, utilizing LinkedHashMap's built-in
 * access-order mode which automatically maintains insertion/access order.
 */
class LRUCacheSolution1 extends LinkedHashMap<Integer, Integer> {
    private int capacity;

    public LRUCacheSolution1(int capacity) {
        // true = access-order (instead of insertion-order)
        super(capacity, 0.75f, true);
        this.capacity = capacity;
    }

    public int get(int key) {
        return super.getOrDefault(key, -1);
    }

    public void put(int key, int value) {
        super.put(key, value);
    }

    @Override
    protected boolean removeEldestEntry(Map.Entry<Integer, Integer> eldest) {
        return size() > capacity;
    }
}

/**
 * Solution 2: Custom implementation using HashMap and Doubly Linked List
 * This demonstrates the underlying mechanism and is what you'd implement
 * in languages without a built-in ordered map.
 */
class LRUCacheSolution2 {
    private class Node {
        int key;
        int value;
        Node prev;
        Node next;

        Node(int key, int value) {
            this.key = key;
            this.value = value;
        }
    }

    private int capacity;
    private Map<Integer, Node> cache;
    private Node head; // Most recently used (dummy head)
    private Node tail; // Least recently used (dummy tail)

    public LRUCacheSolution2(int capacity) {
        this.capacity = capacity;
        this.cache = new HashMap<>();

        // Create dummy head and tail nodes
        this.head = new Node(0, 0);
        this.tail = new Node(0, 0);
        this.head.next = this.tail;
        this.tail.prev = this.head;
    }

    public int get(int key) {
        Node node = cache.get(key);
        if (node == null) {
            return -1;
        }

        // Move to front (most recently used)
        moveToHead(node);
        return node.value;
    }

    public void put(int key, int value) {
        Node node = cache.get(key);

        if (node != null) {
            // Update existing node
            node.value = value;
            moveToHead(node);
        } else {
            // Create new node
            Node newNode = new Node(key, value);
            cache.put(key, newNode);
            addToHead(newNode);

            // Check capacity
            if (cache.size() > capacity) {
                // Remove least recently used (tail)
                Node lru = removeTail();
                cache.remove(lru.key);
            }
        }
    }

    public int size() {
        return cache.size();
    }

    public boolean has(int key) {
        return cache.containsKey(key);
    }

    private void addToHead(Node node) {
        node.prev = head;
        node.next = head.next;
        head.next.prev = node;
        head.next = node;
    }

    private void removeNode(Node node) {
        node.prev.next = node.next;
        node.next.prev = node.prev;
    }

    private void moveToHead(Node node) {
        removeNode(node);
        addToHead(node);
    }

    private Node removeTail() {
        Node lru = tail.prev;
        removeNode(lru);
        return lru;
    }
}

/**
 * Solution 3: Simple HashMap with access tracking
 * A simpler but less efficient approach using timestamps.
 */
class LRUCacheSolution3 {
    private class CacheEntry {
        int value;
        long lastAccessed;

        CacheEntry(int value) {
            this.value = value;
            this.lastAccessed = System.nanoTime();
        }

        void access() {
            this.lastAccessed = System.nanoTime();
        }
    }

    private int capacity;
    private Map<Integer, CacheEntry> cache;

    public LRUCacheSolution3(int capacity) {
        this.capacity = capacity;
        this.cache = new HashMap<>();
    }

    public int get(int key) {
        CacheEntry entry = cache.get(key);
        if (entry == null) {
            return -1;
        }
        entry.access();
        return entry.value;
    }

    public void put(int key, int value) {
        if (cache.containsKey(key)) {
            // Update existing entry
            CacheEntry entry = cache.get(key);
            entry.value = value;
            entry.access();
        } else {
            // Add new entry
            if (cache.size() >= capacity) {
                // Find and remove least recently used
                int lruKey = -1;
                long oldestTime = Long.MAX_VALUE;

                for (Map.Entry<Integer, CacheEntry> entry : cache.entrySet()) {
                    if (entry.getValue().lastAccessed < oldestTime) {
                        oldestTime = entry.getValue().lastAccessed;
                        lruKey = entry.getKey();
                    }
                }

                cache.remove(lruKey);
            }
            cache.put(key, new CacheEntry(value));
        }
    }

    public int size() {
        return cache.size();
    }

    public boolean has(int key) {
        return cache.containsKey(key);
    }
}

// Main demonstration class
public class SOLUTIONS {
    public static void main(String[] args) {
        System.out.println("Testing Solution 1: LinkedHashMap");
        testCache(new LRUCacheSolution1(2));

        System.out.println("\n\nTesting Solution 2: HashMap + Doubly Linked List");
        testCache(new LRUCacheSolution2(2));

        System.out.println("\n\nTesting Solution 3: HashMap + Timestamps");
        testCache(new LRUCacheSolution3(2));
    }

    private static void testCache(Object cacheObj) {
        // Simple test using reflection or specific interface
        // For simplicity, testing Solution2 which has the full interface
        if (cacheObj instanceof LRUCacheSolution2) {
            LRUCacheSolution2 cache = (LRUCacheSolution2) cacheObj;

            cache.put(1, 10);
            cache.put(2, 20);
            System.out.println("get(1): " + cache.get(1)); // 10

            cache.put(3, 30); // Evicts key 2
            System.out.println("get(2): " + cache.get(2)); // -1 (evicted)
            System.out.println("has(1): " + cache.has(1)); // true
            System.out.println("has(2): " + cache.has(2)); // false
            System.out.println("size(): " + cache.size());  // 2
        }
    }
}

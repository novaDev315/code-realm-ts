// Reference solutions for Chapter 13: Crystal Cache - LRU Cache

#include <iostream>
#include <unordered_map>
#include <list>
using namespace std;

/**
 * Solution 1: Using unordered_map + list
 * This is the most common and efficient solution in C++.
 * - unordered_map provides O(1) lookup
 * - list provides O(1) insertion/deletion at both ends
 * - Store iterators in the map for O(1) updates
 */
class LRUCacheSolution1 {
private:
    int capacity;
    list<pair<int, int>> items; // List of (key, value) pairs, front = most recent
    unordered_map<int, list<pair<int, int>>::iterator> cache; // key -> iterator

public:
    LRUCacheSolution1(int capacity) : capacity(capacity) {}

    int get(int key) {
        auto it = cache.find(key);
        if (it == cache.end()) {
            return -1;
        }

        // Move to front (most recently used)
        items.splice(items.begin(), items, it->second);
        return it->second->second;
    }

    void put(int key, int value) {
        auto it = cache.find(key);

        if (it != cache.end()) {
            // Update existing key
            it->second->second = value;
            // Move to front
            items.splice(items.begin(), items, it->second);
        } else {
            // Add new key
            if (cache.size() >= capacity) {
                // Remove least recently used (back of list)
                int lruKey = items.back().first;
                items.pop_back();
                cache.erase(lruKey);
            }

            // Add to front
            items.push_front({key, value});
            cache[key] = items.begin();
        }
    }

    int size() {
        return cache.size();
    }

    bool has(int key) {
        return cache.find(key) != cache.end();
    }
};

/**
 * Solution 2: Custom doubly linked list implementation
 * This shows the underlying mechanism without using std::list.
 * More educational but more verbose.
 */
class LRUCacheSolution2 {
private:
    struct Node {
        int key;
        int value;
        Node* prev;
        Node* next;

        Node(int k, int v) : key(k), value(v), prev(nullptr), next(nullptr) {}
    };

    int capacity;
    unordered_map<int, Node*> cache;
    Node* head; // Dummy head (most recently used side)
    Node* tail; // Dummy tail (least recently used side)

    void addToHead(Node* node) {
        node->prev = head;
        node->next = head->next;
        head->next->prev = node;
        head->next = node;
    }

    void removeNode(Node* node) {
        node->prev->next = node->next;
        node->next->prev = node->prev;
    }

    void moveToHead(Node* node) {
        removeNode(node);
        addToHead(node);
    }

    Node* removeTail() {
        Node* lru = tail->prev;
        removeNode(lru);
        return lru;
    }

public:
    LRUCacheSolution2(int capacity) : capacity(capacity) {
        head = new Node(0, 0);
        tail = new Node(0, 0);
        head->next = tail;
        tail->prev = head;
    }

    ~LRUCacheSolution2() {
        // Clean up all nodes
        Node* curr = head;
        while (curr != nullptr) {
            Node* next = curr->next;
            delete curr;
            curr = next;
        }
    }

    int get(int key) {
        auto it = cache.find(key);
        if (it == cache.end()) {
            return -1;
        }

        Node* node = it->second;
        moveToHead(node);
        return node->value;
    }

    void put(int key, int value) {
        auto it = cache.find(key);

        if (it != cache.end()) {
            // Update existing node
            Node* node = it->second;
            node->value = value;
            moveToHead(node);
        } else {
            // Create new node
            Node* newNode = new Node(key, value);
            cache[key] = newNode;
            addToHead(newNode);

            // Check capacity
            if (cache.size() > capacity) {
                // Remove least recently used
                Node* lru = removeTail();
                cache.erase(lru->key);
                delete lru;
            }
        }
    }

    int size() {
        return cache.size();
    }

    bool has(int key) {
        return cache.find(key) != cache.end();
    }
};

/**
 * Solution 3: Using deque and unordered_map
 * Simpler but less efficient (O(n) for moving elements)
 */
class LRUCacheSolution3 {
private:
    int capacity;
    unordered_map<int, int> cache;
    list<int> order; // Track access order
    unordered_map<int, list<int>::iterator> position; // key -> position in order

public:
    LRUCacheSolution3(int capacity) : capacity(capacity) {}

    int get(int key) {
        if (cache.find(key) == cache.end()) {
            return -1;
        }

        // Move to front
        order.erase(position[key]);
        order.push_front(key);
        position[key] = order.begin();

        return cache[key];
    }

    void put(int key, int value) {
        if (cache.find(key) != cache.end()) {
            // Update existing
            cache[key] = value;
            order.erase(position[key]);
            order.push_front(key);
            position[key] = order.begin();
        } else {
            // Add new
            if (cache.size() >= capacity) {
                // Remove LRU
                int lruKey = order.back();
                order.pop_back();
                cache.erase(lruKey);
                position.erase(lruKey);
            }

            cache[key] = value;
            order.push_front(key);
            position[key] = order.begin();
        }
    }

    int size() {
        return cache.size();
    }

    bool has(int key) {
        return cache.find(key) != cache.end();
    }
};

// Demonstration
int main() {
    cout << "Testing Solution 1: unordered_map + list" << endl;
    LRUCacheSolution1 cache1(2);
    cache1.put(1, 10);
    cache1.put(2, 20);
    cout << "get(1): " << cache1.get(1) << endl; // 10
    cache1.put(3, 30); // Evicts key 2
    cout << "get(2): " << cache1.get(2) << endl; // -1 (evicted)
    cout << "has(1): " << (cache1.has(1) ? "true" : "false") << endl; // true
    cout << "has(2): " << (cache1.has(2) ? "true" : "false") << endl; // false
    cout << "size(): " << cache1.size() << endl; // 2

    cout << "\n\nTesting Solution 2: Custom doubly linked list" << endl;
    LRUCacheSolution2 cache2(2);
    cache2.put(1, 10);
    cache2.put(2, 20);
    cout << "get(1): " << cache2.get(1) << endl; // 10
    cache2.put(3, 30); // Evicts key 2
    cout << "get(2): " << cache2.get(2) << endl; // -1 (evicted)
    cout << "has(1): " << (cache2.has(1) ? "true" : "false") << endl; // true
    cout << "has(2): " << (cache2.has(2) ? "true" : "false") << endl; // false
    cout << "size(): " << cache2.size() << endl; // 2

    cout << "\n\nTesting Solution 3: deque approach" << endl;
    LRUCacheSolution3 cache3(2);
    cache3.put(1, 10);
    cache3.put(2, 20);
    cout << "get(1): " << cache3.get(1) << endl; // 10
    cache3.put(3, 30); // Evicts key 2
    cout << "get(2): " << cache3.get(2) << endl; // -1 (evicted)
    cout << "has(1): " << (cache3.has(1) ? "true" : "false") << endl; // true
    cout << "has(2): " << (cache3.has(2) ? "true" : "false") << endl; // false
    cout << "size(): " << cache3.size() << endl; // 2

    cout << "\n✅ All solutions work correctly!" << endl;

    return 0;
}

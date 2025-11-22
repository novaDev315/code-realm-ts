// Reference solutions for Chapter 15: Core of the Architect - Final Boss
// Distributed Systems: Consistent Hashing and CRDTs

#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <unordered_map>
#include <algorithm>
#include <cstdint>

using namespace std;

/**
 * Solution: ConsistentHash implements consistent hashing for distributed systems.
 * Uses virtual nodes (replicas) to improve key distribution.
 */
class ConsistentHashSolution {
private:
    map<uint32_t, string> ring;     // hash -> node mapping (sorted)
    vector<string> nodes;            // list of physical nodes
    int replicas;                    // number of virtual nodes per physical node

    /**
     * Computes a hash for the given key using FNV-1a algorithm.
     * Time complexity: O(n) where n is key length
     */
    uint32_t hash(const string& key) const {
        uint32_t h = 0x811c9dc5;  // FNV offset basis
        for (char c : key) {
            h ^= static_cast<uint32_t>(static_cast<unsigned char>(c));
            h *= 0x01000193;  // FNV prime
        }
        return h;
    }

public:
    ConsistentHashSolution(int replicas) : replicas(replicas) {}

    /**
     * Adds a physical node to the hash ring.
     * Creates 'replicas' virtual nodes for better distribution.
     * Time complexity: O(replicas * log n)
     */
    void addNode(const string& node) {
        // Check if node already exists
        if (find(nodes.begin(), nodes.end(), node) != nodes.end()) {
            return;
        }

        // Add to nodes list
        nodes.push_back(node);

        // Create virtual nodes
        for (int i = 0; i < replicas; i++) {
            string virtualKey = node + "-" + to_string(i);
            uint32_t hashValue = hash(virtualKey);
            ring[hashValue] = node;
        }
    }

    /**
     * Removes a physical node and all its virtual nodes from the ring.
     * Time complexity: O(replicas * log n)
     */
    void removeNode(const string& node) {
        // Find and remove from nodes vector
        auto it = find(nodes.begin(), nodes.end(), node);
        if (it == nodes.end()) {
            return;
        }
        nodes.erase(it);

        // Remove virtual nodes from ring
        for (int i = 0; i < replicas; i++) {
            string virtualKey = node + "-" + to_string(i);
            uint32_t hashValue = hash(virtualKey);
            ring.erase(hashValue);
        }
    }

    /**
     * Finds the node responsible for a given key.
     * Uses clockwise traversal on the hash ring.
     * Time complexity: O(log n)
     */
    string getNode(const string& key) const {
        if (ring.empty()) {
            return "";
        }

        uint32_t hashValue = hash(key);

        // Find the first node with hash >= key hash
        auto it = ring.lower_bound(hashValue);

        // If no entry found, wrap around to first node
        if (it == ring.end()) {
            it = ring.begin();
        }

        return it->second;
    }

    /**
     * Returns the list of all physical nodes.
     */
    vector<string> getNodes() const {
        return nodes;
    }

    /**
     * Returns the number of physical nodes in the ring.
     */
    int size() const {
        return nodes.size();
    }

    /**
     * Checks if the ring is empty.
     */
    bool isEmpty() const {
        return ring.empty();
    }
};

// ============================================================================
// Solution: GCounter - A CRDT grow-only counter
// ============================================================================

/**
 * Solution: GCounter represents a grow-only counter CRDT.
 * Each node maintains its own counter, merging takes max for convergence.
 */
class GCounterSolution {
private:
    string nodeId;                      // ID of the local node
    unordered_map<string, long> counts; // node ID -> count mapping

public:
    GCounterSolution(const string& nodeId) : nodeId(nodeId) {}

    /**
     * Increments the counter for the local node by 1.
     * Time complexity: O(1)
     */
    void increment() {
        counts[nodeId]++;
    }

    /**
     * Increases the counter for the local node by a specified amount.
     * Only allows positive increments (GCounter can only grow).
     * Time complexity: O(1)
     */
    void incrementBy(long amount) {
        if (amount > 0) {
            counts[nodeId] += amount;
        }
    }

    /**
     * Returns the total count across all nodes.
     * Time complexity: O(n) where n is number of nodes
     */
    long value() const {
        long total = 0;
        for (const auto& [_, count] : counts) {
            total += count;
        }
        return total;
    }

    /**
     * Returns the count for the local node only.
     */
    long localValue() const {
        auto it = counts.find(nodeId);
        return (it != counts.end()) ? it->second : 0;
    }

    /**
     * Merges another GCounter into this one.
     * Takes the maximum value for each node (ensures convergence).
     *
     * CRDT properties ensured:
     * - Commutative: merge(A,B) = merge(B,A)
     * - Associative: merge(merge(A,B),C) = merge(A,merge(B,C))
     * - Idempotent: merge(A,A) = A
     *
     * Time complexity: O(n) where n is number of nodes in other
     */
    void merge(const GCounterSolution& other) {
        for (const auto& [node, count] : other.counts) {
            auto it = counts.find(node);
            if (it != counts.end()) {
                counts[node] = max(it->second, count);
            } else {
                counts[node] = count;
            }
        }
    }

    /**
     * Returns a copy of the internal counts map.
     */
    unordered_map<string, long> getCounts() const {
        return counts;
    }

    /**
     * Returns the node ID.
     */
    string getNodeId() const {
        return nodeId;
    }
};

// ============================================================================
// Solution: PNCounter - A CRDT that supports increment and decrement
// ============================================================================

/**
 * Solution: PNCounter represents a positive-negative counter CRDT.
 * Implemented as two GCounters for supporting decrements.
 */
class PNCounterSolution {
private:
    string nodeId;           // ID of the local node
    GCounterSolution p;      // positive counter (increments)
    GCounterSolution n;      // negative counter (decrements)

public:
    PNCounterSolution(const string& nodeId) : nodeId(nodeId), p(nodeId), n(nodeId) {}

    /**
     * Increases the counter by 1.
     */
    void increment() {
        p.increment();
    }

    /**
     * Decreases the counter by 1.
     */
    void decrement() {
        n.increment();
    }

    /**
     * Returns the current value (P - N).
     */
    long value() const {
        return p.value() - n.value();
    }

    /**
     * Merges another PNCounter into this one.
     * Merge both P and N counters separately.
     */
    void merge(const PNCounterSolution& other) {
        p.merge(other.p);
        n.merge(other.n);
    }

    /**
     * Returns a reference to the positive counter for testing.
     */
    const GCounterSolution& getPositive() const {
        return p;
    }

    /**
     * Returns a reference to the negative counter for testing.
     */
    const GCounterSolution& getNegative() const {
        return n;
    }
};

// Demonstration
int main() {
    cout << "=== Chapter 15: Distributed Systems Solutions ===" << endl << endl;

    // =========================================================================
    // Demo Consistent Hashing
    // =========================================================================
    cout << "1. Consistent Hashing" << endl;
    cout << string(50, '-') << endl;

    ConsistentHashSolution ch(3); // 3 virtual nodes per physical node

    // Add nodes
    ch.addNode("server-1");
    ch.addNode("server-2");
    ch.addNode("server-3");
    cout << "Added 3 servers with 3 virtual nodes each" << endl;
    cout << "Total nodes: " << ch.size() << endl;

    // Test key distribution
    cout << "\nKey distribution:" << endl;
    vector<string> keys = {"user:100", "user:200", "user:300", "order:1", "order:2"};
    map<string, vector<string>> distribution;

    for (const string& key : keys) {
        string node = ch.getNode(key);
        cout << "  " << key << " -> " << node << endl;
        distribution[node].push_back(key);
    }

    // Test consistency
    cout << "\nConsistency test (same key, 5 queries):" << endl;
    string testKey = "consistent-test";
    string expectedNode = ch.getNode(testKey);
    bool consistent = true;
    for (int i = 0; i < 5; i++) {
        if (ch.getNode(testKey) != expectedNode) {
            consistent = false;
            break;
        }
    }
    cout << "  Key always maps to same node: " << (consistent ? "true" : "false") << endl;

    // Test node removal
    cout << "\nRemoving server-2..." << endl;
    ch.removeNode("server-2");
    cout << "Remaining nodes: ";
    for (const auto& n : ch.getNodes()) {
        cout << n << " ";
    }
    cout << endl;
    cout << "Key 'user:100' now maps to: " << ch.getNode("user:100") << endl;

    // =========================================================================
    // Demo GCounter
    // =========================================================================
    cout << "\n2. GCounter (Grow-only CRDT)" << endl;
    cout << string(50, '-') << endl;

    // Simulate distributed counting
    GCounterSolution gc1("node-1");
    GCounterSolution gc2("node-2");
    GCounterSolution gc3("node-3");

    // Each node increments independently
    gc1.increment();
    gc1.increment();
    gc1.incrementBy(3);
    cout << "Node-1 increments: 1 + 1 + 3 = " << gc1.value() << endl;

    gc2.incrementBy(10);
    cout << "Node-2 increments: 10 = " << gc2.value() << endl;

    gc3.increment();
    cout << "Node-3 increments: 1 = " << gc3.value() << endl;

    // Merge all into gc1
    gc1.merge(gc2);
    gc1.merge(gc3);
    cout << "\nAfter merging all into gc1: " << gc1.value() << endl;
    cout << "Counts by node: { ";
    for (const auto& [node, count] : gc1.getCounts()) {
        cout << node << ": " << count << " ";
    }
    cout << "}" << endl;

    // Demonstrate idempotence
    gc1.merge(gc2);
    cout << "After re-merging gc2: " << gc1.value() << " (idempotent)" << endl;

    // =========================================================================
    // Demo PNCounter
    // =========================================================================
    cout << "\n3. PNCounter (Positive-Negative CRDT)" << endl;
    cout << string(50, '-') << endl;

    PNCounterSolution pn1("node-A");
    PNCounterSolution pn2("node-B");

    // Node A: +5 -2 = 3
    for (int i = 0; i < 5; i++) pn1.increment();
    pn1.decrement();
    pn1.decrement();
    cout << "Node-A: +5 -2 = " << pn1.value() << endl;

    // Node B: +3 -4 = -1
    for (int i = 0; i < 3; i++) pn2.increment();
    for (int i = 0; i < 4; i++) pn2.decrement();
    cout << "Node-B: +3 -4 = " << pn2.value() << endl;

    // Merge
    pn1.merge(pn2);
    cout << "After merge: " << pn1.value() << " (should be 3 + (-1) = 2)" << endl;

    cout << "\nAll solutions demonstrated successfully!" << endl;

    return 0;
}

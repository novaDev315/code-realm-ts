// Chapter 15: Core of the Architect - Final Boss
// Distributed Systems: Consistent Hashing and CRDTs

#include <string>
#include <vector>
#include <map>
#include <unordered_map>
#include <algorithm>
#include <cstdint>

using namespace std;

/**
 * ConsistentHash implements consistent hashing for distributed systems.
 * Uses virtual nodes (replicas) to improve key distribution.
 */
class ConsistentHash {
private:
    map<uint32_t, string> ring;     // hash -> node mapping (sorted)
    vector<string> nodes;            // list of physical nodes
    int replicas;                    // number of virtual nodes per physical node

    /**
     * Computes a hash for the given key.
     * TODO: Implement hash function
     * Use FNV-1a or similar hash algorithm
     */
    uint32_t hash(const string& key) const {
        // TODO: Implement hash function
        // FNV-1a algorithm:
        // uint32_t hash = 0x811c9dc5;  // FNV offset basis
        // for (char c : key) {
        //     hash ^= static_cast<uint32_t>(c);
        //     hash *= 0x01000193;  // FNV prime
        // }
        // return hash;
        return 0;
    }

public:
    ConsistentHash(int replicas) : replicas(replicas) {}

    /**
     * Adds a physical node to the hash ring.
     * Creates 'replicas' virtual nodes for better distribution.
     *
     * TODO: Implement adding a node to the consistent hash ring
     * 1. Check if node already exists in nodes vector
     * 2. Add node to nodes vector
     * 3. For each replica (0 to replicas-1):
     *    - Create virtual node key: node + "-" + to_string(i)
     *    - Compute hash using hash()
     *    - Add to ring mapping hash -> node
     * 4. std::map automatically maintains sorted order
     */
    void addNode(const string& node) {
        // TODO: Implement this method
    }

    /**
     * Removes a physical node and all its virtual nodes from the ring.
     *
     * TODO: Implement removing a node from the consistent hash ring
     * 1. Find and remove node from nodes vector
     * 2. For each replica (0 to replicas-1):
     *    - Create virtual node key: node + "-" + to_string(i)
     *    - Compute hash and remove from ring
     */
    void removeNode(const string& node) {
        // TODO: Implement this method
    }

    /**
     * Finds the node responsible for a given key.
     * Uses clockwise traversal on the hash ring.
     *
     * TODO: Implement getting the node for a key
     * 1. If ring is empty, return empty string
     * 2. Compute hash of the key
     * 3. Use ring.lower_bound(hash) to find first hash >= key hash
     * 4. If iterator is end(), wrap around to ring.begin()
     * 5. Return the node from the iterator
     */
    string getNode(const string& key) const {
        // TODO: Implement this method
        return "";
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
// GCounter - A CRDT (Conflict-free Replicated Data Type) grow-only counter
// Each node maintains its own counter, and counters can only be incremented
// Merging takes the maximum value for each node
// ============================================================================

/**
 * GCounter represents a grow-only counter CRDT.
 */
class GCounter {
private:
    string nodeId;                      // ID of the local node
    unordered_map<string, long> counts; // node ID -> count mapping

public:
    GCounter(const string& nodeId) : nodeId(nodeId) {}

    /**
     * Increments the counter for the local node by 1.
     *
     * TODO: Implement incrementing the local counter
     * Increment counts[nodeId] by 1
     */
    void increment() {
        // TODO: Implement this method
    }

    /**
     * Increases the counter for the local node by a specified amount.
     * Only allows positive increments (GCounter can only grow).
     *
     * TODO: Implement incrementing by a specific amount
     * Only allow positive increments
     * Increment counts[nodeId] by amount
     */
    void incrementBy(long amount) {
        // TODO: Implement this method
    }

    /**
     * Returns the total count across all nodes.
     *
     * TODO: Implement getting the total value
     * Sum all values in counts map
     */
    long value() const {
        // TODO: Implement this method
        return 0;
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
     * TODO: Implement merging two GCounters
     * For each node in other.counts:
     *   counts[node] = max(counts[node], other.counts[node])
     * This ensures CRDTs converge to the same value
     */
    void merge(const GCounter& other) {
        // TODO: Implement this method
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
// PNCounter - A CRDT counter that supports both increment and decrement
// Implemented as two GCounters: one for increments (P) and one for decrements (N)
// ============================================================================

/**
 * PNCounter represents a positive-negative counter CRDT.
 */
class PNCounter {
private:
    string nodeId;    // ID of the local node
    GCounter p;       // positive counter (increments)
    GCounter n;       // negative counter (decrements)

public:
    PNCounter(const string& nodeId) : nodeId(nodeId), p(nodeId), n(nodeId) {}

    /**
     * Increases the counter by 1.
     *
     * TODO: Implement increment using the P counter
     */
    void increment() {
        // TODO: Implement this method
    }

    /**
     * Decreases the counter by 1.
     *
     * TODO: Implement decrement using the N counter
     */
    void decrement() {
        // TODO: Implement this method
    }

    /**
     * Returns the current value (P - N).
     *
     * TODO: Implement getting the value
     * Return p.value() - n.value()
     */
    long value() const {
        // TODO: Implement this method
        return 0;
    }

    /**
     * Merges another PNCounter into this one.
     *
     * TODO: Implement merging two PNCounters
     * Merge both P and N counters separately
     */
    void merge(const PNCounter& other) {
        // TODO: Implement this method
    }

    /**
     * Returns a reference to the positive counter for testing.
     */
    const GCounter& getPositive() const {
        return p;
    }

    /**
     * Returns a reference to the negative counter for testing.
     */
    const GCounter& getNegative() const {
        return n;
    }
};

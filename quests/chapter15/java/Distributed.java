// Chapter 15: Core of the Architect - Final Boss
// Distributed Systems: Consistent Hashing and CRDTs

import java.util.*;

/**
 * ConsistentHash implements consistent hashing for distributed systems.
 * Uses virtual nodes (replicas) to improve key distribution.
 */
class ConsistentHash {
    private TreeMap<Integer, String> ring;  // hash -> node mapping
    private List<String> nodes;              // list of physical nodes
    private int replicas;                    // number of virtual nodes per physical node

    public ConsistentHash(int replicas) {
        this.ring = new TreeMap<>();
        this.nodes = new ArrayList<>();
        this.replicas = replicas;
    }

    /**
     * Computes a hash for the given key.
     * Uses a simple but effective hash function.
     */
    private int hash(String key) {
        // TODO: Implement hash function
        // Use key.hashCode() and ensure positive value with Math.abs()
        // Or implement a custom hash like FNV-1a
        return 0;
    }

    /**
     * Adds a physical node to the hash ring.
     * Creates 'replicas' virtual nodes for better distribution.
     *
     * TODO: Implement adding a node to the consistent hash ring
     * 1. Check if node already exists in nodes list
     * 2. Add node to nodes list
     * 3. For each replica (0 to replicas-1):
     *    - Create virtual node key: node + "-" + i
     *    - Compute hash using hash()
     *    - Add to ring mapping hash -> node
     * 4. TreeMap automatically maintains sorted order
     */
    public void addNode(String node) {
        // TODO: Implement this method
    }

    /**
     * Removes a physical node and all its virtual nodes from the ring.
     *
     * TODO: Implement removing a node from the consistent hash ring
     * 1. Check if node exists and remove from nodes list
     * 2. For each replica (0 to replicas-1):
     *    - Create virtual node key: node + "-" + i
     *    - Compute hash and remove from ring
     */
    public void removeNode(String node) {
        // TODO: Implement this method
    }

    /**
     * Finds the node responsible for a given key.
     * Uses clockwise traversal on the hash ring.
     *
     * TODO: Implement getting the node for a key
     * 1. If ring is empty, return null
     * 2. Compute hash of the key
     * 3. Use ring.ceilingEntry(hash) to find first hash >= key hash
     * 4. If no entry found (key hash > all in ring), wrap around to first
     *    Use ring.firstEntry()
     * 5. Return the node from the entry
     */
    public String getNode(String key) {
        // TODO: Implement this method
        return null;
    }

    /**
     * Returns the list of all physical nodes.
     */
    public List<String> getNodes() {
        return new ArrayList<>(nodes);
    }

    /**
     * Returns the number of physical nodes in the ring.
     */
    public int size() {
        return nodes.size();
    }
}

// ============================================================================
// GCounter - A CRDT (Conflict-free Replicated Data Type) grow-only counter
// Each node maintains its own counter, and counters can only be incremented
// Merging takes the maximum value for each node
// ============================================================================

/**
 * GCounter represents a grow-only counter CRDT.
 */
class GCounter {
    private String nodeId;              // ID of the local node
    private Map<String, Long> counts;   // node ID -> count mapping

    public GCounter(String nodeId) {
        this.nodeId = nodeId;
        this.counts = new HashMap<>();
    }

    /**
     * Increments the counter for the local node by 1.
     *
     * TODO: Implement incrementing the local counter
     * Increment counts.get(nodeId) by 1 (handle null case)
     */
    public void increment() {
        // TODO: Implement this method
    }

    /**
     * Increases the counter for the local node by a specified amount.
     * Only allows positive increments (GCounter can only grow).
     *
     * TODO: Implement incrementing by a specific amount
     * Only allow positive increments
     * Increment counts.get(nodeId) by amount
     */
    public void incrementBy(long amount) {
        // TODO: Implement this method
    }

    /**
     * Returns the total count across all nodes.
     *
     * TODO: Implement getting the total value
     * Sum all values in counts map
     */
    public long value() {
        // TODO: Implement this method
        return 0;
    }

    /**
     * Returns the count for the local node only.
     */
    public long localValue() {
        return counts.getOrDefault(nodeId, 0L);
    }

    /**
     * Merges another GCounter into this one.
     * Takes the maximum value for each node (ensures convergence).
     *
     * TODO: Implement merging two GCounters
     * For each node in other.counts:
     *   counts.put(node, max(counts.get(node), other.counts.get(node)))
     * This ensures CRDTs converge to the same value
     */
    public void merge(GCounter other) {
        // TODO: Implement this method
    }

    /**
     * Returns a copy of the internal counts map.
     */
    public Map<String, Long> getCounts() {
        return new HashMap<>(counts);
    }

    /**
     * Returns the node ID.
     */
    public String getNodeId() {
        return nodeId;
    }
}

// ============================================================================
// PNCounter - A CRDT counter that supports both increment and decrement
// Implemented as two GCounters: one for increments (P) and one for decrements (N)
// ============================================================================

/**
 * PNCounter represents a positive-negative counter CRDT.
 */
class PNCounter {
    private String nodeId;    // ID of the local node
    private GCounter p;       // positive counter (increments)
    private GCounter n;       // negative counter (decrements)

    public PNCounter(String nodeId) {
        this.nodeId = nodeId;
        this.p = new GCounter(nodeId);
        this.n = new GCounter(nodeId);
    }

    /**
     * Increases the counter by 1.
     *
     * TODO: Implement increment using the P counter
     */
    public void increment() {
        // TODO: Implement this method
    }

    /**
     * Decreases the counter by 1.
     *
     * TODO: Implement decrement using the N counter
     */
    public void decrement() {
        // TODO: Implement this method
    }

    /**
     * Returns the current value (P - N).
     *
     * TODO: Implement getting the value
     * Return p.value() - n.value()
     */
    public long value() {
        // TODO: Implement this method
        return 0;
    }

    /**
     * Merges another PNCounter into this one.
     *
     * TODO: Implement merging two PNCounters
     * Merge both P and N counters separately
     */
    public void merge(PNCounter other) {
        // TODO: Implement this method
    }

    /**
     * Returns the positive counter for testing.
     */
    public GCounter getPositive() {
        return p;
    }

    /**
     * Returns the negative counter for testing.
     */
    public GCounter getNegative() {
        return n;
    }
}

// Main class for the stub file
public class Distributed {
    public static void main(String[] args) {
        System.out.println("Chapter 15: Core of the Architect - Final Boss");
        System.out.println("Distributed Systems: Consistent Hashing and CRDTs");
        System.out.println("\nImplement the TODOs in ConsistentHash, GCounter, and PNCounter classes!");
    }
}

// Reference solutions for Chapter 15: Core of the Architect - Final Boss
// Distributed Systems: Consistent Hashing and CRDTs

import java.util.*;

/**
 * Solution: ConsistentHash implements consistent hashing for distributed systems.
 * Uses virtual nodes (replicas) to improve key distribution.
 */
class ConsistentHashSolution {
    private TreeMap<Integer, String> ring;  // hash -> node mapping
    private List<String> nodes;              // list of physical nodes
    private int replicas;                    // number of virtual nodes per physical node

    public ConsistentHashSolution(int replicas) {
        this.ring = new TreeMap<>();
        this.nodes = new ArrayList<>();
        this.replicas = replicas;
    }

    /**
     * Computes a hash for the given key.
     * Uses FNV-1a inspired hash for better distribution.
     */
    private int hash(String key) {
        int hash = 0x811c9dc5; // FNV offset basis
        for (byte b : key.getBytes()) {
            hash ^= b;
            hash *= 0x01000193; // FNV prime
        }
        return hash;
    }

    /**
     * Adds a physical node to the hash ring.
     * Creates 'replicas' virtual nodes for better distribution.
     * Time complexity: O(replicas * log n) where n is total virtual nodes
     */
    public void addNode(String node) {
        // Check if node already exists
        if (nodes.contains(node)) {
            return;
        }

        // Add to nodes list
        nodes.add(node);

        // Create virtual nodes
        for (int i = 0; i < replicas; i++) {
            String virtualKey = node + "-" + i;
            int hashValue = hash(virtualKey);
            ring.put(hashValue, node);
        }
    }

    /**
     * Removes a physical node and all its virtual nodes from the ring.
     * Time complexity: O(replicas * log n)
     */
    public void removeNode(String node) {
        // Check if node exists
        if (!nodes.contains(node)) {
            return;
        }

        // Remove from nodes list
        nodes.remove(node);

        // Remove virtual nodes from ring
        for (int i = 0; i < replicas; i++) {
            String virtualKey = node + "-" + i;
            int hashValue = hash(virtualKey);
            ring.remove(hashValue);
        }
    }

    /**
     * Finds the node responsible for a given key.
     * Uses clockwise traversal on the hash ring.
     * Time complexity: O(log n)
     */
    public String getNode(String key) {
        if (ring.isEmpty()) {
            return null;
        }

        int hashValue = hash(key);

        // Find the first node with hash >= key hash
        Map.Entry<Integer, String> entry = ring.ceilingEntry(hashValue);

        // If no entry found, wrap around to first node
        if (entry == null) {
            entry = ring.firstEntry();
        }

        return entry.getValue();
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
// Solution: GCounter - A CRDT grow-only counter
// ============================================================================

/**
 * Solution: GCounter represents a grow-only counter CRDT.
 * Each node maintains its own counter, merging takes max for convergence.
 */
class GCounterSolution {
    private String nodeId;              // ID of the local node
    private Map<String, Long> counts;   // node ID -> count mapping

    public GCounterSolution(String nodeId) {
        this.nodeId = nodeId;
        this.counts = new HashMap<>();
    }

    /**
     * Increments the counter for the local node by 1.
     * Time complexity: O(1)
     */
    public void increment() {
        counts.merge(nodeId, 1L, Long::sum);
    }

    /**
     * Increases the counter for the local node by a specified amount.
     * Only allows positive increments (GCounter can only grow).
     * Time complexity: O(1)
     */
    public void incrementBy(long amount) {
        if (amount > 0) {
            counts.merge(nodeId, amount, Long::sum);
        }
    }

    /**
     * Returns the total count across all nodes.
     * Time complexity: O(n) where n is number of nodes
     */
    public long value() {
        return counts.values().stream().mapToLong(Long::longValue).sum();
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
     * CRDT properties ensured:
     * - Commutative: merge(A,B) = merge(B,A)
     * - Associative: merge(merge(A,B),C) = merge(A,merge(B,C))
     * - Idempotent: merge(A,A) = A
     *
     * Time complexity: O(n) where n is number of nodes in other
     */
    public void merge(GCounterSolution other) {
        for (Map.Entry<String, Long> entry : other.counts.entrySet()) {
            String node = entry.getKey();
            Long otherCount = entry.getValue();
            Long myCount = counts.getOrDefault(node, 0L);
            counts.put(node, Math.max(myCount, otherCount));
        }
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
// Solution: PNCounter - A CRDT that supports increment and decrement
// ============================================================================

/**
 * Solution: PNCounter represents a positive-negative counter CRDT.
 * Implemented as two GCounters for supporting decrements.
 */
class PNCounterSolution {
    private String nodeId;           // ID of the local node
    private GCounterSolution p;      // positive counter (increments)
    private GCounterSolution n;      // negative counter (decrements)

    public PNCounterSolution(String nodeId) {
        this.nodeId = nodeId;
        this.p = new GCounterSolution(nodeId);
        this.n = new GCounterSolution(nodeId);
    }

    /**
     * Increases the counter by 1.
     */
    public void increment() {
        p.increment();
    }

    /**
     * Decreases the counter by 1.
     */
    public void decrement() {
        n.increment();
    }

    /**
     * Returns the current value (P - N).
     */
    public long value() {
        return p.value() - n.value();
    }

    /**
     * Merges another PNCounter into this one.
     * Merge both P and N counters separately.
     */
    public void merge(PNCounterSolution other) {
        p.merge(other.p);
        n.merge(other.n);
    }

    /**
     * Returns the positive counter for testing.
     */
    public GCounterSolution getPositive() {
        return p;
    }

    /**
     * Returns the negative counter for testing.
     */
    public GCounterSolution getNegative() {
        return n;
    }
}

// Main demonstration class
public class SOLUTIONS {
    public static void main(String[] args) {
        System.out.println("=== Chapter 15: Distributed Systems Solutions ===\n");

        // =====================================================================
        // Demo Consistent Hashing
        // =====================================================================
        System.out.println("1. Consistent Hashing");
        System.out.println("-".repeat(50));

        ConsistentHashSolution ch = new ConsistentHashSolution(3); // 3 virtual nodes

        // Add nodes
        ch.addNode("server-1");
        ch.addNode("server-2");
        ch.addNode("server-3");
        System.out.println("Added 3 servers with 3 virtual nodes each");
        System.out.println("Total nodes: " + ch.size());

        // Test key distribution
        System.out.println("\nKey distribution:");
        String[] keys = {"user:100", "user:200", "user:300", "order:1", "order:2"};
        Map<String, List<String>> distribution = new HashMap<>();

        for (String key : keys) {
            String node = ch.getNode(key);
            System.out.println("  " + key + " -> " + node);
            distribution.computeIfAbsent(node, k -> new ArrayList<>()).add(key);
        }

        // Test consistency
        System.out.println("\nConsistency test (same key, 5 queries):");
        String testKey = "consistent-test";
        String expectedNode = ch.getNode(testKey);
        boolean consistent = true;
        for (int i = 0; i < 5; i++) {
            if (!ch.getNode(testKey).equals(expectedNode)) {
                consistent = false;
                break;
            }
        }
        System.out.println("  Key always maps to same node: " + consistent);

        // Test node removal
        System.out.println("\nRemoving server-2...");
        ch.removeNode("server-2");
        System.out.println("Remaining nodes: " + ch.getNodes());
        System.out.println("Key 'user:100' now maps to: " + ch.getNode("user:100"));

        // =====================================================================
        // Demo GCounter
        // =====================================================================
        System.out.println("\n2. GCounter (Grow-only CRDT)");
        System.out.println("-".repeat(50));

        // Simulate distributed counting
        GCounterSolution gc1 = new GCounterSolution("node-1");
        GCounterSolution gc2 = new GCounterSolution("node-2");
        GCounterSolution gc3 = new GCounterSolution("node-3");

        // Each node increments independently
        gc1.increment();
        gc1.increment();
        gc1.incrementBy(3);
        System.out.println("Node-1 increments: 1 + 1 + 3 = " + gc1.value());

        gc2.incrementBy(10);
        System.out.println("Node-2 increments: 10 = " + gc2.value());

        gc3.increment();
        System.out.println("Node-3 increments: 1 = " + gc3.value());

        // Merge all into gc1
        gc1.merge(gc2);
        gc1.merge(gc3);
        System.out.println("\nAfter merging all into gc1: " + gc1.value());
        System.out.println("Counts by node: " + gc1.getCounts());

        // Demonstrate idempotence
        gc1.merge(gc2);
        System.out.println("After re-merging gc2: " + gc1.value() + " (idempotent)");

        // =====================================================================
        // Demo PNCounter
        // =====================================================================
        System.out.println("\n3. PNCounter (Positive-Negative CRDT)");
        System.out.println("-".repeat(50));

        PNCounterSolution pn1 = new PNCounterSolution("node-A");
        PNCounterSolution pn2 = new PNCounterSolution("node-B");

        // Node A: +5 -2 = 3
        for (int i = 0; i < 5; i++) pn1.increment();
        pn1.decrement();
        pn1.decrement();
        System.out.println("Node-A: +5 -2 = " + pn1.value());

        // Node B: +3 -4 = -1
        for (int i = 0; i < 3; i++) pn2.increment();
        for (int i = 0; i < 4; i++) pn2.decrement();
        System.out.println("Node-B: +3 -4 = " + pn2.value());

        // Merge
        pn1.merge(pn2);
        System.out.println("After merge: " + pn1.value() + " (should be 3 + (-1) = 2)");

        System.out.println("\nAll solutions demonstrated successfully!");
    }
}

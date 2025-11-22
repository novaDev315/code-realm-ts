// Test file for Chapter 15: Core of the Architect - Final Boss
// Distributed Systems: Consistent Hashing and CRDTs

import java.util.*;

public class Check {
    public static void main(String[] args) {
        boolean passed = true;

        // ============================================================================
        // Test ConsistentHash
        // ============================================================================

        System.out.println("Testing ConsistentHash...");

        // Test 1: Basic node operations
        System.out.println("\nTest 1: Adding nodes to consistent hash ring...");
        ConsistentHash ch = new ConsistentHash(3); // 3 virtual nodes per physical node

        ch.addNode("server-1");
        ch.addNode("server-2");
        ch.addNode("server-3");

        if (ch.size() != 3) {
            System.err.println("  FAIL: Expected 3 nodes, got " + ch.size());
            passed = false;
        } else {
            System.out.println("  PASS: Added 3 nodes successfully");
        }

        // Test 2: Key distribution
        System.out.println("\nTest 2: Key distribution across nodes...");
        String key1Node = ch.getNode("user:123");
        String key2Node = ch.getNode("user:456");
        String key3Node = ch.getNode("user:789");

        if (key1Node == null || key2Node == null || key3Node == null) {
            System.err.println("  FAIL: GetNode should return a node for any key");
            passed = false;
        } else {
            System.out.println("  PASS: Keys distributed - user:123->" + key1Node +
                             ", user:456->" + key2Node + ", user:789->" + key3Node);
        }

        // Test 3: Consistency
        System.out.println("\nTest 3: Same key should always return same node...");
        String consistentNode = ch.getNode("consistent-key");
        boolean isConsistent = true;
        for (int i = 0; i < 10; i++) {
            if (!Objects.equals(ch.getNode("consistent-key"), consistentNode)) {
                isConsistent = false;
                break;
            }
        }
        if (!isConsistent) {
            System.err.println("  FAIL: Same key should map to same node");
            passed = false;
        } else {
            System.out.println("  PASS: Consistent hashing is consistent");
        }

        // Test 4: Node removal
        System.out.println("\nTest 4: Removing a node...");
        ch.removeNode("server-2");
        if (ch.size() != 2) {
            System.err.println("  FAIL: Expected 2 nodes after removal, got " + ch.size());
            passed = false;
        } else {
            System.out.println("  PASS: Node removed successfully");
        }

        // Test 5: Keys should still map after node removal
        System.out.println("\nTest 5: Keys should still map after node removal...");
        String keyAfterRemoval = ch.getNode("any-key");
        if (keyAfterRemoval == null) {
            System.err.println("  FAIL: GetNode should still work after node removal");
            passed = false;
        } else if ("server-2".equals(keyAfterRemoval)) {
            System.err.println("  FAIL: Key should not map to removed node");
            passed = false;
        } else {
            System.out.println("  PASS: Key mapped to " + keyAfterRemoval + " after server-2 removal");
        }

        // Test 6: Empty ring
        System.out.println("\nTest 6: Empty ring behavior...");
        ConsistentHash emptyRing = new ConsistentHash(3);
        String emptyResult = emptyRing.getNode("some-key");
        if (emptyResult != null) {
            System.err.println("  FAIL: Empty ring should return null");
            passed = false;
        } else {
            System.out.println("  PASS: Empty ring returns null");
        }

        // ============================================================================
        // Test GCounter (CRDT)
        // ============================================================================

        System.out.println("\n" + "=".repeat(50));
        System.out.println("Testing GCounter (CRDT)...");

        // Test 7: Basic increment
        System.out.println("\nTest 7: Basic increment operation...");
        GCounter gc1 = new GCounter("node-1");
        gc1.increment();
        gc1.increment();
        gc1.increment();

        if (gc1.value() != 3) {
            System.err.println("  FAIL: Expected value 3 after 3 increments, got " + gc1.value());
            passed = false;
        } else {
            System.out.println("  PASS: GCounter increments correctly");
        }

        // Test 8: IncrementBy
        System.out.println("\nTest 8: IncrementBy operation...");
        gc1.incrementBy(5);
        if (gc1.value() != 8) {
            System.err.println("  FAIL: Expected value 8 after IncrementBy(5), got " + gc1.value());
            passed = false;
        } else {
            System.out.println("  PASS: IncrementBy works correctly");
        }

        // Test 9: Merge operation
        System.out.println("\nTest 9: Merging two GCounters...");
        GCounter gc2 = new GCounter("node-2");
        gc2.increment();
        gc2.increment();

        gc1.merge(gc2);

        // gc1 should now have counts from both nodes: node-1=8, node-2=2 -> total 10
        if (gc1.value() != 10) {
            System.err.println("  FAIL: Expected merged value 10, got " + gc1.value());
            passed = false;
        } else {
            System.out.println("  PASS: GCounter merge works correctly");
        }

        // Test 10: Merge idempotence
        System.out.println("\nTest 10: Merge idempotence (merging same counter twice)...");
        gc1.merge(gc2);
        if (gc1.value() != 10) {
            System.err.println("  FAIL: Expected value still 10 after re-merge, got " + gc1.value());
            passed = false;
        } else {
            System.out.println("  PASS: Merge is idempotent");
        }

        // Test 11: Merge commutativity
        System.out.println("\nTest 11: Merge commutativity...");
        GCounter gcA = new GCounter("A");
        gcA.incrementBy(5);

        GCounter gcB = new GCounter("B");
        gcB.incrementBy(3);

        GCounter gcC = new GCounter("C");
        gcC.incrementBy(7);

        // Merge A->B->C
        GCounter gc1Copy = new GCounter("copy1");
        gc1Copy.merge(gcA);
        gc1Copy.merge(gcB);
        gc1Copy.merge(gcC);

        // Merge C->A->B
        GCounter gc2Copy = new GCounter("copy2");
        gc2Copy.merge(gcC);
        gc2Copy.merge(gcA);
        gc2Copy.merge(gcB);

        if (gc1Copy.value() != gc2Copy.value()) {
            System.err.println("  FAIL: Different merge orders should give same result: " +
                             gc1Copy.value() + " vs " + gc2Copy.value());
            passed = false;
        } else {
            System.out.println("  PASS: Merge is commutative (both give " + gc1Copy.value() + ")");
        }

        // ============================================================================
        // Test PNCounter
        // ============================================================================

        System.out.println("\n" + "=".repeat(50));
        System.out.println("Testing PNCounter (CRDT)...");

        // Test 12: Basic increment/decrement
        System.out.println("\nTest 12: Increment and decrement operations...");
        PNCounter pn1 = new PNCounter("node-1");
        pn1.increment();
        pn1.increment();
        pn1.increment();
        pn1.decrement();

        if (pn1.value() != 2) {
            System.err.println("  FAIL: Expected value 2 (3 inc - 1 dec), got " + pn1.value());
            passed = false;
        } else {
            System.out.println("  PASS: PNCounter increment/decrement works correctly");
        }

        // Test 13: PNCounter merge
        System.out.println("\nTest 13: Merging two PNCounters...");
        PNCounter pn2 = new PNCounter("node-2");
        pn2.increment();
        pn2.decrement();
        pn2.decrement();

        pn1.merge(pn2);

        // pn1: node-1 has +3-1=2, node-2 has +1-2=-1, total should be 1
        if (pn1.value() != 1) {
            System.err.println("  FAIL: Expected merged value 1, got " + pn1.value());
            passed = false;
        } else {
            System.out.println("  PASS: PNCounter merge works correctly");
        }

        // ============================================================================
        // Summary
        // ============================================================================

        System.out.println("\n" + "=".repeat(50));
        if (passed) {
            System.out.println("ALL TESTS PASSED! You have mastered the Core of the Architect!");
        } else {
            System.out.println("Some tests FAILED. Review your implementation.");
            System.exit(1);
        }
    }
}

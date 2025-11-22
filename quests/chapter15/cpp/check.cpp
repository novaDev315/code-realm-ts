// Test file for Chapter 15: Core of the Architect - Final Boss
// Distributed Systems: Consistent Hashing and CRDTs

#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <unordered_map>
#include <algorithm>
#include <cstdint>

using namespace std;

// Include the implementation
#include "distributed.cpp"

int main() {
    bool passed = true;

    // ============================================================================
    // Test ConsistentHash
    // ============================================================================

    cout << "Testing ConsistentHash..." << endl;

    // Test 1: Basic node operations
    cout << "\nTest 1: Adding nodes to consistent hash ring..." << endl;
    ConsistentHash ch(3); // 3 virtual nodes per physical node

    ch.addNode("server-1");
    ch.addNode("server-2");
    ch.addNode("server-3");

    if (ch.size() != 3) {
        cerr << "  FAIL: Expected 3 nodes, got " << ch.size() << endl;
        passed = false;
    } else {
        cout << "  PASS: Added 3 nodes successfully" << endl;
    }

    // Test 2: Key distribution
    cout << "\nTest 2: Key distribution across nodes..." << endl;
    string key1Node = ch.getNode("user:123");
    string key2Node = ch.getNode("user:456");
    string key3Node = ch.getNode("user:789");

    if (key1Node.empty() || key2Node.empty() || key3Node.empty()) {
        cerr << "  FAIL: GetNode should return a node for any key" << endl;
        passed = false;
    } else {
        cout << "  PASS: Keys distributed - user:123->" << key1Node
             << ", user:456->" << key2Node << ", user:789->" << key3Node << endl;
    }

    // Test 3: Consistency
    cout << "\nTest 3: Same key should always return same node..." << endl;
    string consistentNode = ch.getNode("consistent-key");
    bool isConsistent = true;
    for (int i = 0; i < 10; i++) {
        if (ch.getNode("consistent-key") != consistentNode) {
            isConsistent = false;
            break;
        }
    }
    if (!isConsistent) {
        cerr << "  FAIL: Same key should map to same node" << endl;
        passed = false;
    } else {
        cout << "  PASS: Consistent hashing is consistent" << endl;
    }

    // Test 4: Node removal
    cout << "\nTest 4: Removing a node..." << endl;
    ch.removeNode("server-2");
    if (ch.size() != 2) {
        cerr << "  FAIL: Expected 2 nodes after removal, got " << ch.size() << endl;
        passed = false;
    } else {
        cout << "  PASS: Node removed successfully" << endl;
    }

    // Test 5: Keys should still map after node removal
    cout << "\nTest 5: Keys should still map after node removal..." << endl;
    string keyAfterRemoval = ch.getNode("any-key");
    if (keyAfterRemoval.empty()) {
        cerr << "  FAIL: GetNode should still work after node removal" << endl;
        passed = false;
    } else if (keyAfterRemoval == "server-2") {
        cerr << "  FAIL: Key should not map to removed node" << endl;
        passed = false;
    } else {
        cout << "  PASS: Key mapped to " << keyAfterRemoval << " after server-2 removal" << endl;
    }

    // Test 6: Empty ring
    cout << "\nTest 6: Empty ring behavior..." << endl;
    ConsistentHash emptyRing(3);
    string emptyResult = emptyRing.getNode("some-key");
    if (!emptyResult.empty()) {
        cerr << "  FAIL: Empty ring should return empty string" << endl;
        passed = false;
    } else {
        cout << "  PASS: Empty ring returns empty string" << endl;
    }

    // ============================================================================
    // Test GCounter (CRDT)
    // ============================================================================

    cout << "\n" << string(50, '=') << endl;
    cout << "Testing GCounter (CRDT)..." << endl;

    // Test 7: Basic increment
    cout << "\nTest 7: Basic increment operation..." << endl;
    GCounter gc1("node-1");
    gc1.increment();
    gc1.increment();
    gc1.increment();

    if (gc1.value() != 3) {
        cerr << "  FAIL: Expected value 3 after 3 increments, got " << gc1.value() << endl;
        passed = false;
    } else {
        cout << "  PASS: GCounter increments correctly" << endl;
    }

    // Test 8: IncrementBy
    cout << "\nTest 8: IncrementBy operation..." << endl;
    gc1.incrementBy(5);
    if (gc1.value() != 8) {
        cerr << "  FAIL: Expected value 8 after IncrementBy(5), got " << gc1.value() << endl;
        passed = false;
    } else {
        cout << "  PASS: IncrementBy works correctly" << endl;
    }

    // Test 9: Merge operation
    cout << "\nTest 9: Merging two GCounters..." << endl;
    GCounter gc2("node-2");
    gc2.increment();
    gc2.increment();

    gc1.merge(gc2);

    // gc1 should now have counts from both nodes: node-1=8, node-2=2 -> total 10
    if (gc1.value() != 10) {
        cerr << "  FAIL: Expected merged value 10, got " << gc1.value() << endl;
        passed = false;
    } else {
        cout << "  PASS: GCounter merge works correctly" << endl;
    }

    // Test 10: Merge idempotence
    cout << "\nTest 10: Merge idempotence (merging same counter twice)..." << endl;
    gc1.merge(gc2);
    if (gc1.value() != 10) {
        cerr << "  FAIL: Expected value still 10 after re-merge, got " << gc1.value() << endl;
        passed = false;
    } else {
        cout << "  PASS: Merge is idempotent" << endl;
    }

    // Test 11: Merge commutativity
    cout << "\nTest 11: Merge commutativity..." << endl;
    GCounter gcA("A");
    gcA.incrementBy(5);

    GCounter gcB("B");
    gcB.incrementBy(3);

    GCounter gcC("C");
    gcC.incrementBy(7);

    // Merge A->B->C
    GCounter gc1Copy("copy1");
    gc1Copy.merge(gcA);
    gc1Copy.merge(gcB);
    gc1Copy.merge(gcC);

    // Merge C->A->B
    GCounter gc2Copy("copy2");
    gc2Copy.merge(gcC);
    gc2Copy.merge(gcA);
    gc2Copy.merge(gcB);

    if (gc1Copy.value() != gc2Copy.value()) {
        cerr << "  FAIL: Different merge orders should give same result: "
             << gc1Copy.value() << " vs " << gc2Copy.value() << endl;
        passed = false;
    } else {
        cout << "  PASS: Merge is commutative (both give " << gc1Copy.value() << ")" << endl;
    }

    // ============================================================================
    // Test PNCounter
    // ============================================================================

    cout << "\n" << string(50, '=') << endl;
    cout << "Testing PNCounter (CRDT)..." << endl;

    // Test 12: Basic increment/decrement
    cout << "\nTest 12: Increment and decrement operations..." << endl;
    PNCounter pn1("node-1");
    pn1.increment();
    pn1.increment();
    pn1.increment();
    pn1.decrement();

    if (pn1.value() != 2) {
        cerr << "  FAIL: Expected value 2 (3 inc - 1 dec), got " << pn1.value() << endl;
        passed = false;
    } else {
        cout << "  PASS: PNCounter increment/decrement works correctly" << endl;
    }

    // Test 13: PNCounter merge
    cout << "\nTest 13: Merging two PNCounters..." << endl;
    PNCounter pn2("node-2");
    pn2.increment();
    pn2.decrement();
    pn2.decrement();

    pn1.merge(pn2);

    // pn1: node-1 has +3-1=2, node-2 has +1-2=-1, total should be 1
    if (pn1.value() != 1) {
        cerr << "  FAIL: Expected merged value 1, got " << pn1.value() << endl;
        passed = false;
    } else {
        cout << "  PASS: PNCounter merge works correctly" << endl;
    }

    // ============================================================================
    // Summary
    // ============================================================================

    cout << "\n" << string(50, '=') << endl;
    if (passed) {
        cout << "ALL TESTS PASSED! You have mastered the Core of the Architect!" << endl;
    } else {
        cout << "Some tests FAILED. Review your implementation." << endl;
        return 1;
    }

    return 0;
}

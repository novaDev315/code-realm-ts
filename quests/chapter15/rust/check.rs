// Use SOLUTIONS module for testing the working implementation
// To test the stub, change this to: mod distributed;
#[path = "SOLUTIONS.rs"]
mod SOLUTIONS;

use SOLUTIONS::{ConsistentHash, GCounter, PNCounter};

fn run_check() -> bool {
    let mut passed = true;

    // ============================================================================
    // Test ConsistentHash
    // ============================================================================

    println!("Testing ConsistentHash...");

    // Test 1: Basic node operations
    println!("\nTest 1: Adding nodes to consistent hash ring...");
    let mut ch = ConsistentHash::new(3); // 3 virtual nodes per physical node

    ch.add_node("server-1");
    ch.add_node("server-2");
    ch.add_node("server-3");

    if ch.size() != 3 {
        eprintln!("  FAIL: Expected 3 nodes, got {}", ch.size());
        passed = false;
    } else {
        println!("  PASS: Added 3 nodes successfully");
    }

    // Test 2: Key distribution
    println!("\nTest 2: Key distribution across nodes...");
    let key1_node = ch.get_node("user:123");
    let key2_node = ch.get_node("user:456");
    let key3_node = ch.get_node("user:789");

    if key1_node.is_none() || key2_node.is_none() || key3_node.is_none() {
        eprintln!("  FAIL: GetNode should return a node for any key");
        passed = false;
    } else {
        println!(
            "  PASS: Keys distributed - user:123->{}, user:456->{}, user:789->{}",
            key1_node.unwrap(),
            key2_node.unwrap(),
            key3_node.unwrap()
        );
    }

    // Test 3: Consistency
    println!("\nTest 3: Same key should always return same node...");
    let consistent_node = ch.get_node("consistent-key");
    let mut is_consistent = true;
    for _ in 0..10 {
        if ch.get_node("consistent-key") != consistent_node {
            is_consistent = false;
            break;
        }
    }
    if !is_consistent {
        eprintln!("  FAIL: Same key should map to same node");
        passed = false;
    } else {
        println!("  PASS: Consistent hashing is consistent");
    }

    // Test 4: Node removal
    println!("\nTest 4: Removing a node...");
    ch.remove_node("server-2");
    if ch.size() != 2 {
        eprintln!("  FAIL: Expected 2 nodes after removal, got {}", ch.size());
        passed = false;
    } else {
        println!("  PASS: Node removed successfully");
    }

    // Test 5: Keys should still map after node removal
    println!("\nTest 5: Keys should still map after node removal...");
    let key_after_removal = ch.get_node("any-key");
    if key_after_removal.is_none() {
        eprintln!("  FAIL: GetNode should still work after node removal");
        passed = false;
    } else if key_after_removal.unwrap() == "server-2" {
        eprintln!("  FAIL: Key should not map to removed node");
        passed = false;
    } else {
        println!(
            "  PASS: Key mapped to {} after server-2 removal",
            key_after_removal.unwrap()
        );
    }

    // Test 6: Empty ring
    println!("\nTest 6: Empty ring behavior...");
    let empty_ring = ConsistentHash::new(3);
    let empty_result = empty_ring.get_node("some-key");
    if empty_result.is_some() {
        eprintln!("  FAIL: Empty ring should return None");
        passed = false;
    } else {
        println!("  PASS: Empty ring returns None");
    }

    // ============================================================================
    // Test GCounter (CRDT)
    // ============================================================================

    println!("\n============================================");
    println!("Testing GCounter (CRDT)...");

    // Test 7: Basic increment
    println!("\nTest 7: Basic increment operation...");
    let mut gc1 = GCounter::new("node-1");
    gc1.increment();
    gc1.increment();
    gc1.increment();

    if gc1.value() != 3 {
        eprintln!(
            "  FAIL: Expected value 3 after 3 increments, got {}",
            gc1.value()
        );
        passed = false;
    } else {
        println!("  PASS: GCounter increments correctly");
    }

    // Test 8: IncrementBy
    println!("\nTest 8: IncrementBy operation...");
    gc1.increment_by(5);
    if gc1.value() != 8 {
        eprintln!(
            "  FAIL: Expected value 8 after increment_by(5), got {}",
            gc1.value()
        );
        passed = false;
    } else {
        println!("  PASS: increment_by works correctly");
    }

    // Test 9: Merge operation
    println!("\nTest 9: Merging two GCounters...");
    let mut gc2 = GCounter::new("node-2");
    gc2.increment();
    gc2.increment();

    gc1.merge(&gc2);

    // gc1 should now have counts from both nodes: node-1=8, node-2=2 -> total 10
    if gc1.value() != 10 {
        eprintln!("  FAIL: Expected merged value 10, got {}", gc1.value());
        passed = false;
    } else {
        println!("  PASS: GCounter merge works correctly");
    }

    // Test 10: Merge idempotence
    println!("\nTest 10: Merge idempotence (merging same counter twice)...");
    gc1.merge(&gc2);
    if gc1.value() != 10 {
        eprintln!(
            "  FAIL: Expected value still 10 after re-merge, got {}",
            gc1.value()
        );
        passed = false;
    } else {
        println!("  PASS: Merge is idempotent");
    }

    // Test 11: Merge commutativity
    println!("\nTest 11: Merge commutativity...");
    let mut gc_a = GCounter::new("A");
    gc_a.increment_by(5);

    let mut gc_b = GCounter::new("B");
    gc_b.increment_by(3);

    let mut gc_c = GCounter::new("C");
    gc_c.increment_by(7);

    // Merge A->B->C
    let mut gc1_copy = GCounter::new("copy1");
    gc1_copy.merge(&gc_a);
    gc1_copy.merge(&gc_b);
    gc1_copy.merge(&gc_c);

    // Merge C->A->B
    let mut gc2_copy = GCounter::new("copy2");
    gc2_copy.merge(&gc_c);
    gc2_copy.merge(&gc_a);
    gc2_copy.merge(&gc_b);

    if gc1_copy.value() != gc2_copy.value() {
        eprintln!(
            "  FAIL: Different merge orders should give same result: {} vs {}",
            gc1_copy.value(),
            gc2_copy.value()
        );
        passed = false;
    } else {
        println!(
            "  PASS: Merge is commutative (both give {})",
            gc1_copy.value()
        );
    }

    // ============================================================================
    // Test PNCounter
    // ============================================================================

    println!("\n============================================");
    println!("Testing PNCounter (CRDT)...");

    // Test 12: Basic increment/decrement
    println!("\nTest 12: Increment and decrement operations...");
    let mut pn1 = PNCounter::new("node-1");
    pn1.increment();
    pn1.increment();
    pn1.increment();
    pn1.decrement();

    if pn1.value() != 2 {
        eprintln!(
            "  FAIL: Expected value 2 (3 inc - 1 dec), got {}",
            pn1.value()
        );
        passed = false;
    } else {
        println!("  PASS: PNCounter increment/decrement works correctly");
    }

    // Test 13: PNCounter merge
    println!("\nTest 13: Merging two PNCounters...");
    let mut pn2 = PNCounter::new("node-2");
    pn2.increment();
    pn2.decrement();
    pn2.decrement();

    pn1.merge(&pn2);

    // pn1: node-1 has +3-1=2, node-2 has +1-2=-1, total should be 1
    if pn1.value() != 1 {
        eprintln!("  FAIL: Expected merged value 1, got {}", pn1.value());
        passed = false;
    } else {
        println!("  PASS: PNCounter merge works correctly");
    }

    // ============================================================================
    // Summary
    // ============================================================================

    println!("\n============================================");
    if passed {
        println!("ALL TESTS PASSED! You have mastered the Core of the Architect!");
    } else {
        eprintln!("Some tests FAILED. Review your implementation.");
        std::process::exit(1);
    }

    passed
}

fn main() {
    run_check();
}

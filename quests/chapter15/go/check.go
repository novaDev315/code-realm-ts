package main

import (
	"fmt"
	"os"
)

func runCheck() bool {
	passed := true

	// ============================================================================
	// Test ConsistentHash
	// ============================================================================

	fmt.Println("Testing ConsistentHash...")

	// Test 1: Basic node operations
	fmt.Println("\nTest 1: Adding nodes to consistent hash ring...")
	ch := NewConsistentHash(3) // 3 virtual nodes per physical node

	ch.AddNode("server-1")
	ch.AddNode("server-2")
	ch.AddNode("server-3")

	if ch.Size() != 3 {
		fmt.Printf("  FAIL: Expected 3 nodes, got %d\n", ch.Size())
		passed = false
	} else {
		fmt.Println("  PASS: Added 3 nodes successfully")
	}

	// Test 2: Key distribution
	fmt.Println("\nTest 2: Key distribution across nodes...")
	key1Node := ch.GetNode("user:123")
	key2Node := ch.GetNode("user:456")
	key3Node := ch.GetNode("user:789")

	if key1Node == "" || key2Node == "" || key3Node == "" {
		fmt.Println("  FAIL: GetNode should return a node for any key")
		passed = false
	} else {
		fmt.Printf("  PASS: Keys distributed - user:123->%s, user:456->%s, user:789->%s\n",
			key1Node, key2Node, key3Node)
	}

	// Test 3: Consistency
	fmt.Println("\nTest 3: Same key should always return same node...")
	consistentNode := ch.GetNode("consistent-key")
	isConsistent := true
	for i := 0; i < 10; i++ {
		if ch.GetNode("consistent-key") != consistentNode {
			isConsistent = false
			break
		}
	}
	if !isConsistent {
		fmt.Println("  FAIL: Same key should map to same node")
		passed = false
	} else {
		fmt.Println("  PASS: Consistent hashing is consistent")
	}

	// Test 4: Node removal
	fmt.Println("\nTest 4: Removing a node...")
	ch.RemoveNode("server-2")
	if ch.Size() != 2 {
		fmt.Printf("  FAIL: Expected 2 nodes after removal, got %d\n", ch.Size())
		passed = false
	} else {
		fmt.Println("  PASS: Node removed successfully")
	}

	// Test 5: Keys should still map after node removal
	fmt.Println("\nTest 5: Keys should still map after node removal...")
	keyAfterRemoval := ch.GetNode("any-key")
	if keyAfterRemoval == "" {
		fmt.Println("  FAIL: GetNode should still work after node removal")
		passed = false
	} else if keyAfterRemoval == "server-2" {
		fmt.Println("  FAIL: Key should not map to removed node")
		passed = false
	} else {
		fmt.Printf("  PASS: Key mapped to %s after server-2 removal\n", keyAfterRemoval)
	}

	// Test 6: Empty ring
	fmt.Println("\nTest 6: Empty ring behavior...")
	emptyRing := NewConsistentHash(3)
	emptyResult := emptyRing.GetNode("some-key")
	if emptyResult != "" {
		fmt.Println("  FAIL: Empty ring should return empty string")
		passed = false
	} else {
		fmt.Println("  PASS: Empty ring returns empty string")
	}

	// ============================================================================
	// Test GCounter (CRDT)
	// ============================================================================

	fmt.Println("\n" + "=" + "=========================================")
	fmt.Println("Testing GCounter (CRDT)...")

	// Test 7: Basic increment
	fmt.Println("\nTest 7: Basic increment operation...")
	gc1 := NewGCounter("node-1")
	gc1.Increment()
	gc1.Increment()
	gc1.Increment()

	if gc1.Value() != 3 {
		fmt.Printf("  FAIL: Expected value 3 after 3 increments, got %d\n", gc1.Value())
		passed = false
	} else {
		fmt.Println("  PASS: GCounter increments correctly")
	}

	// Test 8: IncrementBy
	fmt.Println("\nTest 8: IncrementBy operation...")
	gc1.IncrementBy(5)
	if gc1.Value() != 8 {
		fmt.Printf("  FAIL: Expected value 8 after IncrementBy(5), got %d\n", gc1.Value())
		passed = false
	} else {
		fmt.Println("  PASS: IncrementBy works correctly")
	}

	// Test 9: Merge operation
	fmt.Println("\nTest 9: Merging two GCounters...")
	gc2 := NewGCounter("node-2")
	gc2.Increment()
	gc2.Increment()

	gc1.Merge(gc2)

	// gc1 should now have counts from both nodes: node-1=8, node-2=2 -> total 10
	if gc1.Value() != 10 {
		fmt.Printf("  FAIL: Expected merged value 10, got %d\n", gc1.Value())
		passed = false
	} else {
		fmt.Println("  PASS: GCounter merge works correctly")
	}

	// Test 10: Merge idempotence
	fmt.Println("\nTest 10: Merge idempotence (merging same counter twice)...")
	gc1.Merge(gc2)
	if gc1.Value() != 10 {
		fmt.Printf("  FAIL: Expected value still 10 after re-merge, got %d\n", gc1.Value())
		passed = false
	} else {
		fmt.Println("  PASS: Merge is idempotent")
	}

	// Test 11: Merge commutativity
	fmt.Println("\nTest 11: Merge commutativity...")
	gcA := NewGCounter("A")
	gcA.IncrementBy(5)

	gcB := NewGCounter("B")
	gcB.IncrementBy(3)

	gcC := NewGCounter("C")
	gcC.IncrementBy(7)

	// Merge A->B->C
	gc1Copy := NewGCounter("copy1")
	gc1Copy.Merge(gcA)
	gc1Copy.Merge(gcB)
	gc1Copy.Merge(gcC)

	// Merge C->A->B
	gc2Copy := NewGCounter("copy2")
	gc2Copy.Merge(gcC)
	gc2Copy.Merge(gcA)
	gc2Copy.Merge(gcB)

	if gc1Copy.Value() != gc2Copy.Value() {
		fmt.Printf("  FAIL: Different merge orders should give same result: %d vs %d\n",
			gc1Copy.Value(), gc2Copy.Value())
		passed = false
	} else {
		fmt.Printf("  PASS: Merge is commutative (both give %d)\n", gc1Copy.Value())
	}

	// ============================================================================
	// Test PNCounter
	// ============================================================================

	fmt.Println("\n" + "=" + "=========================================")
	fmt.Println("Testing PNCounter (CRDT)...")

	// Test 12: Basic increment/decrement
	fmt.Println("\nTest 12: Increment and decrement operations...")
	pn1 := NewPNCounter("node-1")
	pn1.Increment()
	pn1.Increment()
	pn1.Increment()
	pn1.Decrement()

	if pn1.Value() != 2 {
		fmt.Printf("  FAIL: Expected value 2 (3 inc - 1 dec), got %d\n", pn1.Value())
		passed = false
	} else {
		fmt.Println("  PASS: PNCounter increment/decrement works correctly")
	}

	// Test 13: PNCounter merge
	fmt.Println("\nTest 13: Merging two PNCounters...")
	pn2 := NewPNCounter("node-2")
	pn2.Increment()
	pn2.Decrement()
	pn2.Decrement()

	pn1.Merge(pn2)

	// pn1: node-1 has +3-1=2, node-2 has +1-2=-1, total should be 1
	if pn1.Value() != 1 {
		fmt.Printf("  FAIL: Expected merged value 1, got %d\n", pn1.Value())
		passed = false
	} else {
		fmt.Println("  PASS: PNCounter merge works correctly")
	}

	// ============================================================================
	// Summary
	// ============================================================================

	fmt.Println("\n" + "=" + "=========================================")
	if passed {
		fmt.Println("ALL TESTS PASSED! You have mastered the Core of the Architect!")
	} else {
		fmt.Println("Some tests FAILED. Review your implementation.")
		os.Exit(1)
	}

	return passed
}

func main() {
	runCheck()
}

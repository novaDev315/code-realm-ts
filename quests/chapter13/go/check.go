package main

import (
	"fmt"
	"math"
	"os"
)

func runCheck() bool {
	passed := true

	// Test LRUCache basic operations
	fmt.Println("Testing LRUCache basic operations...")
	cache := NewLRUCache(2)

	// Test put and get
	cache.Put(1, 10)
	cache.Put(2, 20)

	getA := cache.Get(1)
	if getA != 10 {
		fmt.Printf("❌ Expected cache.Get(1) = 10, got %d\n", getA)
		passed = false
	} else {
		fmt.Println("✓ Put and get work correctly")
	}

	// Test capacity and eviction
	fmt.Println("\nTesting eviction policy...")
	cache.Put(3, 30) // Should evict key 2 since key 1 was just accessed

	getB := cache.Get(2)
	if getB != -1 {
		fmt.Printf("❌ Expected cache.Get(2) = -1 (evicted), got %d\n", getB)
		passed = false
	} else {
		fmt.Println("✓ LRU eviction policy works correctly")
	}

	// Test cache size
	fmt.Println("\nTesting cache size...")
	if cache.Size() != 2 {
		fmt.Printf("❌ Expected cache size = 2, got %d\n", cache.Size())
		passed = false
	} else {
		fmt.Println("✓ Cache size correct")
	}

	// Test has method
	fmt.Println("\nTesting has method...")
	if !cache.Has(1) {
		fmt.Println("❌ Expected cache.Has(1) = true")
		passed = false
	} else if cache.Has(2) {
		fmt.Println("❌ Expected cache.Has(2) = false")
		passed = false
	} else {
		fmt.Println("✓ Has method works correctly")
	}

	// Test updating existing key doesn't evict
	fmt.Println("\nTesting update of existing key...")
	cache.Put(1, 100)
	if cache.Get(1) != 100 {
		fmt.Println("❌ Expected cache.Get(1) = 100 after update")
		passed = false
	} else {
		fmt.Println("✓ Update of existing key works")
	}

	// Test cache hit rate calculation
	fmt.Println("\nTesting cache hit rate calculation...")
	operations := []map[string]interface{}{
		{"op": "put", "key": 10, "value": 1},
		{"op": "put", "key": 20, "value": 2},
		{"op": "get", "key": 10}, // hit
		{"op": "get", "key": 20}, // hit
		{"op": "get", "key": 30}, // miss
		{"op": "get", "key": 10}, // hit
	}

	hitRate := CacheHitRate(operations)
	expectedHitRate := 75.0 // 3 hits out of 4 gets = 75%

	if math.Abs(hitRate-expectedHitRate) > 0.01 {
		fmt.Printf("❌ Expected hit rate ≈ %.1f%%, got %.1f%%\n", expectedHitRate, hitRate)
		passed = false
	} else {
		fmt.Printf("✓ Cache hit rate calculation correct (%.1f%%)\n", hitRate)
	}

	// Test with different capacity
	fmt.Println("\nTesting with capacity = 3...")
	cache3 := NewLRUCache(3)
	cache3.Put(1, 100)
	cache3.Put(2, 200)
	cache3.Put(3, 300)
	cache3.Get(1) // Access 1 to make it recently used
	cache3.Put(4, 400) // Should evict 2

	if cache3.Has(2) {
		fmt.Println("❌ Expected 2 to be evicted")
		passed = false
	} else if !cache3.Has(1) {
		fmt.Println("❌ Expected 1 to still be in cache")
		passed = false
	} else {
		fmt.Println("✓ Larger cache capacity works correctly")
	}

	if passed {
		fmt.Println("\n✅ All tests passed!")
	} else {
		fmt.Println("\n❌ Some tests failed.")
		os.Exit(1)
	}

	return passed
}

func main() {
	runCheck()
}

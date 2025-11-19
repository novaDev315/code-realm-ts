package main

import (
	"fmt"
	"os"
	"sort"
)

// Helper function to compare two slices of integers
func sliceEqual(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

// Helper function to check if slice contains all elements from target (in any order)
func sliceContainsAll(slice, target []int) bool {
	if len(slice) != len(target) {
		return false
	}
	sortedSlice := make([]int, len(slice))
	copy(sortedSlice, slice)
	sort.Ints(sortedSlice)

	sortedTarget := make([]int, len(target))
	copy(sortedTarget, target)
	sort.Ints(sortedTarget)

	return sliceEqual(sortedSlice, sortedTarget)
}

// Test the graph functions
func testGraphs() bool {
	passed := 0
	total := 0

	// Test 1: DFS on a simple graph
	fmt.Println("\n--- Testing DFS ---")
	graph1 := Graph{
		0: {1, 2},
		1: {0, 3},
		2: {0, 4},
		3: {1},
		4: {2},
	}
	total++
	dfsResult := DFS(graph1, 0)
	expectedDFS := []int{0, 1, 3, 2, 4}
	if sliceEqual(dfsResult, expectedDFS) {
		fmt.Printf("✓ DFS result: %v\n", dfsResult)
		passed++
	} else {
		fmt.Printf("✗ DFS result: %v, expected: %v\n", dfsResult, expectedDFS)
	}

	// Test 2: BFS on the same graph
	fmt.Println("\n--- Testing BFS ---")
	total++
	bfsResult := BFS(graph1, 0)
	expectedBFS := []int{0, 1, 2, 3, 4}
	if sliceEqual(bfsResult, expectedBFS) {
		fmt.Printf("✓ BFS result: %v\n", bfsResult)
		passed++
	} else {
		fmt.Printf("✗ BFS result: %v, expected: %v\n", bfsResult, expectedBFS)
	}

	// Test 3: Shortest Path - direct connection
	fmt.Println("\n--- Testing Shortest Path ---")
	total++
	path := ShortestPath(graph1, 0, 3)
	expectedPath := []int{0, 1, 3}
	if sliceEqual(path, expectedPath) {
		fmt.Printf("✓ Shortest path from 0 to 3: %v\n", path)
		passed++
	} else {
		fmt.Printf("✗ Shortest path from 0 to 3: %v, expected: %v\n", path, expectedPath)
	}

	// Test 4: Shortest Path - indirect connection
	total++
	path2 := ShortestPath(graph1, 0, 4)
	expectedPath2 := []int{0, 2, 4}
	if sliceEqual(path2, expectedPath2) {
		fmt.Printf("✓ Shortest path from 0 to 4: %v\n", path2)
		passed++
	} else {
		fmt.Printf("✗ Shortest path from 0 to 4: %v, expected: %v\n", path2, expectedPath2)
	}

	// Test 5: Shortest Path - same node
	total++
	path3 := ShortestPath(graph1, 0, 0)
	expectedPath3 := []int{0}
	if sliceEqual(path3, expectedPath3) {
		fmt.Printf("✓ Shortest path from 0 to 0: %v\n", path3)
		passed++
	} else {
		fmt.Printf("✗ Shortest path from 0 to 0: %v, expected: %v\n", path3, expectedPath3)
	}

	// Test 6: Graph without cycle
	fmt.Println("\n--- Testing Cycle Detection ---")
	acyclicGraph := Graph{
		0: {1, 2},
		1: {3},
		2: {3},
		3: {},
	}
	total++
	hasCycle := HasCycle(acyclicGraph)
	if !hasCycle {
		fmt.Printf("✓ Acyclic graph correctly identified: no cycle\n")
		passed++
	} else {
		fmt.Printf("✗ Acyclic graph incorrectly identified as having a cycle\n")
	}

	// Test 7: Graph with cycle
	total++
	cyclicGraph := Graph{
		0: {1},
		1: {2},
		2: {0},
	}
	hasCycle = HasCycle(cyclicGraph)
	if hasCycle {
		fmt.Printf("✓ Cyclic graph correctly identified: has cycle\n")
		passed++
	} else {
		fmt.Printf("✗ Cyclic graph incorrectly identified as acyclic\n")
	}

	// Test 8: Single node graph
	fmt.Println("\n--- Testing Single Node Graph ---")
	total++
	singleGraph := Graph{
		0: {},
	}
	dfsResult = DFS(singleGraph, 0)
	expectedSingle := []int{0}
	if sliceEqual(dfsResult, expectedSingle) {
		fmt.Printf("✓ Single node DFS: %v\n", dfsResult)
		passed++
	} else {
		fmt.Printf("✗ Single node DFS: %v, expected: %v\n", dfsResult, expectedSingle)
	}

	// Test 9: Disconnected path
	total++
	graph2 := Graph{
		0: {1},
		1: {},
		2: {3},
		3: {},
	}
	path4 := ShortestPath(graph2, 0, 2)
	if len(path4) == 0 {
		fmt.Printf("✓ Disconnected nodes correctly return empty path\n")
		passed++
	} else {
		fmt.Printf("✗ Disconnected nodes should return empty path, got: %v\n", path4)
	}

	// Test 10: Self-loop cycle detection
	total++
	selfLoopGraph := Graph{
		0: {0},
	}
	hasCycle = HasCycle(selfLoopGraph)
	if hasCycle {
		fmt.Printf("✓ Self-loop correctly identified as cycle\n")
		passed++
	} else {
		fmt.Printf("✗ Self-loop not identified as cycle\n")
	}

	fmt.Printf("\nPassed: %d/%d tests\n", passed, total)
	return passed == total
}

func main() {
	if !testGraphs() {
		os.Exit(1)
	}
}

package main

import (
	"fmt"
	"os"
)

func runCheck() bool {
	passed := true

	// Test enqueue and size
	fmt.Println("Testing enqueue and size...")
	queue1 := NewMessageQueue()

	if queue1.Size() != 0 {
		fmt.Printf("size() on empty queue should be 0, got %d\n", queue1.Size())
		passed = false
	}

	queue1.Enqueue("message1")
	if queue1.Size() != 1 {
		fmt.Printf("size() after enqueue should be 1, got %d\n", queue1.Size())
		passed = false
	}

	queue1.Enqueue("message2")
	queue1.Enqueue("message3")
	if queue1.Size() != 3 {
		fmt.Printf("size() after 3 enqueues should be 3, got %d\n", queue1.Size())
		passed = false
	} else {
		fmt.Println("Enqueue and size work correctly")
	}

	// Test isEmpty
	fmt.Println("\nTesting isEmpty...")
	queue2 := NewMessageQueue()
	if !queue2.IsEmpty() {
		fmt.Println("isEmpty() on empty queue should be true")
		passed = false
	}

	queue2.Enqueue("test")
	if queue2.IsEmpty() {
		fmt.Println("isEmpty() on non-empty queue should be false")
		passed = false
	} else {
		fmt.Println("IsEmpty works correctly")
	}

	// Test peek
	fmt.Println("\nTesting peek...")
	queue3 := NewMessageQueue()
	if msg, ok := queue3.Peek(); ok {
		fmt.Printf("peek() on empty queue should return false, got %v\n", msg)
		passed = false
	}

	queue3.Enqueue("first")
	queue3.Enqueue("second")
	if msg, ok := queue3.Peek(); !ok || msg != "first" {
		fmt.Printf("peek() should return 'first', got '%s'\n", msg)
		passed = false
	}
	if queue3.Size() != 2 {
		fmt.Println("peek() should not remove, size should still be 2")
		passed = false
	} else {
		fmt.Println("Peek works correctly")
	}

	// Test dequeue FIFO order
	fmt.Println("\nTesting dequeue (FIFO)...")
	queue4 := NewMessageQueue()
	if _, ok := queue4.Dequeue(); ok {
		fmt.Println("dequeue() on empty queue should return false")
		passed = false
	}

	queue4.Enqueue("alpha")
	queue4.Enqueue("beta")
	queue4.Enqueue("gamma")

	first, _ := queue4.Dequeue()
	if first != "alpha" {
		fmt.Printf("First dequeue should return 'alpha', got '%s'\n", first)
		passed = false
	}

	second, _ := queue4.Dequeue()
	if second != "beta" {
		fmt.Printf("Second dequeue should return 'beta', got '%s'\n", second)
		passed = false
	}

	third, _ := queue4.Dequeue()
	if third != "gamma" {
		fmt.Printf("Third dequeue should return 'gamma', got '%s'\n", third)
		passed = false
	}

	if _, ok := queue4.Dequeue(); ok {
		fmt.Println("Dequeue from empty queue should return false")
		passed = false
	} else {
		fmt.Println("Dequeue FIFO order works correctly")
	}

	// Test processBatch
	fmt.Println("\nTesting processBatch...")
	queue5 := NewMessageQueue()
	queue5.Enqueue("1")
	queue5.Enqueue("2")
	queue5.Enqueue("3")
	queue5.Enqueue("4")
	queue5.Enqueue("5")

	batch1 := ProcessBatch(queue5, 2)
	if len(batch1) != 2 {
		fmt.Printf("ProcessBatch(queue, 2) should return 2 items, got %d\n", len(batch1))
		passed = false
	}
	if batch1[0] != "1" || batch1[1] != "2" {
		fmt.Printf("ProcessBatch should return ['1', '2'], got %v\n", batch1)
		passed = false
	}
	if queue5.Size() != 3 {
		fmt.Printf("Queue should have 3 items left, got %d\n", queue5.Size())
		passed = false
	}

	batch2 := ProcessBatch(queue5, 5)
	if len(batch2) != 3 {
		fmt.Printf("ProcessBatch(queue, 5) should return 3 items, got %d\n", len(batch2))
		passed = false
	}
	if batch2[0] != "3" || batch2[1] != "4" || batch2[2] != "5" {
		fmt.Printf("ProcessBatch should return ['3', '4', '5'], got %v\n", batch2)
		passed = false
	} else {
		fmt.Println("ProcessBatch works correctly")
	}

	// Test edge cases
	fmt.Println("\nTesting edge cases...")
	queue6 := NewMessageQueue()
	emptyBatch := ProcessBatch(queue6, 10)
	if len(emptyBatch) != 0 {
		fmt.Printf("ProcessBatch on empty queue should return empty slice, got %d items\n", len(emptyBatch))
		passed = false
	} else {
		fmt.Println("Edge cases handled correctly")
	}

	// Test interleaved operations
	fmt.Println("\nTesting interleaved operations...")
	queue7 := NewMessageQueue()
	queue7.Enqueue("a")
	queue7.Enqueue("b")
	msg, _ := queue7.Dequeue() // "a"
	queue7.Enqueue("c")
	msg, _ = queue7.Dequeue() // "b"
	if msg != "b" {
		fmt.Printf("Expected 'b' after interleaved ops, got '%s'\n", msg)
		passed = false
	}
	msg, _ = queue7.Dequeue() // "c"
	if msg != "c" {
		fmt.Printf("Expected 'c' after interleaved ops, got '%s'\n", msg)
		passed = false
	} else {
		fmt.Println("Interleaved operations work correctly")
	}

	if passed {
		fmt.Println("\nAll tests passed!")
	} else {
		fmt.Println("\nSome tests failed.")
		os.Exit(1)
	}

	return passed
}

func main() {
	runCheck()
}

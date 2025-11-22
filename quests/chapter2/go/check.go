package main

import (
	"fmt"
	"math/rand"
	"os"
)

type TestCase struct {
	input  []int
	expect []int
	name   string
}

func main() {
	passed := true

	testCases := []TestCase{
		{input: []int{5, 2, 8, 1, 9}, expect: []int{1, 2, 5, 8, 9}, name: "basic array"},
		{input: []int{3, 3, 1, 2}, expect: []int{1, 2, 3, 3}, name: "duplicates"},
		{input: []int{1}, expect: []int{1}, name: "single element"},
		{input: []int{}, expect: []int{}, name: "empty array"},
		{input: []int{5, 4, 3, 2, 1}, expect: []int{1, 2, 3, 4, 5}, name: "reverse sorted"},
		{input: []int{1, 2, 3, 4, 5}, expect: []int{1, 2, 3, 4, 5}, name: "already sorted"},
	}

	fmt.Println("Testing MergeSort...")
	for _, tc := range testCases {
		input := make([]int, len(tc.input))
		copy(input, tc.input)
		result := MergeSort(input)
		if !arraysEqual(result, tc.expect) {
			fmt.Printf("❌ MergeSort(%v) [%s] expected %v but got %v\n", tc.input, tc.name, tc.expect, result)
			passed = false
		} else {
			fmt.Printf("✓ MergeSort %s\n", tc.name)
		}
	}

	fmt.Println("\nTesting QuickSort...")
	for _, tc := range testCases {
		input := make([]int, len(tc.input))
		copy(input, tc.input)
		result := QuickSort(input)
		if !arraysEqual(result, tc.expect) {
			fmt.Printf("❌ QuickSort(%v) [%s] expected %v but got %v\n", tc.input, tc.name, tc.expect, result)
			passed = false
		} else {
			fmt.Printf("✓ QuickSort %s\n", tc.name)
		}
	}

	// Test on large arrays
	fmt.Println("\nTesting performance on large array...")
	largeArray := make([]int, 1000)
	for i := range largeArray {
		largeArray[i] = rand.Intn(1000)
	}

	mergeSorted := MergeSort(append([]int{}, largeArray...))
	if !isSorted(mergeSorted) {
		fmt.Println("❌ MergeSort failed to sort large array")
		passed = false
	} else {
		fmt.Println("✓ MergeSort handles large arrays")
	}

	quickSorted := QuickSort(append([]int{}, largeArray...))
	if !isSorted(quickSorted) {
		fmt.Println("❌ QuickSort failed to sort large array")
		passed = false
	} else {
		fmt.Println("✓ QuickSort handles large arrays")
	}

	if passed {
		fmt.Println("\n✅ All tests passed!")
	} else {
		fmt.Println("\n❌ Some tests failed.")
		os.Exit(1)
	}
}

func arraysEqual(a, b []int) bool {
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

func isSorted(arr []int) bool {
	for i := 1; i < len(arr); i++ {
		if arr[i] < arr[i-1] {
			return false
		}
	}
	return true
}

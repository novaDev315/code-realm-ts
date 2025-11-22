package main

import (
	"fmt"
	"os"
	"reflect"
)

func main() {
	passed := true

	// Test maxSumSubarray
	fmt.Println("Testing MaxSumSubarray...")
	maxSumCases := []struct {
		arr    []int
		k      int
		expect int
	}{
		{[]int{2, 1, 5, 1, 3, 2}, 3, 9},
		{[]int{2, 3, 4, 1, 5}, 2, 7},
		{[]int{1, 4, 2, 10, 23, 3, 1, 0, 20}, 4, 39},
	}

	for _, tc := range maxSumCases {
		result := MaxSumSubarray(tc.arr, tc.k)
		if result != tc.expect {
			fmt.Printf("❌ MaxSumSubarray(%v, %d) expected %d but got %d\n", tc.arr, tc.k, tc.expect, result)
			passed = false
		} else {
			fmt.Printf("✓ MaxSumSubarray k=%d\n", tc.k)
		}
	}

	// Test twoSum
	fmt.Println("\nTesting TwoSum...")
	twoSumCases := []struct {
		arr       []int
		target    int
		expectSum int
	}{
		{[]int{1, 2, 3, 4, 5}, 9, 9},
		{[]int{2, 7, 11, 15}, 9, 9},
	}

	for _, tc := range twoSumCases {
		result := TwoSum(tc.arr, tc.target)
		if result == nil {
			fmt.Printf("❌ TwoSum(%v, %d) returned nil\n", tc.arr, tc.target)
			passed = false
		} else if tc.arr[result[0]]+tc.arr[result[1]] != tc.expectSum {
			fmt.Printf("❌ TwoSum(%v, %d) indices don't sum to %d\n", tc.arr, tc.target, tc.expectSum)
			passed = false
		} else {
			fmt.Printf("✓ TwoSum target=%d\n", tc.target)
		}
	}

	// Test threeSum
	fmt.Println("\nTesting ThreeSum...")
	threeSumResult := ThreeSum([]int{-1, 0, 1, 2, -1, -4}, 0)
	if len(threeSumResult) == 0 {
		fmt.Println("❌ ThreeSum should find at least one triplet")
		passed = false
	} else {
		validTriplets := true
		for _, triplet := range threeSumResult {
			if triplet[0]+triplet[1]+triplet[2] != 0 {
				validTriplets = false
				break
			}
		}
		if validTriplets {
			fmt.Printf("✓ ThreeSum found %d valid triplet(s)\n", len(threeSumResult))
		} else {
			fmt.Println("❌ ThreeSum returned invalid triplets")
			passed = false
		}
	}

	if passed {
		fmt.Println("\n✅ All tests passed!")
	} else {
		fmt.Println("\n❌ Some tests failed.")
		os.Exit(1)
	}
}

// Helper to compare slices (for debugging)
func equalSlices(a, b []int) bool {
	return reflect.DeepEqual(a, b)
}

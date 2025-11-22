package main

import (
	"fmt"
	"os"
)

func main() {
	passed := true

	// Test Knapsack 01
	fmt.Println("Testing 0/1 Knapsack...")

	// Test case 1: Basic knapsack
	weights1 := []int{2, 3, 4, 5}
	values1 := []int{3, 4, 5, 6}
	capacity1 := 5
	knapsackResult1 := Knapsack(weights1, values1, capacity1)
	// Optimal: items with weight 2 (value 3) + weight 3 (value 4) = weight 5, value 7
	if knapsackResult1 != 7 {
		fmt.Printf("❌ Knapsack test 1 failed. Expected 7, got: %d\n", knapsackResult1)
		passed = false
	} else {
		fmt.Printf("✓ Knapsack test 1 passed: max value = %d\n", knapsackResult1)
	}

	// Test case 2: Single item exceeds capacity
	weights2 := []int{10, 20, 30}
	values2 := []int{100, 200, 300}
	capacity2 := 5
	knapsackResult2 := Knapsack(weights2, values2, capacity2)
	if knapsackResult2 != 0 {
		fmt.Printf("❌ Knapsack test 2 failed. Expected 0 (no items fit), got: %d\n", knapsackResult2)
		passed = false
	} else {
		fmt.Printf("✓ Knapsack test 2 passed: no items fit in capacity (value = %d)\n", knapsackResult2)
	}

	// Test case 3: All items fit
	weights3 := []int{1, 2, 3}
	values3 := []int{10, 20, 30}
	capacity3 := 10
	knapsackResult3 := Knapsack(weights3, values3, capacity3)
	// All items fit: weight 1+2+3=6 <= 10, value 10+20+30=60
	if knapsackResult3 != 60 {
		fmt.Printf("❌ Knapsack test 3 failed. Expected 60, got: %d\n", knapsackResult3)
		passed = false
	} else {
		fmt.Printf("✓ Knapsack test 3 passed: all items fit (value = %d)\n", knapsackResult3)
	}

	// Test LCS
	fmt.Println("\nTesting Longest Common Subsequence...")

	// Test case 1: Simple LCS
	lcsResult1 := LongestCommonSubsequence("abcde", "ace")
	// "ace" is a subsequence of "abcde", length = 3
	if lcsResult1 != 3 {
		fmt.Printf("❌ LCS test 1 failed. Expected 3, got: %d\n", lcsResult1)
		passed = false
	} else {
		fmt.Printf("✓ LCS test 1 passed: LCS length = %d\n", lcsResult1)
	}

	// Test case 2: No common subsequence (except empty)
	lcsResult2 := LongestCommonSubsequence("abc", "def")
	if lcsResult2 != 0 {
		fmt.Printf("❌ LCS test 2 failed. Expected 0, got: %d\n", lcsResult2)
		passed = false
	} else {
		fmt.Printf("✓ LCS test 2 passed: no common subsequence (length = %d)\n", lcsResult2)
	}

	// Test case 3: Complete match
	lcsResult3 := LongestCommonSubsequence("hello", "hello")
	if lcsResult3 != 5 {
		fmt.Printf("❌ LCS test 3 failed. Expected 5, got: %d\n", lcsResult3)
		passed = false
	} else {
		fmt.Printf("✓ LCS test 3 passed: complete match (length = %d)\n", lcsResult3)
	}

	// Test case 4: Complex LCS
	lcsResult4 := LongestCommonSubsequence("AGGTAB", "GXTXAYB")
	// LCS is "GTAB", length = 4
	if lcsResult4 != 4 {
		fmt.Printf("❌ LCS test 4 failed. Expected 4, got: %d\n", lcsResult4)
		passed = false
	} else {
		fmt.Printf("✓ LCS test 4 passed: complex case (length = %d)\n", lcsResult4)
	}

	// Test Coin Change
	fmt.Println("\nTesting Coin Change...")

	// Test case 1: Standard coins
	coinResult1 := CoinChange([]int{1, 2, 5}, 5)
	// Can use [5] or [2,2,1], minimum is 1 coin (using 5)
	if coinResult1 != 1 {
		fmt.Printf("❌ Coin change test 1 failed. Expected 1, got: %d\n", coinResult1)
		passed = false
	} else {
		fmt.Printf("✓ Coin change test 1 passed: minimum coins for amount 5 = %d\n", coinResult1)
	}

	// Test case 2: Multiple coins needed
	coinResult2 := CoinChange([]int{2}, 3)
	// Can't make 3 with only 2-cent coins
	if coinResult2 != -1 {
		fmt.Printf("❌ Coin change test 2 failed. Expected -1, got: %d\n", coinResult2)
		passed = false
	} else {
		fmt.Printf("✓ Coin change test 2 passed: impossible amount (returned %d)\n", coinResult2)
	}

	// Test case 3: Zero amount
	coinResult3 := CoinChange([]int{1, 2, 5}, 0)
	if coinResult3 != 0 {
		fmt.Printf("❌ Coin change test 3 failed. Expected 0, got: %d\n", coinResult3)
		passed = false
	} else {
		fmt.Printf("✓ Coin change test 3 passed: zero amount needs %d coins\n", coinResult3)
	}

	// Test case 4: Complex change-making
	coinResult4 := CoinChange([]int{2, 3, 5}, 11)
	// Can use [5, 3, 3] = 3 coins
	// Can use [5, 2, 2, 2] = 4 coins
	// Minimum is 3
	if coinResult4 != 3 {
		fmt.Printf("❌ Coin change test 4 failed. Expected 3, got: %d\n", coinResult4)
		passed = false
	} else {
		fmt.Printf("✓ Coin change test 4 passed: amount 11 needs %d coins\n", coinResult4)
	}

	// Test case 5: Large amount
	coinResult5 := CoinChange([]int{1, 2, 5}, 20)
	// Minimum: 4 coins of 5
	if coinResult5 != 4 {
		fmt.Printf("❌ Coin change test 5 failed. Expected 4, got: %d\n", coinResult5)
		passed = false
	} else {
		fmt.Printf("✓ Coin change test 5 passed: amount 20 needs %d coins\n", coinResult5)
	}

	if passed {
		fmt.Println("\n✅ All tests passed!")
	} else {
		fmt.Println("\n❌ Some tests failed.")
		os.Exit(1)
	}
}

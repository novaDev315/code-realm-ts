package main

import (
	"fmt"
	"os"
	"sort"
	"strings"
)

// Test the recursion functions
func testRecursion() bool {
	passed := 0
	total := 0

	// Test Fibonacci
	testCases := []struct {
		input    int
		expected int
	}{
		{0, 0},
		{1, 1},
		{5, 5},
		{6, 8},
		{10, 55},
	}

	for _, tc := range testCases {
		total++
		result := Fibonacci(tc.input)
		if result == tc.expected {
			fmt.Printf("✓ Fibonacci(%d) = %d\n", tc.input, result)
			passed++
		} else {
			fmt.Printf("✗ Fibonacci(%d) = %d, expected %d\n", tc.input, result, tc.expected)
		}
	}

	// Test Factorial
	factorialCases := []struct {
		input    int
		expected int
	}{
		{0, 1},
		{1, 1},
		{5, 120},
		{6, 720},
		{7, 5040},
	}

	for _, tc := range factorialCases {
		total++
		result := Factorial(tc.input)
		if result == tc.expected {
			fmt.Printf("✓ Factorial(%d) = %d\n", tc.input, result)
			passed++
		} else {
			fmt.Printf("✗ Factorial(%d) = %d, expected %d\n", tc.input, result, tc.expected)
		}
	}

	// Test StringPermutations
	permutationCases := []struct {
		input    string
		expected int // count of unique permutations
	}{
		{"A", 1},
		{"AB", 2},
		{"ABC", 6},
	}

	for _, tc := range permutationCases {
		total++
		result := StringPermutations(tc.input)
		if len(result) == tc.expected {
			fmt.Printf("✓ StringPermutations(\"%s\") generated %d permutations\n", tc.input, len(result))
			passed++
		} else {
			fmt.Printf("✗ StringPermutations(\"%s\") generated %d permutations, expected %d\n", tc.input, len(result), tc.expected)
		}

		// Verify all permutations contain the same characters
		for _, perm := range result {
			originalChars := strings.Split(tc.input, "")
			sort.Strings(originalChars)
			permChars := strings.Split(perm, "")
			sort.Strings(permChars)
			if strings.Join(originalChars, "") != strings.Join(permChars, "") {
				fmt.Printf("✗ StringPermutations(\"%s\") contains invalid permutation: %s\n", tc.input, perm)
			}
		}
	}

	fmt.Printf("\nPassed: %d/%d tests\n", passed, total)
	return passed == total
}

func main() {
	if !testRecursion() {
		os.Exit(1)
	}
}

package main

// FibonacciSolution calculates the nth Fibonacci number
func FibonacciSolution(n int) int {
	if n <= 1 {
		return n
	}
	return FibonacciSolution(n-1) + FibonacciSolution(n-2)
}

// FactorialSolution calculates n!
func FactorialSolution(n int) int {
	if n <= 1 {
		return 1
	}
	return n * FactorialSolution(n-1)
}

// StringPermutationsSolution generates all unique permutations
func StringPermutationsSolution(str string) []string {
	if len(str) <= 1 {
		return []string{str}
	}

	var result []string
	seen := make(map[rune]bool)

	for i, char := range str {
		// Skip duplicate characters
		if seen[char] {
			continue
		}
		seen[char] = true

		// Get remaining string (exclude current character)
		remaining := str[:i] + str[i+1:]

		// Get permutations of remaining string
		perms := StringPermutationsSolution(remaining)

		// Prepend current character to each permutation
		for _, perm := range perms {
			result = append(result, string(char)+perm)
		}
	}

	return result
}

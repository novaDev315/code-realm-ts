package main

// Knapsack solves the 0/1 Knapsack problem using dynamic programming
func KnapsackSolution(weights []int, values []int, capacity int) int {
	n := len(weights)

	// Create DP table: dp[i][w] = max value using items 0..i-1 with capacity w
	dp := make([][]int, n+1)
	for i := range dp {
		dp[i] = make([]int, capacity+1)
	}

	// Build DP table
	for i := 1; i <= n; i++ {
		for w := 0; w <= capacity; w++ {
			// Option 1: Don't take item i-1
			dp[i][w] = dp[i-1][w]

			// Option 2: Take item i-1 (if it fits)
			if weights[i-1] <= w {
				takeValue := values[i-1] + dp[i-1][w-weights[i-1]]
				if takeValue > dp[i][w] {
					dp[i][w] = takeValue
				}
			}
		}
	}

	return dp[n][capacity]
}

// LongestCommonSubsequence finds the length of LCS using dynamic programming
func LongestCommonSubsequenceSolution(str1 string, str2 string) int {
	m := len(str1)
	n := len(str2)

	// Create DP table: dp[i][j] = length of LCS of str1[0..i-1] and str2[0..j-1]
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
	}

	// Fill DP table
	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			if str1[i-1] == str2[j-1] {
				// Characters match: add 1 to the LCS length
				dp[i][j] = dp[i-1][j-1] + 1
			} else {
				// Characters don't match: take max of two options
				if dp[i-1][j] > dp[i][j-1] {
					dp[i][j] = dp[i-1][j]
				} else {
					dp[i][j] = dp[i][j-1]
				}
			}
		}
	}

	return dp[m][n]
}

// CoinChange finds the minimum number of coins needed using dynamic programming
func CoinChangeSolution(coins []int, amount int) int {
	// DP array where dp[i] = minimum coins needed to make amount i
	dp := make([]int, amount+1)
	const infinity = int(1e9) // Use a large number instead of math.MaxInt to avoid overflow
	for i := range dp {
		dp[i] = infinity
	}
	dp[0] = 0 // 0 coins needed to make amount 0

	for i := 1; i <= amount; i++ {
		for _, coin := range coins {
			if coin <= i {
				// Try using this coin
				if dp[i-coin]+1 < dp[i] {
					dp[i] = dp[i-coin] + 1
				}
			}
		}
	}

	// Return result: -1 if impossible, otherwise the minimum coins needed
	if dp[amount] == infinity {
		return -1
	}
	return dp[amount]
}

package main

// TODO: Implement the 0/1 Knapsack problem using dynamic programming
// Hint: Create a 2D DP table dp[i][w] where:
//   - i represents items 0..i-1 considered
//   - w represents the current capacity
//   - dp[i][w] = maximum value achievable
// For each item, decide whether to include it or not
func Knapsack(weights []int, values []int, capacity int) int {
	// TODO: Implement this function
	return 0
}

// TODO: Implement Longest Common Subsequence using dynamic programming
// Hint: Create a 2D DP table dp[i][j] where:
//   - dp[i][j] = length of LCS of str1[0..i-1] and str2[0..j-1]
//   - If characters match: dp[i][j] = dp[i-1][j-1] + 1
//   - If they don't match: dp[i][j] = max(dp[i-1][j], dp[i][j-1])
func LongestCommonSubsequence(str1 string, str2 string) int {
	// TODO: Implement this function
	return 0
}

// TODO: Implement Coin Change problem using dynamic programming
// Hint: Create a 1D DP array dp[i] where:
//   - dp[i] = minimum coins needed to make amount i
//   - Initialize dp[0] = 0 and others to infinity
//   - For each amount, try using each coin
// Return -1 if the amount cannot be made
func CoinChange(coins []int, amount int) int {
	// TODO: Implement this function
	return -1
}

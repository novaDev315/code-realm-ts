// Chapter 6: Caves of Shadows - Dynamic Programming
// Your task: Implement the three core dynamic programming algorithms

#include <vector>
#include <string>
#include <stdexcept>

/**
 * 0/1 Knapsack Problem
 * Given weights and values of items, and a knapsack capacity,
 * find the maximum value that can be achieved.
 *
 * TODO: Implement using dynamic programming
 * - Create a 2D DP table: dp[i][w] = max value using items 0..i-1 with capacity w
 * - For each item, decide whether to include it or not
 * - Time complexity: O(n * capacity)
 * - Space complexity: O(n * capacity)
 *
 * @param weights Vector of item weights
 * @param values Vector of item values
 * @param capacity Maximum capacity of the knapsack
 * @return Maximum value achievable
 */
int knapsack(std::vector<int> weights, std::vector<int> values, int capacity) {
    // TODO: Implement the 0/1 knapsack algorithm
    // Hint: dp[i][w] = max(dp[i-1][w], values[i-1] + dp[i-1][w-weights[i-1]])
    throw std::runtime_error("Not implemented yet");
}

/**
 * Longest Common Subsequence
 * Find the length of the longest subsequence present in both strings.
 * A subsequence is a sequence that can be derived by deleting some or no
 * elements without changing the order of the remaining elements.
 *
 * TODO: Implement using dynamic programming
 * - Create a 2D DP table: dp[i][j] = length of LCS of str1[0..i-1] and str2[0..j-1]
 * - If characters match: dp[i][j] = dp[i-1][j-1] + 1
 * - If they don't match: dp[i][j] = max(dp[i-1][j], dp[i][j-1])
 * - Time complexity: O(m * n) where m and n are string lengths
 * - Space complexity: O(m * n)
 *
 * @param str1 First string
 * @param str2 Second string
 * @return Length of the longest common subsequence
 */
int longestCommonSubsequence(std::string str1, std::string str2) {
    // TODO: Implement the LCS algorithm
    // Hint: Build a 2D table comparing all character pairs
    throw std::runtime_error("Not implemented yet");
}

/**
 * Coin Change Problem
 * Find the minimum number of coins needed to make a given amount.
 * You have an infinite supply of each coin denomination.
 *
 * TODO: Implement using dynamic programming
 * - Create a 1D DP array: dp[i] = minimum coins needed to make amount i
 * - Initialize dp[0] = 0, all others to infinity
 * - For each amount, try using each coin and take the minimum
 * - Time complexity: O(amount * coins.size())
 * - Space complexity: O(amount)
 *
 * @param coins Vector of available coin denominations
 * @param amount Target amount
 * @return Minimum number of coins needed, or -1 if impossible
 */
int coinChange(std::vector<int> coins, int amount) {
    // TODO: Implement the coin change algorithm
    // Hint: dp[i] = min(dp[i], dp[i - coin] + 1) for each coin
    throw std::runtime_error("Not implemented yet");
}

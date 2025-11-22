// Reference solutions for Chapter 6: Caves of Shadows - Dynamic Programming

#include <vector>
#include <string>
#include <algorithm>
#include <limits>

/**
 * 0/1 Knapsack Problem - SOLUTION
 * Dynamic programming approach using a 2D table.
 */
int knapsack(std::vector<int> weights, std::vector<int> values, int capacity) {
    int n = weights.size();

    // Create DP table: dp[i][w] = max value using items 0..i-1 with capacity w
    std::vector<std::vector<int>> dp(n + 1, std::vector<int>(capacity + 1, 0));

    // Build DP table
    for (int i = 1; i <= n; i++) {
        for (int w = 0; w <= capacity; w++) {
            // Option 1: Don't take item i-1
            dp[i][w] = dp[i - 1][w];

            // Option 2: Take item i-1 (if it fits)
            if (weights[i - 1] <= w) {
                dp[i][w] = std::max(
                    dp[i][w],
                    values[i - 1] + dp[i - 1][w - weights[i - 1]]
                );
            }
        }
    }

    return dp[n][capacity];
}

/**
 * Longest Common Subsequence - SOLUTION
 * Dynamic programming approach using a 2D table.
 */
int longestCommonSubsequence(std::string str1, std::string str2) {
    int m = str1.length();
    int n = str2.length();

    // Create DP table: dp[i][j] = length of LCS of str1[0..i-1] and str2[0..j-1]
    std::vector<std::vector<int>> dp(m + 1, std::vector<int>(n + 1, 0));

    // Fill DP table
    for (int i = 1; i <= m; i++) {
        for (int j = 1; j <= n; j++) {
            if (str1[i - 1] == str2[j - 1]) {
                // Characters match: add 1 to the LCS length
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                // Characters don't match: take max of two options
                dp[i][j] = std::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    return dp[m][n];
}

/**
 * Coin Change Problem - SOLUTION
 * Dynamic programming approach using a 1D array.
 */
int coinChange(std::vector<int> coins, int amount) {
    // DP array where dp[i] = minimum coins needed to make amount i
    std::vector<int> dp(amount + 1, std::numeric_limits<int>::max());
    dp[0] = 0; // 0 coins needed to make amount 0

    for (int i = 1; i <= amount; i++) {
        for (int coin : coins) {
            if (coin <= i && dp[i - coin] != std::numeric_limits<int>::max()) {
                // Try using this coin
                dp[i] = std::min(dp[i], dp[i - coin] + 1);
            }
        }
    }

    // Return result: -1 if impossible, otherwise the minimum coins needed
    return dp[amount] == std::numeric_limits<int>::max() ? -1 : dp[amount];
}

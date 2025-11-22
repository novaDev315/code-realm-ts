// Reference solutions for Chapter 6: Caves of Shadows - Dynamic Programming

public class SOLUTIONS {
    /**
     * 0/1 Knapsack Problem - SOLUTION
     * Dynamic programming approach using a 2D table.
     */
    public static int knapsack(int[] weights, int[] values, int capacity) {
        int n = weights.length;

        // Create DP table: dp[i][w] = max value using items 0..i-1 with capacity w
        int[][] dp = new int[n + 1][capacity + 1];

        // Build DP table
        for (int i = 1; i <= n; i++) {
            for (int w = 0; w <= capacity; w++) {
                // Option 1: Don't take item i-1
                dp[i][w] = dp[i - 1][w];

                // Option 2: Take item i-1 (if it fits)
                if (weights[i - 1] <= w) {
                    dp[i][w] = Math.max(
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
    public static int longestCommonSubsequence(String str1, String str2) {
        int m = str1.length();
        int n = str2.length();

        // Create DP table: dp[i][j] = length of LCS of str1[0..i-1] and str2[0..j-1]
        int[][] dp = new int[m + 1][n + 1];

        // Fill DP table
        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                if (str1.charAt(i - 1) == str2.charAt(j - 1)) {
                    // Characters match: add 1 to the LCS length
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    // Characters don't match: take max of two options
                    dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }

        return dp[m][n];
    }

    /**
     * Coin Change Problem - SOLUTION
     * Dynamic programming approach using a 1D array.
     */
    public static int coinChange(int[] coins, int amount) {
        // DP array where dp[i] = minimum coins needed to make amount i
        int[] dp = new int[amount + 1];

        // Initialize with a large value (representing infinity)
        for (int i = 1; i <= amount; i++) {
            dp[i] = Integer.MAX_VALUE;
        }
        dp[0] = 0; // 0 coins needed to make amount 0

        for (int i = 1; i <= amount; i++) {
            for (int coin : coins) {
                if (coin <= i && dp[i - coin] != Integer.MAX_VALUE) {
                    // Try using this coin
                    dp[i] = Math.min(dp[i], dp[i - coin] + 1);
                }
            }
        }

        // Return result: -1 if impossible, otherwise the minimum coins needed
        return dp[amount] == Integer.MAX_VALUE ? -1 : dp[amount];
    }
}

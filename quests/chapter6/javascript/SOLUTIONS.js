// Reference solutions for Chapter 6: Caves of Shadows

function knapsack01(weights, values, capacity) {
  const n = weights.length;

  // Create DP table: dp[i][w] = max value using items 0..i-1 with capacity w
  const dp = Array(n + 1)
    .fill(null)
    .map(() => Array(capacity + 1).fill(0));

  // Build DP table
  for (let i = 1; i <= n; i++) {
    for (let w = 0; w <= capacity; w++) {
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

function longestCommonSubsequence(str1, str2) {
  const m = str1.length;
  const n = str2.length;

  // Create DP table: dp[i][j] = length of LCS of str1[0..i-1] and str2[0..j-1]
  const dp = Array(m + 1)
    .fill(null)
    .map(() => Array(n + 1).fill(0));

  // Fill DP table
  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      if (str1[i - 1] === str2[j - 1]) {
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

function coinChange(coins, amount) {
  // DP array where dp[i] = minimum coins needed to make amount i
  const dp = Array(amount + 1).fill(Infinity);
  dp[0] = 0; // 0 coins needed to make amount 0

  for (let i = 1; i <= amount; i++) {
    for (const coin of coins) {
      if (coin <= i) {
        // Try using this coin
        dp[i] = Math.min(dp[i], dp[i - coin] + 1);
      }
    }
  }

  // Return result: -1 if impossible, otherwise the minimum coins needed
  return dp[amount] === Infinity ? -1 : dp[amount];
}

module.exports = { knapsack01, longestCommonSubsequence, coinChange };

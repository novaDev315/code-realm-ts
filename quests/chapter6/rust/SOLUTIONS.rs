// Knapsack solves the 0/1 Knapsack problem using dynamic programming
pub fn knapsack_solution(weights: Vec<i32>, values: Vec<i32>, capacity: i32) -> i32 {
    let n = weights.len();
    let cap = capacity as usize;

    // Create DP table: dp[i][w] = max value using items 0..i-1 with capacity w
    let mut dp = vec![vec![0; cap + 1]; n + 1];

    // Build DP table
    for i in 1..=n {
        for w in 0..=cap {
            // Option 1: Don't take item i-1
            dp[i][w] = dp[i - 1][w];

            // Option 2: Take item i-1 (if it fits)
            if weights[i - 1] <= w as i32 {
                let take_value = values[i - 1] + dp[i - 1][w - weights[i - 1] as usize];
                dp[i][w] = dp[i][w].max(take_value);
            }
        }
    }

    dp[n][cap]
}

// LongestCommonSubsequence finds the length of LCS using dynamic programming
pub fn longest_common_subsequence_solution(str1: &str, str2: &str) -> i32 {
    let m = str1.len();
    let n = str2.len();
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();

    // Create DP table: dp[i][j] = length of LCS of str1[0..i-1] and str2[0..j-1]
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Fill DP table
    for i in 1..=m {
        for j in 1..=n {
            if chars1[i - 1] == chars2[j - 1] {
                // Characters match: add 1 to the LCS length
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                // Characters don't match: take max of two options
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[m][n]
}

// CoinChange finds the minimum number of coins needed using dynamic programming
pub fn coin_change_solution(coins: Vec<i32>, amount: i32) -> i32 {
    let amt = amount as usize;

    // DP array where dp[i] = minimum coins needed to make amount i
    let mut dp = vec![i32::MAX; amt + 1];
    dp[0] = 0; // 0 coins needed to make amount 0

    for i in 1..=amt {
        for &coin in &coins {
            if coin <= i as i32 && dp[i - coin as usize] != i32::MAX {
                // Try using this coin
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            }
        }
    }

    // Return result: -1 if impossible, otherwise the minimum coins needed
    if dp[amt] == i32::MAX {
        -1
    } else {
        dp[amt]
    }
}

# Reference solutions for Chapter 6: Caves of Shadows


def knapsack_01(weights: list[int], values: list[int], capacity: int) -> int:
    """Implement 0/1 Knapsack using dynamic programming.

    Return maximum value achievable within capacity.
    """
    n = len(weights)

    # Create DP table: dp[i][w] = max value using items 0..i-1 with capacity w
    dp = [[0] * (capacity + 1) for _ in range(n + 1)]

    # Build DP table
    for i in range(1, n + 1):
        for w in range(capacity + 1):
            # Option 1: Don't take item i-1
            dp[i][w] = dp[i - 1][w]

            # Option 2: Take item i-1 (if it fits)
            if weights[i - 1] <= w:
                dp[i][w] = max(
                    dp[i][w],
                    values[i - 1] + dp[i - 1][w - weights[i - 1]]
                )

    return dp[n][capacity]


def longest_common_subsequence(str1: str, str2: str) -> int:
    """Implement LCS using dynamic programming.

    Return length of longest common subsequence.
    """
    m = len(str1)
    n = len(str2)

    # Create DP table: dp[i][j] = length of LCS of str1[0..i-1] and str2[0..j-1]
    dp = [[0] * (n + 1) for _ in range(m + 1)]

    # Fill DP table
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if str1[i - 1] == str2[j - 1]:
                # Characters match: add 1 to the LCS length
                dp[i][j] = dp[i - 1][j - 1] + 1
            else:
                # Characters don't match: take max of two options
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])

    return dp[m][n]


def coin_change(coins: list[int], amount: int) -> int:
    """Implement coin change using dynamic programming.

    Return minimum number of coins, or -1 if impossible.
    """
    # DP array where dp[i] = minimum coins needed to make amount i
    dp = [float('inf')] * (amount + 1)
    dp[0] = 0  # 0 coins needed to make amount 0

    for i in range(1, amount + 1):
        for coin in coins:
            if coin <= i:
                # Try using this coin
                dp[i] = min(dp[i], dp[i - coin] + 1)

    # Return result: -1 if impossible, otherwise the minimum coins needed
    return -1 if dp[amount] == float('inf') else dp[amount]

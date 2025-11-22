// Test suite for Chapter 6: Caves of Shadows - Dynamic Programming

public class Check {
    public static void main(String[] args) {
        boolean passed = true;

        // Test Knapsack 01
        System.out.println("Testing 0/1 Knapsack...");

        // Test case 1: Basic knapsack
        int[] weights1 = {2, 3, 4, 5};
        int[] values1 = {3, 4, 5, 6};
        int capacity1 = 5;
        int knapsackResult1 = DP.knapsack(weights1, values1, capacity1);
        // Optimal: items with weight 2 (value 3) + weight 3 (value 4) = weight 5, value 7
        if (knapsackResult1 != 7) {
            System.err.println("❌ Knapsack test 1 failed. Expected 7, got: " + knapsackResult1);
            passed = false;
        } else {
            System.out.println("✓ Knapsack test 1 passed: max value = " + knapsackResult1);
        }

        // Test case 2: Single item exceeds capacity
        int[] weights2 = {10, 20, 30};
        int[] values2 = {100, 200, 300};
        int capacity2 = 5;
        int knapsackResult2 = DP.knapsack(weights2, values2, capacity2);
        if (knapsackResult2 != 0) {
            System.err.println("❌ Knapsack test 2 failed. Expected 0 (no items fit), got: " + knapsackResult2);
            passed = false;
        } else {
            System.out.println("✓ Knapsack test 2 passed: no items fit in capacity (value = " + knapsackResult2 + ")");
        }

        // Test case 3: All items fit
        int[] weights3 = {1, 2, 3};
        int[] values3 = {10, 20, 30};
        int capacity3 = 10;
        int knapsackResult3 = DP.knapsack(weights3, values3, capacity3);
        // All items fit: weight 1+2+3=6 <= 10, value 10+20+30=60
        if (knapsackResult3 != 60) {
            System.err.println("❌ Knapsack test 3 failed. Expected 60, got: " + knapsackResult3);
            passed = false;
        } else {
            System.out.println("✓ Knapsack test 3 passed: all items fit (value = " + knapsackResult3 + ")");
        }

        // Test LCS
        System.out.println("\nTesting Longest Common Subsequence...");

        // Test case 1: Simple LCS
        int lcsResult1 = DP.longestCommonSubsequence("abcde", "ace");
        // "ace" is a subsequence of "abcde", length = 3
        if (lcsResult1 != 3) {
            System.err.println("❌ LCS test 1 failed. Expected 3, got: " + lcsResult1);
            passed = false;
        } else {
            System.out.println("✓ LCS test 1 passed: LCS length = " + lcsResult1);
        }

        // Test case 2: No common subsequence (except empty)
        int lcsResult2 = DP.longestCommonSubsequence("abc", "def");
        if (lcsResult2 != 0) {
            System.err.println("❌ LCS test 2 failed. Expected 0, got: " + lcsResult2);
            passed = false;
        } else {
            System.out.println("✓ LCS test 2 passed: no common subsequence (length = " + lcsResult2 + ")");
        }

        // Test case 3: Complete match
        int lcsResult3 = DP.longestCommonSubsequence("hello", "hello");
        if (lcsResult3 != 5) {
            System.err.println("❌ LCS test 3 failed. Expected 5, got: " + lcsResult3);
            passed = false;
        } else {
            System.out.println("✓ LCS test 3 passed: complete match (length = " + lcsResult3 + ")");
        }

        // Test case 4: Complex LCS
        int lcsResult4 = DP.longestCommonSubsequence("AGGTAB", "GXTXAYB");
        // LCS is "GTAB", length = 4
        if (lcsResult4 != 4) {
            System.err.println("❌ LCS test 4 failed. Expected 4, got: " + lcsResult4);
            passed = false;
        } else {
            System.out.println("✓ LCS test 4 passed: complex case (length = " + lcsResult4 + ")");
        }

        // Test Coin Change
        System.out.println("\nTesting Coin Change...");

        // Test case 1: Standard coins
        int coinResult1 = DP.coinChange(new int[]{1, 2, 5}, 5);
        // Can use [5] or [2,2,1], minimum is 1 coin (using 5)
        if (coinResult1 != 1) {
            System.err.println("❌ Coin change test 1 failed. Expected 1, got: " + coinResult1);
            passed = false;
        } else {
            System.out.println("✓ Coin change test 1 passed: minimum coins for amount 5 = " + coinResult1);
        }

        // Test case 2: Multiple coins needed
        int coinResult2 = DP.coinChange(new int[]{2}, 3);
        // Can't make 3 with only 2-cent coins
        if (coinResult2 != -1) {
            System.err.println("❌ Coin change test 2 failed. Expected -1, got: " + coinResult2);
            passed = false;
        } else {
            System.out.println("✓ Coin change test 2 passed: impossible amount (returned " + coinResult2 + ")");
        }

        // Test case 3: Zero amount
        int coinResult3 = DP.coinChange(new int[]{1, 2, 5}, 0);
        if (coinResult3 != 0) {
            System.err.println("❌ Coin change test 3 failed. Expected 0, got: " + coinResult3);
            passed = false;
        } else {
            System.out.println("✓ Coin change test 3 passed: zero amount needs " + coinResult3 + " coins");
        }

        // Test case 4: Complex change-making
        int coinResult4 = DP.coinChange(new int[]{2, 3, 5}, 11);
        // Can use [5, 3, 3] = 3 coins
        // Can use [5, 2, 2, 2] = 4 coins
        // Minimum is 3
        if (coinResult4 != 3) {
            System.err.println("❌ Coin change test 4 failed. Expected 3, got: " + coinResult4);
            passed = false;
        } else {
            System.out.println("✓ Coin change test 4 passed: amount 11 needs " + coinResult4 + " coins");
        }

        // Test case 5: Large amount
        int coinResult5 = DP.coinChange(new int[]{1, 2, 5}, 20);
        // Minimum: 4 coins of 5
        if (coinResult5 != 4) {
            System.err.println("❌ Coin change test 5 failed. Expected 4, got: " + coinResult5);
            passed = false;
        } else {
            System.out.println("✓ Coin change test 5 passed: amount 20 needs " + coinResult5 + " coins");
        }

        if (passed) {
            System.out.println("\n✅ All tests passed!");
        } else {
            System.out.println("\n❌ Some tests failed.");
            System.exit(1);
        }
    }
}

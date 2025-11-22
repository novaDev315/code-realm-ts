// Test suite for Chapter 6: Caves of Shadows - Dynamic Programming

#include <iostream>
#include <vector>
#include <string>
#include "dp.cpp"

int main() {
    bool passed = true;

    // Test Knapsack 01
    std::cout << "Testing 0/1 Knapsack..." << std::endl;

    // Test case 1: Basic knapsack
    std::vector<int> weights1 = {2, 3, 4, 5};
    std::vector<int> values1 = {3, 4, 5, 6};
    int capacity1 = 5;
    int knapsackResult1 = knapsack(weights1, values1, capacity1);
    // Optimal: items with weight 2 (value 3) + weight 3 (value 4) = weight 5, value 7
    if (knapsackResult1 != 7) {
        std::cerr << "❌ Knapsack test 1 failed. Expected 7, got: " << knapsackResult1 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ Knapsack test 1 passed: max value = " << knapsackResult1 << std::endl;
    }

    // Test case 2: Single item exceeds capacity
    std::vector<int> weights2 = {10, 20, 30};
    std::vector<int> values2 = {100, 200, 300};
    int capacity2 = 5;
    int knapsackResult2 = knapsack(weights2, values2, capacity2);
    if (knapsackResult2 != 0) {
        std::cerr << "❌ Knapsack test 2 failed. Expected 0 (no items fit), got: " << knapsackResult2 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ Knapsack test 2 passed: no items fit in capacity (value = " << knapsackResult2 << ")" << std::endl;
    }

    // Test case 3: All items fit
    std::vector<int> weights3 = {1, 2, 3};
    std::vector<int> values3 = {10, 20, 30};
    int capacity3 = 10;
    int knapsackResult3 = knapsack(weights3, values3, capacity3);
    // All items fit: weight 1+2+3=6 <= 10, value 10+20+30=60
    if (knapsackResult3 != 60) {
        std::cerr << "❌ Knapsack test 3 failed. Expected 60, got: " << knapsackResult3 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ Knapsack test 3 passed: all items fit (value = " << knapsackResult3 << ")" << std::endl;
    }

    // Test LCS
    std::cout << "\nTesting Longest Common Subsequence..." << std::endl;

    // Test case 1: Simple LCS
    int lcsResult1 = longestCommonSubsequence("abcde", "ace");
    // "ace" is a subsequence of "abcde", length = 3
    if (lcsResult1 != 3) {
        std::cerr << "❌ LCS test 1 failed. Expected 3, got: " << lcsResult1 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ LCS test 1 passed: LCS length = " << lcsResult1 << std::endl;
    }

    // Test case 2: No common subsequence (except empty)
    int lcsResult2 = longestCommonSubsequence("abc", "def");
    if (lcsResult2 != 0) {
        std::cerr << "❌ LCS test 2 failed. Expected 0, got: " << lcsResult2 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ LCS test 2 passed: no common subsequence (length = " << lcsResult2 << ")" << std::endl;
    }

    // Test case 3: Complete match
    int lcsResult3 = longestCommonSubsequence("hello", "hello");
    if (lcsResult3 != 5) {
        std::cerr << "❌ LCS test 3 failed. Expected 5, got: " << lcsResult3 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ LCS test 3 passed: complete match (length = " << lcsResult3 << ")" << std::endl;
    }

    // Test case 4: Complex LCS
    int lcsResult4 = longestCommonSubsequence("AGGTAB", "GXTXAYB");
    // LCS is "GTAB", length = 4
    if (lcsResult4 != 4) {
        std::cerr << "❌ LCS test 4 failed. Expected 4, got: " << lcsResult4 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ LCS test 4 passed: complex case (length = " << lcsResult4 << ")" << std::endl;
    }

    // Test Coin Change
    std::cout << "\nTesting Coin Change..." << std::endl;

    // Test case 1: Standard coins
    int coinResult1 = coinChange({1, 2, 5}, 5);
    // Can use [5] or [2,2,1], minimum is 1 coin (using 5)
    if (coinResult1 != 1) {
        std::cerr << "❌ Coin change test 1 failed. Expected 1, got: " << coinResult1 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ Coin change test 1 passed: minimum coins for amount 5 = " << coinResult1 << std::endl;
    }

    // Test case 2: Multiple coins needed
    int coinResult2 = coinChange({2}, 3);
    // Can't make 3 with only 2-cent coins
    if (coinResult2 != -1) {
        std::cerr << "❌ Coin change test 2 failed. Expected -1, got: " << coinResult2 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ Coin change test 2 passed: impossible amount (returned " << coinResult2 << ")" << std::endl;
    }

    // Test case 3: Zero amount
    int coinResult3 = coinChange({1, 2, 5}, 0);
    if (coinResult3 != 0) {
        std::cerr << "❌ Coin change test 3 failed. Expected 0, got: " << coinResult3 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ Coin change test 3 passed: zero amount needs " << coinResult3 << " coins" << std::endl;
    }

    // Test case 4: Complex change-making
    int coinResult4 = coinChange({2, 3, 5}, 11);
    // Can use [5, 3, 3] = 3 coins
    // Can use [5, 2, 2, 2] = 4 coins
    // Minimum is 3
    if (coinResult4 != 3) {
        std::cerr << "❌ Coin change test 4 failed. Expected 3, got: " << coinResult4 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ Coin change test 4 passed: amount 11 needs " << coinResult4 << " coins" << std::endl;
    }

    // Test case 5: Large amount
    int coinResult5 = coinChange({1, 2, 5}, 20);
    // Minimum: 4 coins of 5
    if (coinResult5 != 4) {
        std::cerr << "❌ Coin change test 5 failed. Expected 4, got: " << coinResult5 << std::endl;
        passed = false;
    } else {
        std::cout << "✓ Coin change test 5 passed: amount 20 needs " << coinResult5 << " coins" << std::endl;
    }

    if (passed) {
        std::cout << "\n✅ All tests passed!" << std::endl;
        return 0;
    } else {
        std::cout << "\n❌ Some tests failed." << std::endl;
        return 1;
    }
}

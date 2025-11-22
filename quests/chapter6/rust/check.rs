mod dp;

use dp::{knapsack, longest_common_subsequence, coin_change};

fn main() {
    let mut passed = true;

    // Test Knapsack 01
    println!("Testing 0/1 Knapsack...");

    // Test case 1: Basic knapsack
    let weights1 = vec![2, 3, 4, 5];
    let values1 = vec![3, 4, 5, 6];
    let capacity1 = 5;
    let knapsack_result1 = knapsack(weights1, values1, capacity1);
    // Optimal: items with weight 2 (value 3) + weight 3 (value 4) = weight 5, value 7
    if knapsack_result1 != 7 {
        eprintln!("❌ Knapsack test 1 failed. Expected 7, got: {}", knapsack_result1);
        passed = false;
    } else {
        println!("✓ Knapsack test 1 passed: max value = {}", knapsack_result1);
    }

    // Test case 2: Single item exceeds capacity
    let weights2 = vec![10, 20, 30];
    let values2 = vec![100, 200, 300];
    let capacity2 = 5;
    let knapsack_result2 = knapsack(weights2, values2, capacity2);
    if knapsack_result2 != 0 {
        eprintln!("❌ Knapsack test 2 failed. Expected 0 (no items fit), got: {}", knapsack_result2);
        passed = false;
    } else {
        println!("✓ Knapsack test 2 passed: no items fit in capacity (value = {})", knapsack_result2);
    }

    // Test case 3: All items fit
    let weights3 = vec![1, 2, 3];
    let values3 = vec![10, 20, 30];
    let capacity3 = 10;
    let knapsack_result3 = knapsack(weights3, values3, capacity3);
    // All items fit: weight 1+2+3=6 <= 10, value 10+20+30=60
    if knapsack_result3 != 60 {
        eprintln!("❌ Knapsack test 3 failed. Expected 60, got: {}", knapsack_result3);
        passed = false;
    } else {
        println!("✓ Knapsack test 3 passed: all items fit (value = {})", knapsack_result3);
    }

    // Test LCS
    println!("\nTesting Longest Common Subsequence...");

    // Test case 1: Simple LCS
    let lcs_result1 = longest_common_subsequence("abcde", "ace");
    // "ace" is a subsequence of "abcde", length = 3
    if lcs_result1 != 3 {
        eprintln!("❌ LCS test 1 failed. Expected 3, got: {}", lcs_result1);
        passed = false;
    } else {
        println!("✓ LCS test 1 passed: LCS length = {}", lcs_result1);
    }

    // Test case 2: No common subsequence (except empty)
    let lcs_result2 = longest_common_subsequence("abc", "def");
    if lcs_result2 != 0 {
        eprintln!("❌ LCS test 2 failed. Expected 0, got: {}", lcs_result2);
        passed = false;
    } else {
        println!("✓ LCS test 2 passed: no common subsequence (length = {})", lcs_result2);
    }

    // Test case 3: Complete match
    let lcs_result3 = longest_common_subsequence("hello", "hello");
    if lcs_result3 != 5 {
        eprintln!("❌ LCS test 3 failed. Expected 5, got: {}", lcs_result3);
        passed = false;
    } else {
        println!("✓ LCS test 3 passed: complete match (length = {})", lcs_result3);
    }

    // Test case 4: Complex LCS
    let lcs_result4 = longest_common_subsequence("AGGTAB", "GXTXAYB");
    // LCS is "GTAB", length = 4
    if lcs_result4 != 4 {
        eprintln!("❌ LCS test 4 failed. Expected 4, got: {}", lcs_result4);
        passed = false;
    } else {
        println!("✓ LCS test 4 passed: complex case (length = {})", lcs_result4);
    }

    // Test Coin Change
    println!("\nTesting Coin Change...");

    // Test case 1: Standard coins
    let coin_result1 = coin_change(vec![1, 2, 5], 5);
    // Can use [5] or [2,2,1], minimum is 1 coin (using 5)
    if coin_result1 != 1 {
        eprintln!("❌ Coin change test 1 failed. Expected 1, got: {}", coin_result1);
        passed = false;
    } else {
        println!("✓ Coin change test 1 passed: minimum coins for amount 5 = {}", coin_result1);
    }

    // Test case 2: Multiple coins needed
    let coin_result2 = coin_change(vec![2], 3);
    // Can't make 3 with only 2-cent coins
    if coin_result2 != -1 {
        eprintln!("❌ Coin change test 2 failed. Expected -1, got: {}", coin_result2);
        passed = false;
    } else {
        println!("✓ Coin change test 2 passed: impossible amount (returned {})", coin_result2);
    }

    // Test case 3: Zero amount
    let coin_result3 = coin_change(vec![1, 2, 5], 0);
    if coin_result3 != 0 {
        eprintln!("❌ Coin change test 3 failed. Expected 0, got: {}", coin_result3);
        passed = false;
    } else {
        println!("✓ Coin change test 3 passed: zero amount needs {} coins", coin_result3);
    }

    // Test case 4: Complex change-making
    let coin_result4 = coin_change(vec![2, 3, 5], 11);
    // Can use [5, 3, 3] = 3 coins
    // Can use [5, 2, 2, 2] = 4 coins
    // Minimum is 3
    if coin_result4 != 3 {
        eprintln!("❌ Coin change test 4 failed. Expected 3, got: {}", coin_result4);
        passed = false;
    } else {
        println!("✓ Coin change test 4 passed: amount 11 needs {} coins", coin_result4);
    }

    // Test case 5: Large amount
    let coin_result5 = coin_change(vec![1, 2, 5], 20);
    // Minimum: 4 coins of 5
    if coin_result5 != 4 {
        eprintln!("❌ Coin change test 5 failed. Expected 4, got: {}", coin_result5);
        passed = false;
    } else {
        println!("✓ Coin change test 5 passed: amount 20 needs {} coins", coin_result5);
    }

    if passed {
        println!("\n✅ All tests passed!");
    } else {
        eprintln!("\n❌ Some tests failed.");
        std::process::exit(1);
    }
}

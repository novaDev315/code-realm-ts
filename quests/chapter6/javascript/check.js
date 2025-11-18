const { knapsack01, longestCommonSubsequence, coinChange } = require("./dp");

function runCheck() {
  let passed = true;

  // Test Knapsack 01
  console.log("Testing 0/1 Knapsack...");

  // Test case 1: Basic knapsack
  const weights1 = [2, 3, 4, 5];
  const values1 = [3, 4, 5, 6];
  const capacity1 = 5;
  const knapsackResult1 = knapsack01(weights1, values1, capacity1);
  // Optimal: items with weight 2 (value 3) + weight 3 (value 4) = weight 5, value 7
  if (knapsackResult1 !== 7) {
    console.error(
      `❌ Knapsack test 1 failed. Expected 7, got: ${knapsackResult1}`
    );
    passed = false;
  } else {
    console.log(`✓ Knapsack test 1 passed: max value = ${knapsackResult1}`);
  }

  // Test case 2: Single item exceeds capacity
  const weights2 = [10, 20, 30];
  const values2 = [100, 200, 300];
  const capacity2 = 5;
  const knapsackResult2 = knapsack01(weights2, values2, capacity2);
  if (knapsackResult2 !== 0) {
    console.error(
      `❌ Knapsack test 2 failed. Expected 0 (no items fit), got: ${knapsackResult2}`
    );
    passed = false;
  } else {
    console.log(
      `✓ Knapsack test 2 passed: no items fit in capacity (value = ${knapsackResult2})`
    );
  }

  // Test case 3: All items fit
  const weights3 = [1, 2, 3];
  const values3 = [10, 20, 30];
  const capacity3 = 10;
  const knapsackResult3 = knapsack01(weights3, values3, capacity3);
  // All items fit: weight 1+2+3=6 <= 10, value 10+20+30=60
  if (knapsackResult3 !== 60) {
    console.error(
      `❌ Knapsack test 3 failed. Expected 60, got: ${knapsackResult3}`
    );
    passed = false;
  } else {
    console.log(`✓ Knapsack test 3 passed: all items fit (value = ${knapsackResult3})`);
  }

  // Test LCS
  console.log("\nTesting Longest Common Subsequence...");

  // Test case 1: Simple LCS
  const lcsResult1 = longestCommonSubsequence("abcde", "ace");
  // "ace" is a subsequence of "abcde", length = 3
  if (lcsResult1 !== 3) {
    console.error(`❌ LCS test 1 failed. Expected 3, got: ${lcsResult1}`);
    passed = false;
  } else {
    console.log(`✓ LCS test 1 passed: LCS length = ${lcsResult1}`);
  }

  // Test case 2: No common subsequence (except empty)
  const lcsResult2 = longestCommonSubsequence("abc", "def");
  if (lcsResult2 !== 0) {
    console.error(`❌ LCS test 2 failed. Expected 0, got: ${lcsResult2}`);
    passed = false;
  } else {
    console.log(`✓ LCS test 2 passed: no common subsequence (length = ${lcsResult2})`);
  }

  // Test case 3: Complete match
  const lcsResult3 = longestCommonSubsequence("hello", "hello");
  if (lcsResult3 !== 5) {
    console.error(`❌ LCS test 3 failed. Expected 5, got: ${lcsResult3}`);
    passed = false;
  } else {
    console.log(`✓ LCS test 3 passed: complete match (length = ${lcsResult3})`);
  }

  // Test case 4: Complex LCS
  const lcsResult4 = longestCommonSubsequence("AGGTAB", "GXTXAYB");
  // LCS is "GTAB", length = 4
  if (lcsResult4 !== 4) {
    console.error(`❌ LCS test 4 failed. Expected 4, got: ${lcsResult4}`);
    passed = false;
  } else {
    console.log(
      `✓ LCS test 4 passed: complex case (length = ${lcsResult4})`
    );
  }

  // Test Coin Change
  console.log("\nTesting Coin Change...");

  // Test case 1: Standard coins
  const coinResult1 = coinChange([1, 2, 5], 5);
  // Can use [5] or [2,2,1], minimum is 1 coin (using 5)
  if (coinResult1 !== 1) {
    console.error(
      `❌ Coin change test 1 failed. Expected 1, got: ${coinResult1}`
    );
    passed = false;
  } else {
    console.log(
      `✓ Coin change test 1 passed: minimum coins for amount 5 = ${coinResult1}`
    );
  }

  // Test case 2: Multiple coins needed
  const coinResult2 = coinChange([2], 3);
  // Can't make 3 with only 2-cent coins
  if (coinResult2 !== -1) {
    console.error(
      `❌ Coin change test 2 failed. Expected -1, got: ${coinResult2}`
    );
    passed = false;
  } else {
    console.log(
      `✓ Coin change test 2 passed: impossible amount (returned ${coinResult2})`
    );
  }

  // Test case 3: Zero amount
  const coinResult3 = coinChange([1, 2, 5], 0);
  if (coinResult3 !== 0) {
    console.error(`❌ Coin change test 3 failed. Expected 0, got: ${coinResult3}`);
    passed = false;
  } else {
    console.log(
      `✓ Coin change test 3 passed: zero amount needs ${coinResult3} coins`
    );
  }

  // Test case 4: Complex change-making
  const coinResult4 = coinChange([2, 3, 5], 11);
  // Can use [5, 3, 3] = 3 coins (minimum)
  if (coinResult4 !== 3) {
    console.error(
      `❌ Coin change test 4 failed. Expected 3, got: ${coinResult4}`
    );
    passed = false;
  } else {
    console.log(
      `✓ Coin change test 4 passed: amount 11 needs ${coinResult4} coins`
    );
  }

  // Test case 5: Large amount
  const coinResult5 = coinChange([1, 2, 5], 20);
  // Minimum: 4 coins of 5
  if (coinResult5 !== 4) {
    console.error(
      `❌ Coin change test 5 failed. Expected 4, got: ${coinResult5}`
    );
    passed = false;
  } else {
    console.log(
      `✓ Coin change test 5 passed: amount 20 needs ${coinResult5} coins`
    );
  }

  if (passed) {
    console.log("\n✅ All tests passed!");
  } else {
    console.log("\n❌ Some tests failed.");
    process.exit(1);
  }

  return passed;
}

if (require.main === module) {
  runCheck();
}

module.exports = { runCheck };

from dp import knapsack_01, longest_common_subsequence, coin_change


def run_check():
    """Run comprehensive tests for dynamic programming algorithms."""
    passed = True

    # Test Knapsack 01
    print("Testing 0/1 Knapsack...")

    # Test case 1: Basic knapsack
    weights1 = [2, 3, 4, 5]
    values1 = [3, 4, 5, 6]
    capacity1 = 5
    knapsack_result1 = knapsack_01(weights1, values1, capacity1)
    # Optimal: items with weight 2 (value 3) + weight 3 (value 4) = weight 5, value 7
    if knapsack_result1 != 7:
        print(f"❌ Knapsack test 1 failed. Expected 7, got: {knapsack_result1}")
        passed = False
    else:
        print(f"✓ Knapsack test 1 passed: max value = {knapsack_result1}")

    # Test case 2: Single item exceeds capacity
    weights2 = [10, 20, 30]
    values2 = [100, 200, 300]
    capacity2 = 5
    knapsack_result2 = knapsack_01(weights2, values2, capacity2)
    if knapsack_result2 != 0:
        print(
            f"❌ Knapsack test 2 failed. Expected 0 (no items fit), got: {knapsack_result2}"
        )
        passed = False
    else:
        print(f"✓ Knapsack test 2 passed: no items fit in capacity (value = {knapsack_result2})")

    # Test case 3: All items fit
    weights3 = [1, 2, 3]
    values3 = [10, 20, 30]
    capacity3 = 10
    knapsack_result3 = knapsack_01(weights3, values3, capacity3)
    # All items fit: weight 1+2+3=6 <= 10, value 10+20+30=60
    if knapsack_result3 != 60:
        print(
            f"❌ Knapsack test 3 failed. Expected 60, got: {knapsack_result3}"
        )
        passed = False
    else:
        print(f"✓ Knapsack test 3 passed: all items fit (value = {knapsack_result3})")

    # Test LCS
    print("\nTesting Longest Common Subsequence...")

    # Test case 1: Simple LCS
    lcs_result1 = longest_common_subsequence("abcde", "ace")
    # "ace" is a subsequence of "abcde", length = 3
    if lcs_result1 != 3:
        print(f"❌ LCS test 1 failed. Expected 3, got: {lcs_result1}")
        passed = False
    else:
        print(f"✓ LCS test 1 passed: LCS length = {lcs_result1}")

    # Test case 2: No common subsequence (except empty)
    lcs_result2 = longest_common_subsequence("abc", "def")
    if lcs_result2 != 0:
        print(f"❌ LCS test 2 failed. Expected 0, got: {lcs_result2}")
        passed = False
    else:
        print(f"✓ LCS test 2 passed: no common subsequence (length = {lcs_result2})")

    # Test case 3: Complete match
    lcs_result3 = longest_common_subsequence("hello", "hello")
    if lcs_result3 != 5:
        print(f"❌ LCS test 3 failed. Expected 5, got: {lcs_result3}")
        passed = False
    else:
        print(f"✓ LCS test 3 passed: complete match (length = {lcs_result3})")

    # Test case 4: Complex LCS
    lcs_result4 = longest_common_subsequence("AGGTAB", "GXTXAYB")
    # LCS is "GTAB", length = 4
    if lcs_result4 != 4:
        print(f"❌ LCS test 4 failed. Expected 4, got: {lcs_result4}")
        passed = False
    else:
        print(f"✓ LCS test 4 passed: complex case (length = {lcs_result4})")

    # Test Coin Change
    print("\nTesting Coin Change...")

    # Test case 1: Standard coins
    coin_result1 = coin_change([1, 2, 5], 5)
    # Can use [5] or [2,2,1], minimum is 1 coin (using 5)
    if coin_result1 != 1:
        print(
            f"❌ Coin change test 1 failed. Expected 1, got: {coin_result1}"
        )
        passed = False
    else:
        print(
            f"✓ Coin change test 1 passed: minimum coins for amount 5 = {coin_result1}"
        )

    # Test case 2: Multiple coins needed
    coin_result2 = coin_change([2], 3)
    # Can't make 3 with only 2-cent coins
    if coin_result2 != -1:
        print(
            f"❌ Coin change test 2 failed. Expected -1, got: {coin_result2}"
        )
        passed = False
    else:
        print(
            f"✓ Coin change test 2 passed: impossible amount (returned {coin_result2})"
        )

    # Test case 3: Zero amount
    coin_result3 = coin_change([1, 2, 5], 0)
    if coin_result3 != 0:
        print(f"❌ Coin change test 3 failed. Expected 0, got: {coin_result3}")
        passed = False
    else:
        print(
            f"✓ Coin change test 3 passed: zero amount needs {coin_result3} coins"
        )

    # Test case 4: Complex change-making
    coin_result4 = coin_change([2, 3, 5], 11)
    # Can use [5, 3, 3] = 3 coins (minimum)
    if coin_result4 != 3:
        print(
            f"❌ Coin change test 4 failed. Expected 3, got: {coin_result4}"
        )
        passed = False
    else:
        print(
            f"✓ Coin change test 4 passed: amount 11 needs {coin_result4} coins"
        )

    # Test case 5: Large amount
    coin_result5 = coin_change([1, 2, 5], 20)
    # Minimum: 4 coins of 5
    if coin_result5 != 4:
        print(
            f"❌ Coin change test 5 failed. Expected 4, got: {coin_result5}"
        )
        passed = False
    else:
        print(
            f"✓ Coin change test 5 passed: amount 20 needs {coin_result5} coins"
        )

    if passed:
        print("\n✅ All tests passed!")
    else:
        print("\n❌ Some tests failed.")
        exit(1)

    return passed


if __name__ == "__main__":
    run_check()

#!/usr/bin/env python3
from sliding import max_sum_subarray, two_sum, three_sum
import sys


def run_check() -> bool:
    """Run all test cases for Chapter 3."""
    passed = True

    print("Testing max_sum_subarray...")
    max_sum_cases = [
        {"arr": [2, 1, 5, 1, 3, 2], "k": 3, "expect": 9},
        {"arr": [2, 3, 4, 1, 5], "k": 2, "expect": 7},
        {"arr": [1, 4, 2, 10, 23, 3, 1, 0, 20], "k": 4, "expect": 39},
    ]

    for tc in max_sum_cases:
        result = max_sum_subarray(tc["arr"], tc["k"])
        if result != tc["expect"]:
            print(f"❌ max_sum_subarray({tc['arr']}, {tc['k']}) expected {tc['expect']} but got {result}")
            passed = False
        else:
            print(f"✓ max_sum_subarray k={tc['k']}")

    print("\nTesting two_sum...")
    two_sum_cases = [
        {"arr": [1, 2, 3, 4, 5], "target": 9, "expect_sum": 9},
        {"arr": [2, 7, 11, 15], "target": 9, "expect_sum": 9},
    ]

    for tc in two_sum_cases:
        result = two_sum(tc["arr"], tc["target"])
        if result is None:
            print(f"❌ two_sum({tc['arr']}, {tc['target']}) returned None")
            passed = False
        elif tc["arr"][result[0]] + tc["arr"][result[1]] != tc["expect_sum"]:
            print(f"❌ two_sum({tc['arr']}, {tc['target']}) indices don't sum to {tc['expect_sum']}")
            passed = False
        else:
            print(f"✓ two_sum target={tc['target']}")

    print("\nTesting three_sum...")
    three_sum_result = three_sum([-1, 0, 1, 2, -1, -4], 0)
    if len(three_sum_result) == 0:
        print("❌ three_sum should find at least one triplet")
        passed = False
    else:
        valid_triplets = all(sum(triplet) == 0 for triplet in three_sum_result)
        if valid_triplets:
            print(f"✓ three_sum found {len(three_sum_result)} valid triplet(s)")
        else:
            print("❌ three_sum returned invalid triplets")
            passed = False

    if passed:
        print("\n✅ All tests passed!")
    else:
        print("\n❌ Some tests failed.")
        sys.exit(1)

    return passed


if __name__ == "__main__":
    run_check()

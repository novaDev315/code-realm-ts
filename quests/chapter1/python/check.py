#!/usr/bin/env python3
from recursion import fibonacci, factorial, string_permutations
import sys


def run_check() -> bool:
    """Run all test cases for Chapter 1."""
    passed = True

    # Fibonacci test cases
    fib_cases = [
        {"input": 5, "expect": 5},
        {"input": 8, "expect": 21}
    ]

    for case in fib_cases:
        result = fibonacci(case["input"])
        if result != case["expect"]:
            print(f"fibonacci({case['input']}) expected {case['expect']} but got {result}")
            passed = False

    # Factorial test cases
    fact_cases = [{"input": 5, "expect": 120}]
    for case in fact_cases:
        result = factorial(case["input"])
        if result != case["expect"]:
            print(f"factorial({case['input']}) expected {case['expect']} but got {result}")
            passed = False

    # String permutations test cases
    perm_cases = [
        {"input": "abc", "expect": 6},
        {"input": "aab", "expect": 3}
    ]
    for case in perm_cases:
        result = string_permutations(case["input"])
        if len(result) != case["expect"]:
            print(f"string_permutations('{case['input']}') expected {case['expect']} permutations but got {len(result)}")
            passed = False

    if passed:
        print("✅ All tests passed!")
    else:
        print("❌ Some tests failed.")
        sys.exit(1)

    return passed


if __name__ == "__main__":
    run_check()

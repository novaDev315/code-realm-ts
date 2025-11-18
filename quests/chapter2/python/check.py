#!/usr/bin/env python3
from sorting import merge_sort, quick_sort, is_sorted
import sys
import random


def run_check() -> bool:
    """Run all test cases for Chapter 2."""
    passed = True

    test_cases = [
        {"input": [5, 2, 8, 1, 9], "expect": [1, 2, 5, 8, 9], "name": "basic array"},
        {"input": [3, 3, 1, 2], "expect": [1, 2, 3, 3], "name": "duplicates"},
        {"input": [1], "expect": [1], "name": "single element"},
        {"input": [], "expect": [], "name": "empty array"},
        {"input": [5, 4, 3, 2, 1], "expect": [1, 2, 3, 4, 5], "name": "reverse sorted"},
        {"input": [1, 2, 3, 4, 5], "expect": [1, 2, 3, 4, 5], "name": "already sorted"},
    ]

    print("Testing MergeSort...")
    for tc in test_cases:
        result = merge_sort(tc["input"][:])
        if result != tc["expect"]:
            print(f"❌ merge_sort({tc['input']}) [{tc['name']}] expected {tc['expect']} but got {result}")
            passed = False
        else:
            print(f"✓ merge_sort {tc['name']}")

    print("\nTesting QuickSort...")
    for tc in test_cases:
        result = quick_sort(tc["input"][:])
        if result != tc["expect"]:
            print(f"❌ quick_sort({tc['input']}) [{tc['name']}] expected {tc['expect']} but got {result}")
            passed = False
        else:
            print(f"✓ quick_sort {tc['name']}")

    # Verify both work on large arrays
    print("\nTesting performance on large array...")
    large_array = [random.randint(0, 999) for _ in range(1000)]

    merge_sorted = merge_sort(large_array[:])
    quick_sorted = quick_sort(large_array[:])

    if not is_sorted(merge_sorted):
        print("❌ merge_sort failed to sort large array")
        passed = False
    else:
        print("✓ merge_sort handles large arrays")

    if not is_sorted(quick_sorted):
        print("❌ quick_sort failed to sort large array")
        passed = False
    else:
        print("✓ quick_sort handles large arrays")

    if passed:
        print("\n✅ All tests passed!")
    else:
        print("\n❌ Some tests failed.")
        sys.exit(1)

    return passed


if __name__ == "__main__":
    run_check()

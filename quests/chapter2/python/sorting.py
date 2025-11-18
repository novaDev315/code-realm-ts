# Chapter 2: Mountains of Order
# Implement sorting algorithms

def merge_sort(arr: list[int]) -> list[int]:
    """Implement merge sort algorithm."""
    # TODO: Implement merge sort
    # Base case: array of length 0 or 1 is already sorted
    # Recursive case: divide, sort, and merge
    return arr


def quick_sort(arr: list[int]) -> list[int]:
    """Implement quick sort algorithm."""
    # TODO: Implement quick sort
    # Base case: array of length 0 or 1 is already sorted
    # Recursive case: partition around pivot and sort subarrays
    return arr


def is_sorted(arr: list[int]) -> bool:
    """Check if an array is sorted."""
    for i in range(1, len(arr)):
        if arr[i] < arr[i - 1]:
            return False
    return True

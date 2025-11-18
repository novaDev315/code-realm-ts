# Reference solutions for Chapter 3

def max_sum_subarray(arr: list[int], k: int) -> int:
    """Find maximum sum of subarray of size k using sliding window."""
    if len(arr) < k:
        return 0

    max_sum = 0
    window_sum = 0

    for i in range(k):
        window_sum += arr[i]
    max_sum = window_sum

    for i in range(k, len(arr)):
        window_sum = window_sum - arr[i - k] + arr[i]
        max_sum = max(max_sum, window_sum)

    return max_sum


def two_sum(arr: list[int], target: int) -> tuple[int, int] | None:
    """Find two indices where arr[i] + arr[j] = target."""
    left = 0
    right = len(arr) - 1

    while left < right:
        current_sum = arr[left] + arr[right]
        if current_sum == target:
            return (left, right)
        elif current_sum < target:
            left += 1
        else:
            right -= 1

    return None


def three_sum(arr: list[int], target: int) -> list[list[int]]:
    """Find all unique triplets that sum to target."""
    result = []
    arr.sort()

    for i in range(len(arr) - 2):
        if i > 0 and arr[i] == arr[i - 1]:
            continue

        left = i + 1
        right = len(arr) - 1

        while left < right:
            current_sum = arr[i] + arr[left] + arr[right]
            if current_sum == target:
                result.append([arr[i], arr[left], arr[right]])
                while left < right and arr[left] == arr[left + 1]:
                    left += 1
                while left < right and arr[right] == arr[right - 1]:
                    right -= 1
                left += 1
                right -= 1
            elif current_sum < target:
                left += 1
            else:
                right -= 1

    return result

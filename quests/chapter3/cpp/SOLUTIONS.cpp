// Reference solutions for Chapter 3

#include <vector>
#include <algorithm>

/**
 * Find maximum sum of subarray of size k using sliding window
 */
int maxSumSubarray(std::vector<int> arr, int k) {
    if (arr.size() < static_cast<size_t>(k)) return 0;

    int maxSum = 0;
    int windowSum = 0;

    // Calculate first window
    for (int i = 0; i < k; i++) {
        windowSum += arr[i];
    }
    maxSum = windowSum;

    // Slide the window
    for (size_t i = k; i < arr.size(); i++) {
        windowSum = windowSum - arr[i - k] + arr[i];
        maxSum = std::max(maxSum, windowSum);
    }

    return maxSum;
}

/**
 * Find two indices where arr[i] + arr[j] = target
 * Array is sorted. Use two pointers approach.
 */
std::vector<int> twoSum(std::vector<int> arr, int target) {
    int left = 0;
    int right = arr.size() - 1;

    while (left < right) {
        int sum = arr[left] + arr[right];
        if (sum == target) {
            return {left, right};
        } else if (sum < target) {
            left++;
        } else {
            right--;
        }
    }

    return {};
}

/**
 * Find all unique triplets that sum to target
 */
std::vector<std::vector<int>> threeSum(std::vector<int> arr, int target) {
    std::vector<std::vector<int>> result;
    std::sort(arr.begin(), arr.end());

    for (size_t i = 0; i < arr.size() - 2; i++) {
        // Skip duplicates
        if (i > 0 && arr[i] == arr[i - 1]) continue;

        int left = i + 1;
        int right = arr.size() - 1;

        while (left < right) {
            int sum = arr[i] + arr[left] + arr[right];
            if (sum == target) {
                result.push_back({arr[i], arr[left], arr[right]});
                // Skip duplicates
                while (left < right && arr[left] == arr[left + 1]) left++;
                while (left < right && arr[right] == arr[right - 1]) right--;
                left++;
                right--;
            } else if (sum < target) {
                left++;
            } else {
                right--;
            }
        }
    }

    return result;
}

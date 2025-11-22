// Chapter 3: Mirror Maze
// Sliding Window & Two Pointers

#include <vector>
#include <algorithm>

/**
 * Find maximum sum of subarray of size k using sliding window
 * @param arr input vector
 * @param k window size
 * @return maximum sum
 */
int maxSumSubarray(std::vector<int> arr, int k) {
    // TODO: Find maximum sum of subarray of size k using sliding window
    // 1. Calculate sum of first k elements
    // 2. Slide the window by removing leftmost element and adding next element
    // 3. Keep track of maximum sum
    return 0;
}

/**
 * Find two indices where arr[i] + arr[j] = target
 * Array is sorted. Use two pointers approach.
 * @param arr sorted input vector
 * @param target target sum
 * @return vector of two indices [i, j] or empty vector if no solution
 */
std::vector<int> twoSum(std::vector<int> arr, int target) {
    // TODO: Find two indices where arr[i] + arr[j] = target
    // 1. Use two pointers: left at start, right at end
    // 2. If sum equals target, return indices
    // 3. If sum < target, move left pointer right
    // 4. If sum > target, move right pointer left
    return {};
}

/**
 * Find all unique triplets that sum to target
 * @param arr input vector
 * @param target target sum
 * @return vector of triplets
 */
std::vector<std::vector<int>> threeSum(std::vector<int> arr, int target) {
    // TODO: Find all unique triplets that sum to target
    // 1. Sort the array
    // 2. For each element, use two pointers to find pairs that complete the triplet
    // 3. Skip duplicates to ensure unique triplets
    return {};
}

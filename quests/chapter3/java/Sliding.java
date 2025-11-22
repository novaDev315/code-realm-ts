// Chapter 3: Mirror Maze
// Sliding Window & Two Pointers

import java.util.ArrayList;
import java.util.List;

public class Sliding {

    /**
     * Find maximum sum of subarray of size k using sliding window
     * @param arr input array
     * @param k window size
     * @return maximum sum
     */
    public static int maxSumSubarray(int[] arr, int k) {
        // TODO: Find maximum sum of subarray of size k using sliding window
        // 1. Calculate sum of first k elements
        // 2. Slide the window by removing leftmost element and adding next element
        // 3. Keep track of maximum sum
        return 0;
    }

    /**
     * Find two indices where arr[i] + arr[j] = target
     * Array is sorted. Use two pointers approach.
     * @param arr sorted input array
     * @param target target sum
     * @return array of two indices [i, j] or null if no solution
     */
    public static int[] twoSum(int[] arr, int target) {
        // TODO: Find two indices where arr[i] + arr[j] = target
        // 1. Use two pointers: left at start, right at end
        // 2. If sum equals target, return indices
        // 3. If sum < target, move left pointer right
        // 4. If sum > target, move right pointer left
        return null;
    }

    /**
     * Find all unique triplets that sum to target
     * @param arr input array
     * @param target target sum
     * @return list of triplets
     */
    public static List<List<Integer>> threeSum(int[] arr, int target) {
        // TODO: Find all unique triplets that sum to target
        // 1. Sort the array
        // 2. For each element, use two pointers to find pairs that complete the triplet
        // 3. Skip duplicates to ensure unique triplets
        return new ArrayList<>();
    }
}

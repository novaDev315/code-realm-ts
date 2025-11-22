// Reference solutions for Chapter 3

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class SOLUTIONS {

    /**
     * Find maximum sum of subarray of size k using sliding window
     */
    public static int maxSumSubarray(int[] arr, int k) {
        if (arr.length < k) return 0;

        int maxSum = 0;
        int windowSum = 0;

        // Calculate first window
        for (int i = 0; i < k; i++) {
            windowSum += arr[i];
        }
        maxSum = windowSum;

        // Slide the window
        for (int i = k; i < arr.length; i++) {
            windowSum = windowSum - arr[i - k] + arr[i];
            maxSum = Math.max(maxSum, windowSum);
        }

        return maxSum;
    }

    /**
     * Find two indices where arr[i] + arr[j] = target
     * Array is sorted. Use two pointers approach.
     */
    public static int[] twoSum(int[] arr, int target) {
        int left = 0;
        int right = arr.length - 1;

        while (left < right) {
            int sum = arr[left] + arr[right];
            if (sum == target) {
                return new int[]{left, right};
            } else if (sum < target) {
                left++;
            } else {
                right--;
            }
        }

        return null;
    }

    /**
     * Find all unique triplets that sum to target
     */
    public static List<List<Integer>> threeSum(int[] arr, int target) {
        List<List<Integer>> result = new ArrayList<>();
        Arrays.sort(arr);

        for (int i = 0; i < arr.length - 2; i++) {
            // Skip duplicates
            if (i > 0 && arr[i] == arr[i - 1]) continue;

            int left = i + 1;
            int right = arr.length - 1;

            while (left < right) {
                int sum = arr[i] + arr[left] + arr[right];
                if (sum == target) {
                    result.add(Arrays.asList(arr[i], arr[left], arr[right]));
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
}

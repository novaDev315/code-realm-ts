import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * Reference solutions for Chapter 2: Mountains of Order
 */
public class SOLUTIONS {

    /**
     * Sorts an array using the merge sort algorithm.
     * Time Complexity: O(n log n)
     * Space Complexity: O(n)
     */
    public static int[] mergeSort(int[] arr) {
        if (arr.length <= 1) {
            return arr;
        }

        int mid = arr.length / 2;
        int[] left = mergeSort(Arrays.copyOfRange(arr, 0, mid));
        int[] right = mergeSort(Arrays.copyOfRange(arr, mid, arr.length));

        return merge(left, right);
    }

    /**
     * Merges two sorted arrays into one sorted array.
     */
    private static int[] merge(int[] left, int[] right) {
        int[] result = new int[left.length + right.length];
        int i = 0, j = 0, k = 0;

        while (i < left.length && j < right.length) {
            if (left[i] <= right[j]) {
                result[k++] = left[i++];
            } else {
                result[k++] = right[j++];
            }
        }

        while (i < left.length) {
            result[k++] = left[i++];
        }

        while (j < right.length) {
            result[k++] = right[j++];
        }

        return result;
    }

    /**
     * Sorts an array using the quick sort algorithm.
     * Time Complexity: O(n log n) average, O(n^2) worst case
     * Space Complexity: O(n)
     */
    public static int[] quickSort(int[] arr) {
        if (arr.length <= 1) {
            return arr;
        }

        int pivot = arr[arr.length / 2];
        List<Integer> left = new ArrayList<>();
        List<Integer> middle = new ArrayList<>();
        List<Integer> right = new ArrayList<>();

        for (int num : arr) {
            if (num < pivot) {
                left.add(num);
            } else if (num == pivot) {
                middle.add(num);
            } else {
                right.add(num);
            }
        }

        int[] sortedLeft = quickSort(toArray(left));
        int[] sortedRight = quickSort(toArray(right));

        return concatenate(sortedLeft, toArray(middle), sortedRight);
    }

    /**
     * Converts a List<Integer> to int[]
     */
    private static int[] toArray(List<Integer> list) {
        int[] arr = new int[list.size()];
        for (int i = 0; i < list.size(); i++) {
            arr[i] = list.get(i);
        }
        return arr;
    }

    /**
     * Concatenates three arrays into one.
     */
    private static int[] concatenate(int[] a, int[] b, int[] c) {
        int[] result = new int[a.length + b.length + c.length];
        int pos = 0;

        for (int num : a) result[pos++] = num;
        for (int num : b) result[pos++] = num;
        for (int num : c) result[pos++] = num;

        return result;
    }

    /**
     * Checks if an array is sorted in ascending order.
     */
    public static boolean isSorted(int[] arr) {
        for (int i = 1; i < arr.length; i++) {
            if (arr[i] < arr[i - 1]) {
                return false;
            }
        }
        return true;
    }
}

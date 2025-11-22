/**
 * Chapter 2: Mountains of Order - Sorting Algorithms
 *
 * Implement the two fundamental sorting algorithms:
 * - MergeSort: Divide and conquer approach with O(n log n) time complexity
 * - QuickSort: Partition-based approach with O(n log n) average time complexity
 */
public class Sorting {

    /**
     * Sorts an array using the merge sort algorithm.
     *
     * Algorithm:
     * 1. Divide the array into two halves
     * 2. Recursively sort each half
     * 3. Merge the sorted halves back together
     *
     * @param arr the array to sort
     * @return a new sorted array
     */
    public static int[] mergeSort(int[] arr) {
        // TODO: Implement merge sort
        // Hint: Base case is when array has 1 or 0 elements
        // Hint: You'll need a helper method to merge two sorted arrays
        throw new UnsupportedOperationException("mergeSort not implemented yet");
    }

    /**
     * Sorts an array using the quick sort algorithm.
     *
     * Algorithm:
     * 1. Choose a pivot element (e.g., middle element)
     * 2. Partition array into: elements < pivot, elements == pivot, elements > pivot
     * 3. Recursively sort the left and right partitions
     * 4. Concatenate: sorted(left) + middle + sorted(right)
     *
     * @param arr the array to sort
     * @return a new sorted array
     */
    public static int[] quickSort(int[] arr) {
        // TODO: Implement quick sort
        // Hint: Base case is when array has 1 or 0 elements
        // Hint: Choose the middle element as the pivot
        throw new UnsupportedOperationException("quickSort not implemented yet");
    }
}

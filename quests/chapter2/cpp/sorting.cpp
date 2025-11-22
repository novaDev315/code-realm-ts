#include <vector>
#include <stdexcept>

/**
 * Chapter 2: Mountains of Order - Sorting Algorithms
 *
 * Implement the two fundamental sorting algorithms:
 * - MergeSort: Divide and conquer approach with O(n log n) time complexity
 * - QuickSort: Partition-based approach with O(n log n) average time complexity
 */

/**
 * Sorts a vector using the merge sort algorithm.
 *
 * Algorithm:
 * 1. Divide the vector into two halves
 * 2. Recursively sort each half
 * 3. Merge the sorted halves back together
 *
 * @param arr the vector to sort
 * @return a new sorted vector
 */
std::vector<int> mergeSort(std::vector<int> arr) {
    // TODO: Implement merge sort
    // Hint: Base case is when vector has 1 or 0 elements
    // Hint: You'll need a helper function to merge two sorted vectors
    throw std::runtime_error("mergeSort not implemented yet");
}

/**
 * Sorts a vector using the quick sort algorithm.
 *
 * Algorithm:
 * 1. Choose a pivot element (e.g., middle element)
 * 2. Partition vector into: elements < pivot, elements == pivot, elements > pivot
 * 3. Recursively sort the left and right partitions
 * 4. Concatenate: sorted(left) + middle + sorted(right)
 *
 * @param arr the vector to sort
 * @return a new sorted vector
 */
std::vector<int> quickSort(std::vector<int> arr) {
    // TODO: Implement quick sort
    // Hint: Base case is when vector has 1 or 0 elements
    // Hint: Choose the middle element as the pivot
    throw std::runtime_error("quickSort not implemented yet");
}

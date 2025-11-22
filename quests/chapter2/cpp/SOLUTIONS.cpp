#include <vector>
#include <algorithm>

/**
 * Reference solutions for Chapter 2: Mountains of Order
 */

/**
 * Merges two sorted vectors into one sorted vector.
 */
std::vector<int> merge(const std::vector<int>& left, const std::vector<int>& right) {
    std::vector<int> result;
    size_t i = 0, j = 0;

    while (i < left.size() && j < right.size()) {
        if (left[i] <= right[j]) {
            result.push_back(left[i++]);
        } else {
            result.push_back(right[j++]);
        }
    }

    while (i < left.size()) {
        result.push_back(left[i++]);
    }

    while (j < right.size()) {
        result.push_back(right[j++]);
    }

    return result;
}

/**
 * Sorts a vector using the merge sort algorithm.
 * Time Complexity: O(n log n)
 * Space Complexity: O(n)
 */
std::vector<int> mergeSort(std::vector<int> arr) {
    if (arr.size() <= 1) {
        return arr;
    }

    size_t mid = arr.size() / 2;
    std::vector<int> left(arr.begin(), arr.begin() + mid);
    std::vector<int> right(arr.begin() + mid, arr.end());

    left = mergeSort(left);
    right = mergeSort(right);

    return merge(left, right);
}

/**
 * Sorts a vector using the quick sort algorithm.
 * Time Complexity: O(n log n) average, O(n^2) worst case
 * Space Complexity: O(n)
 */
std::vector<int> quickSort(std::vector<int> arr) {
    if (arr.size() <= 1) {
        return arr;
    }

    int pivot = arr[arr.size() / 2];
    std::vector<int> left, middle, right;

    for (int num : arr) {
        if (num < pivot) {
            left.push_back(num);
        } else if (num == pivot) {
            middle.push_back(num);
        } else {
            right.push_back(num);
        }
    }

    std::vector<int> sortedLeft = quickSort(left);
    std::vector<int> sortedRight = quickSort(right);

    // Concatenate: sortedLeft + middle + sortedRight
    std::vector<int> result;
    result.insert(result.end(), sortedLeft.begin(), sortedLeft.end());
    result.insert(result.end(), middle.begin(), middle.end());
    result.insert(result.end(), sortedRight.begin(), sortedRight.end());

    return result;
}

/**
 * Checks if a vector is sorted in ascending order.
 */
bool isSorted(const std::vector<int>& arr) {
    for (size_t i = 1; i < arr.size(); i++) {
        if (arr[i] < arr[i - 1]) {
            return false;
        }
    }
    return true;
}

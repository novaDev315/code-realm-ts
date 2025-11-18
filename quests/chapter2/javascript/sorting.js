// Chapter 2: Mountains of Order
// Implement sorting algorithms

function mergeSort(arr) {
  // TODO: Implement merge sort
  // Base case: array of length 0 or 1 is already sorted
  // Recursive case: divide, sort, and merge
  return arr;
}

function quickSort(arr) {
  // TODO: Implement quick sort
  // Base case: array of length 0 or 1 is already sorted
  // Recursive case: partition around pivot and sort subarrays
  return arr;
}

function isSorted(arr) {
  for (let i = 1; i < arr.length; i++) {
    if (arr[i] < arr[i - 1]) return false;
  }
  return true;
}

module.exports = { mergeSort, quickSort, isSorted };

// Reference solutions for Chapter 3

package main

import "sort"

// MaxSumSubarray finds the maximum sum of a subarray of size k using sliding window
func MaxSumSubarray(arr []int, k int) int {
	if len(arr) < k {
		return 0
	}

	maxSum := 0
	windowSum := 0

	// Calculate first window
	for i := 0; i < k; i++ {
		windowSum += arr[i]
	}
	maxSum = windowSum

	// Slide the window
	for i := k; i < len(arr); i++ {
		windowSum = windowSum - arr[i-k] + arr[i]
		if windowSum > maxSum {
			maxSum = windowSum
		}
	}

	return maxSum
}

// TwoSum finds two indices where arr[i] + arr[j] = target
func TwoSum(arr []int, target int) []int {
	left := 0
	right := len(arr) - 1

	for left < right {
		sum := arr[left] + arr[right]
		if sum == target {
			return []int{left, right}
		} else if sum < target {
			left++
		} else {
			right--
		}
	}

	return nil
}

// ThreeSum finds all unique triplets that sum to target
func ThreeSum(arr []int, target int) [][]int {
	result := [][]int{}

	// Sort the array first
	sortedArr := make([]int, len(arr))
	copy(sortedArr, arr)
	sort.Ints(sortedArr)

	for i := 0; i < len(sortedArr)-2; i++ {
		// Skip duplicates
		if i > 0 && sortedArr[i] == sortedArr[i-1] {
			continue
		}

		left := i + 1
		right := len(sortedArr) - 1

		for left < right {
			sum := sortedArr[i] + sortedArr[left] + sortedArr[right]
			if sum == target {
				result = append(result, []int{sortedArr[i], sortedArr[left], sortedArr[right]})

				// Skip duplicates
				for left < right && sortedArr[left] == sortedArr[left+1] {
					left++
				}
				for left < right && sortedArr[right] == sortedArr[right-1] {
					right--
				}
				left++
				right--
			} else if sum < target {
				left++
			} else {
				right--
			}
		}
	}

	return result
}

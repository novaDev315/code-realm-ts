package main

// Reference solutions for Chapter 2

// MergeSort sorts an array using the merge sort algorithm
func MergeSortSolution(arr []int) []int {
	if len(arr) <= 1 {
		return arr
	}

	mid := len(arr) / 2
	left := MergeSortSolution(append([]int{}, arr[:mid]...))
	right := MergeSortSolution(append([]int{}, arr[mid:]...))

	return merge(left, right)
}

func merge(left, right []int) []int {
	result := make([]int, 0, len(left)+len(right))
	i, j := 0, 0

	for i < len(left) && j < len(right) {
		if left[i] <= right[j] {
			result = append(result, left[i])
			i++
		} else {
			result = append(result, right[j])
			j++
		}
	}

	result = append(result, left[i:]...)
	result = append(result, right[j:]...)

	return result
}

// QuickSort sorts an array using the quick sort algorithm
func QuickSortSolution(arr []int) []int {
	if len(arr) <= 1 {
		return arr
	}

	pivot := arr[len(arr)/2]
	left := []int{}
	middle := []int{}
	right := []int{}

	for _, x := range arr {
		if x < pivot {
			left = append(left, x)
		} else if x == pivot {
			middle = append(middle, x)
		} else {
			right = append(right, x)
		}
	}

	result := []int{}
	result = append(result, QuickSortSolution(left)...)
	result = append(result, middle...)
	result = append(result, QuickSortSolution(right)...)

	return result
}

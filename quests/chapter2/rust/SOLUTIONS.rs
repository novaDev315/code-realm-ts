// Reference solutions for Chapter 2

// MergeSort sorts a vector using the merge sort algorithm
pub fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }

    let mid = arr.len() / 2;
    let left = merge_sort(arr[..mid].to_vec());
    let right = merge_sort(arr[mid..].to_vec());

    merge(left, right)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);

    result
}

// QuickSort sorts a vector using the quick sort algorithm
pub fn quick_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = arr[arr.len() / 2];
    let left: Vec<i32> = arr.iter().filter(|&&x| x < pivot).copied().collect();
    let middle: Vec<i32> = arr.iter().filter(|&&x| x == pivot).copied().collect();
    let right: Vec<i32> = arr.iter().filter(|&&x| x > pivot).copied().collect();

    let mut result = quick_sort(left);
    result.extend(middle);
    result.extend(quick_sort(right));

    result
}

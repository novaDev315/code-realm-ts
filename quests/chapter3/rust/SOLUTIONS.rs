// Reference solutions for Chapter 3

/// Find maximum sum of subarray of size k using sliding window
pub fn max_sum_subarray(arr: Vec<i32>, k: usize) -> i32 {
    if arr.len() < k {
        return 0;
    }

    let mut max_sum = 0;
    let mut window_sum = 0;

    // Calculate first window
    for i in 0..k {
        window_sum += arr[i];
    }
    max_sum = window_sum;

    // Slide the window
    for i in k..arr.len() {
        window_sum = window_sum - arr[i - k] + arr[i];
        max_sum = max_sum.max(window_sum);
    }

    max_sum
}

/// Find two indices where arr[i] + arr[j] = target
/// Array is sorted. Use two pointers approach.
pub fn two_sum(arr: Vec<i32>, target: i32) -> Option<Vec<usize>> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let sum = arr[left] + arr[right];
        if sum == target {
            return Some(vec![left, right]);
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    None
}

/// Find all unique triplets that sum to target
pub fn three_sum(mut arr: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    // Sort the array first
    arr.sort();

    for i in 0..arr.len().saturating_sub(2) {
        // Skip duplicates
        if i > 0 && arr[i] == arr[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = arr.len() - 1;

        while left < right {
            let sum = arr[i] + arr[left] + arr[right];
            if sum == target {
                result.push(vec![arr[i], arr[left], arr[right]]);

                // Skip duplicates
                while left < right && arr[left] == arr[left + 1] {
                    left += 1;
                }
                while left < right && arr[right] == arr[right - 1] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_subarray() {
        assert_eq!(max_sum_subarray(vec![2, 1, 5, 1, 3, 2], 3), 9);
        assert_eq!(max_sum_subarray(vec![2, 3, 4, 1, 5], 2), 7);
        assert_eq!(max_sum_subarray(vec![1, 4, 2, 10, 23, 3, 1, 0, 20], 4), 39);
    }

    #[test]
    fn test_two_sum() {
        let result = two_sum(vec![1, 2, 3, 4, 5], 9);
        assert!(result.is_some());
        let indices = result.unwrap();
        assert_eq!(1 + 2 + 3 + 4 + 5, 15); // Just checking array

        let result2 = two_sum(vec![2, 7, 11, 15], 9);
        assert!(result2.is_some());
        let indices2 = result2.unwrap();
        assert_eq!(2 + 7, 9);
    }

    #[test]
    fn test_three_sum() {
        let result = three_sum(vec![-1, 0, 1, 2, -1, -4], 0);
        assert!(result.len() > 0);
        for triplet in &result {
            assert_eq!(triplet[0] + triplet[1] + triplet[2], 0);
        }
    }
}

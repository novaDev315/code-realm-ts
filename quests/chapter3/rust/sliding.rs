// Chapter 3: Mirror Maze
// Sliding Window & Two Pointers

/// Find maximum sum of subarray of size k using sliding window
pub fn max_sum_subarray(arr: Vec<i32>, k: usize) -> i32 {
    // TODO: Find maximum sum of subarray of size k using sliding window
    // Return the maximum sum
    0
}

/// Find two indices where arr[i] + arr[j] = target
/// Array is sorted. Use two pointers approach.
/// Returns Some(vec![i, j]) or None if no solution
pub fn two_sum(arr: Vec<i32>, target: i32) -> Option<Vec<usize>> {
    // TODO: Find two indices where arr[i] + arr[j] = target
    // Array is sorted. Use two pointers approach.
    // Return Some(vec![i, j]) or None if no solution
    None
}

/// Find all unique triplets that sum to target
/// Returns a vector of triplets Vec<Vec<i32>>
pub fn three_sum(arr: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    // TODO: Find all unique triplets that sum to target
    // Return vector of triplets Vec<Vec<i32>>
    Vec::new()
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
        assert_eq!(indices.len(), 2);

        let result2 = two_sum(vec![2, 7, 11, 15], 9);
        assert!(result2.is_some());
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

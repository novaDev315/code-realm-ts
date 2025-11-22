// Test runner for Chapter 3

mod sliding;

use sliding::{max_sum_subarray, two_sum, three_sum};

fn main() {
    let mut passed = true;

    // Test max_sum_subarray
    println!("Testing max_sum_subarray...");
    let max_sum_cases = vec![
        (vec![2, 1, 5, 1, 3, 2], 3, 9),
        (vec![2, 3, 4, 1, 5], 2, 7),
        (vec![1, 4, 2, 10, 23, 3, 1, 0, 20], 4, 39),
    ];

    for (arr, k, expect) in max_sum_cases {
        let result = max_sum_subarray(arr.clone(), k);
        if result != expect {
            eprintln!("❌ max_sum_subarray({:?}, {}) expected {} but got {}", arr, k, expect, result);
            passed = false;
        } else {
            println!("✓ max_sum_subarray k={}", k);
        }
    }

    // Test two_sum
    println!("\nTesting two_sum...");
    let two_sum_cases = vec![
        (vec![1, 2, 3, 4, 5], 9, 9),
        (vec![2, 7, 11, 15], 9, 9),
    ];

    for (arr, target, expect_sum) in two_sum_cases {
        let result = two_sum(arr.clone(), target);
        match result {
            None => {
                eprintln!("❌ two_sum({:?}, {}) returned None", arr, target);
                passed = false;
            }
            Some(indices) => {
                if arr[indices[0]] + arr[indices[1]] != expect_sum {
                    eprintln!("❌ two_sum({:?}, {}) indices don't sum to {}", arr, target, expect_sum);
                    passed = false;
                } else {
                    println!("✓ two_sum target={}", target);
                }
            }
        }
    }

    // Test three_sum
    println!("\nTesting three_sum...");
    let three_sum_result = three_sum(vec![-1, 0, 1, 2, -1, -4], 0);
    if three_sum_result.is_empty() {
        eprintln!("❌ three_sum should find at least one triplet");
        passed = false;
    } else {
        let mut valid_triplets = true;
        for triplet in &three_sum_result {
            if triplet[0] + triplet[1] + triplet[2] != 0 {
                valid_triplets = false;
                break;
            }
        }
        if valid_triplets {
            println!("✓ three_sum found {} valid triplet(s)", three_sum_result.len());
        } else {
            eprintln!("❌ three_sum returned invalid triplets");
            passed = false;
        }
    }

    if passed {
        println!("\n✅ All tests passed!");
    } else {
        eprintln!("\n❌ Some tests failed.");
        std::process::exit(1);
    }
}

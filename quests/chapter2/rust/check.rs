mod sorting;

use sorting::{merge_sort, quick_sort};

struct TestCase {
    input: Vec<i32>,
    expect: Vec<i32>,
    name: &'static str,
}

fn main() {
    let mut passed = true;

    let test_cases = vec![
        TestCase {
            input: vec![5, 2, 8, 1, 9],
            expect: vec![1, 2, 5, 8, 9],
            name: "basic array",
        },
        TestCase {
            input: vec![3, 3, 1, 2],
            expect: vec![1, 2, 3, 3],
            name: "duplicates",
        },
        TestCase {
            input: vec![1],
            expect: vec![1],
            name: "single element",
        },
        TestCase {
            input: vec![],
            expect: vec![],
            name: "empty array",
        },
        TestCase {
            input: vec![5, 4, 3, 2, 1],
            expect: vec![1, 2, 3, 4, 5],
            name: "reverse sorted",
        },
        TestCase {
            input: vec![1, 2, 3, 4, 5],
            expect: vec![1, 2, 3, 4, 5],
            name: "already sorted",
        },
    ];

    println!("Testing MergeSort...");
    for tc in &test_cases {
        let result = merge_sort(tc.input.clone());
        if result != tc.expect {
            eprintln!(
                "❌ merge_sort({:?}) [{}] expected {:?} but got {:?}",
                tc.input, tc.name, tc.expect, result
            );
            passed = false;
        } else {
            println!("✓ merge_sort {}", tc.name);
        }
    }

    println!("\nTesting QuickSort...");
    for tc in &test_cases {
        let result = quick_sort(tc.input.clone());
        if result != tc.expect {
            eprintln!(
                "❌ quick_sort({:?}) [{}] expected {:?} but got {:?}",
                tc.input, tc.name, tc.expect, result
            );
            passed = false;
        } else {
            println!("✓ quick_sort {}", tc.name);
        }
    }

    // Test on large arrays
    println!("\nTesting performance on large array...");
    let large_array: Vec<i32> = (0..1000).map(|i| (i * 7 + 13) % 1000).collect();

    let merge_sorted = merge_sort(large_array.clone());
    if !is_sorted(&merge_sorted) {
        eprintln!("❌ merge_sort failed to sort large array");
        passed = false;
    } else {
        println!("✓ merge_sort handles large arrays");
    }

    let quick_sorted = quick_sort(large_array.clone());
    if !is_sorted(&quick_sorted) {
        eprintln!("❌ quick_sort failed to sort large array");
        passed = false;
    } else {
        println!("✓ quick_sort handles large arrays");
    }

    if passed {
        println!("\n✅ All tests passed!");
    } else {
        eprintln!("\n❌ Some tests failed.");
        std::process::exit(1);
    }
}

fn is_sorted(arr: &[i32]) -> bool {
    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }
    true
}

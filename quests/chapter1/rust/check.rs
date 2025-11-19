use std::collections::HashSet;

// Fibonacci calculates the nth Fibonacci number
fn fibonacci(n: u32) -> u32 {
    // TODO: Implement Fibonacci recursively
    0
}

// Factorial calculates n!
fn factorial(n: u32) -> u32 {
    // TODO: Implement factorial recursively
    1
}

// String permutations generates all unique permutations
fn string_permutations(s: &str) -> Vec<String> {
    // TODO: Implement using recursion
    vec![]
}

fn main() {
    let mut all_passed = true;

    // Test Fibonacci
    println!("Testing fibonacci()...");
    let fib_tests = vec![
        (0, 0),
        (1, 1),
        (2, 1),
        (3, 2),
        (4, 3),
        (5, 5),
        (6, 8),
        (10, 55),
    ];

    for (input, expected) in fib_tests {
        let result = fibonacci(input);
        if result == expected {
            println!("  ✓ fibonacci({}) = {}", input, result);
        } else {
            println!("  ✗ fibonacci({}) = {} (expected {})", input, result, expected);
            all_passed = false;
        }
    }

    // Test Factorial
    println!("\nTesting factorial()...");
    let fact_tests = vec![
        (0, 1),
        (1, 1),
        (2, 2),
        (3, 6),
        (4, 24),
        (5, 120),
        (6, 720),
        (10, 3628800),
    ];

    for (input, expected) in fact_tests {
        let result = factorial(input);
        if result == expected {
            println!("  ✓ factorial({}) = {}", input, result);
        } else {
            println!("  ✗ factorial({}) = {} (expected {})", input, result, expected);
            all_passed = false;
        }
    }

    // Test String Permutations
    println!("\nTesting string_permutations()...");
    let perm_tests = vec![
        ("a", vec!["a"]),
        ("ab", vec!["ab", "ba"]),
        ("abc", vec!["abc", "acb", "bac", "bca", "cab", "cba"]),
    ];

    for (input, mut expected) in perm_tests {
        let mut result = string_permutations(input);
        result.sort();
        expected.sort();

        if result == expected {
            println!("  ✓ string_permutations(\"{}\") = {:?}", input, result);
        } else {
            println!(
                "  ✗ string_permutations(\"{}\") = {:?} (expected {:?})",
                input, result, expected
            );
            all_passed = false;
        }
    }

    if all_passed {
        println!("\n✓ All tests passed!");
        std::process::exit(0);
    } else {
        println!("\n✗ Some tests failed!");
        std::process::exit(1);
    }
}

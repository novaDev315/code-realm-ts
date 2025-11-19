use std::collections::HashSet;

// Fibonacci calculates the nth Fibonacci number
// Time Complexity: O(2^n) - exponential due to repeated subproblems
// Space Complexity: O(n) - recursive call stack depth
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Factorial calculates n!
// Time Complexity: O(n)
// Space Complexity: O(n) - recursive call stack depth
fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

// String permutations generates all unique permutations
// Uses recursion with swap-based approach for efficient generation
// Time Complexity: O(n! * n) - generate all permutations and copy strings
// Space Complexity: O(n! * n) - store all permutations
fn string_permutations(s: &str) -> Vec<String> {
    let mut chars: Vec<char> = s.chars().collect();
    let mut result = Vec::new();
    generate_permutations(&mut chars, 0, &mut result);
    result
}

// Helper function to generate permutations recursively
fn generate_permutations(chars: &mut Vec<char>, start: usize, result: &mut Vec<String>) {
    if start == chars.len() {
        result.push(chars.iter().collect());
        return;
    }

    for i in start..chars.len() {
        // Swap
        chars.swap(start, i);
        // Recurse
        generate_permutations(chars, start + 1, result);
        // Swap back
        chars.swap(start, i);
    }
}

fn main() {
    println!("=== Recursion Solutions ===\n");

    // Fibonacci examples
    println!("Fibonacci Numbers:");
    for i in 0..8 {
        println!("  fibonacci({}) = {}", i, fibonacci(i));
    }

    // Factorial examples
    println!("\nFactorial Numbers:");
    for i in 0..8 {
        println!("  factorial({}) = {}", i, factorial(i));
    }

    // Permutations examples
    println!("\nString Permutations:");
    let test_strings = vec!["a", "ab", "abc"];
    for s in test_strings {
        let perms = string_permutations(s);
        println!("  Permutations of \"{}\": {:?}", s, perms);
    }
}

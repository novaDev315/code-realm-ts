// Test runner for Chapter 11 - URL Shortener Base62 Encoding

mod shortener;

use shortener::{base62_encode, base62_decode, BASE62_CHARS};

fn main() {
    let mut passed = true;

    // Test base62_encode
    println!("Testing base62_encode...");
    let encode_cases: Vec<(u64, &str, &str)> = vec![
        (0, "0", "Zero"),
        (1, "1", "One"),
        (9, "9", "Single digit (9)"),
        (10, "A", "Ten (should be 'A')"),
        (35, "Z", "35 (uppercase Z)"),
        (36, "a", "36 (lowercase a)"),
        (61, "z", "61 (lowercase z)"),
        (62, "10", "62 (should be '10')"),
        (3844, "100", "3844 (62^2)"),
        (238327, "zzz", "238327 (61*62^2 + 61*62 + 61)"),
        (238328, "1000", "238328 (62^3)"),
        (12345678, "pnfq", "Large number"),
    ];

    for (input, expect, name) in &encode_cases {
        let result = base62_encode(*input);
        if result != *expect {
            eprintln!("  X base62_encode({}) - {}: expected \"{}\" but got \"{}\"", input, name, expect, result);
            passed = false;
        } else {
            println!("  OK base62_encode({}) = \"{}\" ({})", input, result, name);
        }
    }

    // Test base62_decode
    println!("\nTesting base62_decode...");
    let decode_cases: Vec<(&str, u64, &str)> = vec![
        ("0", 0, "Zero"),
        ("1", 1, "One"),
        ("9", 9, "Single digit (9)"),
        ("A", 10, "Letter 'A' (uppercase)"),
        ("Z", 35, "Letter 'Z' (uppercase)"),
        ("a", 36, "Letter 'a' (lowercase)"),
        ("z", 61, "Letter 'z' (lowercase)"),
        ("10", 62, "Two chars '10'"),
        ("100", 3844, "62^2"),
        ("zzz", 238327, "61*62^2 + 61*62 + 61"),
        ("1000", 238328, "62^3"),
        ("pnfq", 12345678, "Large number"),
    ];

    for (input, expect, name) in &decode_cases {
        let result = base62_decode(input);
        if result != *expect {
            eprintln!("  X base62_decode(\"{}\") - {}: expected {} but got {}", input, name, expect, result);
            passed = false;
        } else {
            println!("  OK base62_decode(\"{}\") = {} ({})", input, result, name);
        }
    }

    // Test round-trip encoding/decoding
    println!("\nTesting encode/decode round-trip...");
    let round_trip_cases: Vec<u64> = vec![0, 1, 10, 35, 36, 61, 62, 100, 1000, 3844, 10000, 238327, 238328, 12345678, 999999999];

    for num in &round_trip_cases {
        let encoded = base62_encode(*num);
        let decoded = base62_decode(&encoded);
        if decoded != *num {
            eprintln!("  X Round-trip failed for {}: encode -> \"{}\" -> decode -> {}", num, encoded, decoded);
            passed = false;
        } else {
            println!("  OK {} -> \"{}\" -> {}", num, encoded, decoded);
        }
    }

    // Test character set validity
    println!("\nTesting character set validity...");
    let test_nums: Vec<u64> = vec![123, 456789, 9876543210];
    for num in &test_nums {
        let encoded = base62_encode(*num);
        let valid = encoded.chars().all(|c| {
            BASE62_CHARS.contains(&(c as u8))
        });
        if !valid {
            eprintln!("  X base62_encode({}) = \"{}\" contains invalid characters", num, encoded);
            passed = false;
        } else {
            println!("  OK base62_encode({}) = \"{}\" uses valid characters", num, encoded);
        }
    }

    if passed {
        println!("\nAll tests passed!");
    } else {
        eprintln!("\nSome tests failed.");
        std::process::exit(1);
    }
}

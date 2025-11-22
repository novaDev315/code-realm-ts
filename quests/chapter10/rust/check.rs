// Test runner for Chapter 10 - Security (Hashing, JWT, XSS Prevention)

mod security;

use security::{hash_password, verify_password, sanitize_input, validate_jwt_structure, decode_jwt_payload};

fn main() {
    let mut passed = true;

    // Test hash_password
    println!("Testing hash_password...");
    let hash_cases = vec![
        ("password123", "randomsalt", "Basic password"),
        ("", "salt", "Empty password"),
        ("pass", "", "Empty salt"),
        ("MySecureP@ssw0rd!", "pepper123", "Complex password"),
    ];

    for (password, salt, name) in &hash_cases {
        let result = hash_password(password, salt);
        if result.is_empty() {
            eprintln!("  X hash_password({:?}, {:?}) - {}: returned empty string", password, salt, name);
            passed = false;
        } else if result.len() != 64 {
            eprintln!("  X hash_password({:?}, {:?}) - {}: expected 64 char hex, got {} chars", password, salt, name, result.len());
            passed = false;
        } else {
            println!("  OK hash_password({:?}, {:?}) = {}... ({})", password, salt, &result[..16], name);
        }
    }

    // Test hash consistency
    println!("\nTesting hash consistency...");
    let hash1 = hash_password("test", "salt");
    let hash2 = hash_password("test", "salt");
    if hash1 != hash2 {
        eprintln!("  X Same inputs should produce same hash");
        passed = false;
    } else if !hash1.is_empty() {
        println!("  OK Same inputs produce consistent hash");
    }

    // Different inputs should produce different hashes
    let hash3 = hash_password("test", "salt1");
    let hash4 = hash_password("test", "salt2");
    if hash3 == hash4 && !hash3.is_empty() {
        eprintln!("  X Different salts should produce different hashes");
        passed = false;
    } else if !hash3.is_empty() && !hash4.is_empty() {
        println!("  OK Different salts produce different hashes");
    }

    // Test verify_password
    println!("\nTesting verify_password...");
    let correct_hash = hash_password("password123", "salt");
    let verify_cases: Vec<(&str, &str, bool, &str)> = vec![
        ("password123", "salt", true, "Correct password"),
        ("wrongpassword", "salt", false, "Wrong password"),
        ("password123", "wrongsalt", false, "Wrong salt"),
    ];

    for (password, salt, expected, name) in &verify_cases {
        let result = verify_password(password, salt, &correct_hash);
        if result != *expected {
            eprintln!("  X verify_password({:?}, {:?}, hash) - {}: expected {} but got {}", password, salt, name, expected, result);
            passed = false;
        } else {
            println!("  OK verify_password({:?}, {:?}, hash) = {} ({})", password, salt, result, name);
        }
    }

    // Test sanitize_input
    println!("\nTesting sanitize_input...");
    let sanitize_cases: Vec<(&str, Vec<&str>, Vec<&str>, &str)> = vec![
        ("<script>alert('xss')</script>", vec![], vec!["<script>", "</script>", "alert"], "Script tag removal"),
        ("<b>Bold</b>", vec!["Bold"], vec!["<b>", "</b>"], "HTML tag removal"),
        ("Hello <World>", vec!["Hello"], vec!["<", ">"], "Angle bracket removal"),
        ("Say \"Hello\"", vec!["&quot;"], vec!["\""], "Quote escaping"),
        ("It's fine", vec!["&#39;"], vec!["'"], "Apostrophe escaping"),
        ("Normal text", vec!["Normal text"], vec![], "Plain text unchanged"),
    ];

    for (input, should_contain, should_not_contain, name) in &sanitize_cases {
        let result = sanitize_input(input);
        let mut success = true;

        for should in should_contain {
            if !result.contains(should) {
                eprintln!("  X sanitize_input({:?}) - {}: should contain {:?} but got {:?}", input, name, should, result);
                success = false;
                passed = false;
                break;
            }
        }

        if success {
            for should_not in should_not_contain {
                if result.contains(should_not) {
                    eprintln!("  X sanitize_input({:?}) - {}: should NOT contain {:?} but got {:?}", input, name, should_not, result);
                    success = false;
                    passed = false;
                    break;
                }
            }
        }

        if success {
            println!("  OK sanitize_input({:?}) = {:?} ({})", input, result, name);
        }
    }

    // Test validate_jwt_structure
    println!("\nTesting validate_jwt_structure...");
    let valid_jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

    let jwt_cases: Vec<(&str, bool, &str)> = vec![
        (valid_jwt, true, "Valid JWT structure"),
        ("header.payload.signature", true, "Simple 3-part structure"),
        ("only.two", false, "Only 2 parts"),
        ("one", false, "Only 1 part"),
        ("", false, "Empty string"),
        ("a.b.c.d", false, "4 parts (too many)"),
        (".payload.signature", false, "Empty header"),
        ("header..signature", false, "Empty payload"),
        ("header.payload.", false, "Empty signature"),
    ];

    for (token, expected, name) in &jwt_cases {
        let result = validate_jwt_structure(token);
        let display: String = if token.len() > 30 {
            format!("{}...", &token[..30])
        } else {
            token.to_string()
        };

        if result != *expected {
            eprintln!("  X validate_jwt_structure({:?}) - {}: expected {} but got {}", display, name, expected, result);
            passed = false;
        } else {
            println!("  OK validate_jwt_structure({:?}) = {} ({})", display, result, name);
        }
    }

    // Test decode_jwt_payload
    println!("\nTesting decode_jwt_payload...");
    match decode_jwt_payload(valid_jwt) {
        Ok(payload) => {
            // Check for expected fields
            match payload.get("sub") {
                Some(sub) if sub == "1234567890" => {
                    println!("  OK decode_jwt_payload extracted sub={}", sub);
                }
                other => {
                    eprintln!("  X decode_jwt_payload: expected sub=1234567890, got {:?}", other);
                    passed = false;
                }
            }

            match payload.get("name") {
                Some(name) if name == "John Doe" => {
                    println!("  OK decode_jwt_payload extracted name={}", name);
                }
                other => {
                    eprintln!("  X decode_jwt_payload: expected name='John Doe', got {:?}", other);
                    passed = false;
                }
            }
        }
        Err(e) => {
            eprintln!("  X decode_jwt_payload(valid_jwt) returned error: {}", e);
            passed = false;
        }
    }

    // Test invalid JWT decoding
    match decode_jwt_payload("invalid.jwt") {
        Ok(_) => {
            println!("  ? decode_jwt_payload(invalid) returned Ok (check structure validation)");
        }
        Err(_) => {
            println!("  OK decode_jwt_payload(invalid) correctly returned error");
        }
    }

    if passed {
        println!("\nAll tests passed!");
    } else {
        eprintln!("\nSome tests failed.");
        std::process::exit(1);
    }
}

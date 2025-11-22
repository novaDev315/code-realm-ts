// Test runner for Chapter 9 - DevOps Configuration & Health Checks

mod devops;

use devops::{parse_config, health_check, get_env_with_default, validate_port};
use std::collections::HashMap;
use std::env;

fn main() {
    let mut passed = true;

    // Test parse_config
    println!("Testing parse_config...");
    let parse_config_cases: Vec<(&str, &str, i32, bool, &str)> = vec![
        ("host=localhost\nport=8080\ndebug=true", "localhost", 8080, true, "Basic config"),
        ("host=0.0.0.0\nport=3000\ndebug=false", "0.0.0.0", 3000, false, "Production config"),
        ("host=api.example.com\nport=443\ndebug=false", "api.example.com", 443, false, "HTTPS config"),
        ("port=9000\nhost=server.local\ndebug=true", "server.local", 9000, true, "Different order"),
    ];

    for (input, expect_host, expect_port, expect_debug, name) in &parse_config_cases {
        match parse_config(input) {
            Ok(config) => {
                if config.host != *expect_host || config.port != *expect_port || config.debug != *expect_debug {
                    eprintln!("  X parse_config - {}: expected ({}, {}, {}) but got ({}, {}, {})",
                        name, expect_host, expect_port, expect_debug, config.host, config.port, config.debug);
                    passed = false;
                } else {
                    println!("  OK parse_config - {}: {{host: {}, port: {}, debug: {}}}",
                        name, config.host, config.port, config.debug);
                }
            }
            Err(e) => {
                eprintln!("  X parse_config - {}: unexpected error: {}", name, e);
                passed = false;
            }
        }
    }

    // Test health_check
    println!("\nTesting health_check...");

    // All healthy case
    {
        let mut services = HashMap::new();
        services.insert("db".to_string(), true);
        services.insert("cache".to_string(), true);
        let result = health_check(&services);

        let expected_db = "healthy".to_string();
        let expected_cache = "healthy".to_string();
        if result.get("db") != Some(&expected_db) || result.get("cache") != Some(&expected_cache) {
            eprintln!("  X health_check - All healthy: expected {{db: healthy, cache: healthy}} but got {:?}", result);
            passed = false;
        } else {
            println!("  OK health_check - All healthy: {:?}", result);
        }
    }

    // All unhealthy case
    {
        let mut services = HashMap::new();
        services.insert("db".to_string(), false);
        services.insert("cache".to_string(), false);
        let result = health_check(&services);

        let expected_db = "unhealthy".to_string();
        let expected_cache = "unhealthy".to_string();
        if result.get("db") != Some(&expected_db) || result.get("cache") != Some(&expected_cache) {
            eprintln!("  X health_check - All unhealthy: expected {{db: unhealthy, cache: unhealthy}} but got {:?}", result);
            passed = false;
        } else {
            println!("  OK health_check - All unhealthy: {:?}", result);
        }
    }

    // Mixed status case
    {
        let mut services = HashMap::new();
        services.insert("api".to_string(), true);
        services.insert("db".to_string(), false);
        services.insert("cache".to_string(), true);
        let result = health_check(&services);

        let healthy = "healthy".to_string();
        let unhealthy = "unhealthy".to_string();
        if result.get("api") != Some(&healthy) || result.get("db") != Some(&unhealthy) || result.get("cache") != Some(&healthy) {
            eprintln!("  X health_check - Mixed status: expected {{api: healthy, db: unhealthy, cache: healthy}} but got {:?}", result);
            passed = false;
        } else {
            println!("  OK health_check - Mixed status: {:?}", result);
        }
    }

    // Empty services case
    {
        let services: HashMap<String, bool> = HashMap::new();
        let result = health_check(&services);

        if !result.is_empty() {
            eprintln!("  X health_check - Empty services: expected empty map but got {:?}", result);
            passed = false;
        } else {
            println!("  OK health_check - Empty services: {{}}");
        }
    }

    // Test get_env_with_default
    println!("\nTesting get_env_with_default...");

    // Test with unset variable
    let result = get_env_with_default("NONEXISTENT_VAR_12345", "default_value");
    if result != "default_value" {
        eprintln!("  X get_env_with_default - Unset var: expected \"default_value\" but got \"{}\"", result);
        passed = false;
    } else {
        println!("  OK get_env_with_default - Unset var returns default: \"{}\"", result);
    }

    // Test with set variable
    env::set_var("TEST_VAR_CHAPTER9", "custom_value");
    let result = get_env_with_default("TEST_VAR_CHAPTER9", "default_value");
    if result != "custom_value" {
        eprintln!("  X get_env_with_default - Set var: expected \"custom_value\" but got \"{}\"", result);
        passed = false;
    } else {
        println!("  OK get_env_with_default - Set var returns value: \"{}\"", result);
    }
    env::remove_var("TEST_VAR_CHAPTER9");

    // Test with empty string variable
    env::set_var("EMPTY_VAR_CHAPTER9", "");
    let result = get_env_with_default("EMPTY_VAR_CHAPTER9", "fallback");
    if result != "fallback" {
        eprintln!("  X get_env_with_default - Empty var: expected \"fallback\" but got \"{}\"", result);
        passed = false;
    } else {
        println!("  OK get_env_with_default - Empty var returns default: \"{}\"", result);
    }
    env::remove_var("EMPTY_VAR_CHAPTER9");

    // Test validate_port
    println!("\nTesting validate_port...");
    let validate_port_cases: Vec<(i32, bool, &str)> = vec![
        (80, true, "HTTP port"),
        (443, true, "HTTPS port"),
        (8080, true, "Common dev port"),
        (3000, true, "Node.js port"),
        (65535, true, "Max valid port"),
        (1, true, "Min valid port"),
        (0, false, "Zero (reserved)"),
        (-1, false, "Negative port"),
        (65536, false, "Above max port"),
        (100000, false, "Way above max"),
    ];

    for (input, expect, name) in &validate_port_cases {
        let result = validate_port(*input);
        if result != *expect {
            eprintln!("  X validate_port({}) - {}: expected {} but got {}", input, name, expect, result);
            passed = false;
        } else {
            println!("  OK validate_port({}) = {} ({})", input, result, name);
        }
    }

    if passed {
        println!("\nAll tests passed!");
    } else {
        eprintln!("\nSome tests failed.");
        std::process::exit(1);
    }
}

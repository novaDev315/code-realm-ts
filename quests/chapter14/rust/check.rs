// Use SOLUTIONS module for testing the working implementation
// To test the stub, change this to: mod balancer;
#[path = "SOLUTIONS.rs"]
mod SOLUTIONS;

use SOLUTIONS::{CircuitBreaker, CircuitState, LoadBalancer, rate_limit_sliding_window};
use std::time::Duration;

fn run_check() -> bool {
    let mut passed = true;

    // Test 1: Round-Robin Balancer
    println!("\nTesting Round-Robin Balancer...");
    let servers = vec![
        "server1".to_string(),
        "server2".to_string(),
        "server3".to_string(),
    ];
    let mut balancer = LoadBalancer::new(servers.clone());

    let round_robin_tests = vec![
        (0, "First call should return first server"),
        (1, "Second call should return second server"),
        (2, "Third call should return third server"),
        (0, "Fourth call should wrap around to first server"),
        (1, "Fifth call should return second server"),
    ];

    for (expected_index, description) in round_robin_tests {
        let result = balancer.get_next_server();
        let expected_server = &servers[expected_index];

        match result {
            Some(ref server) if server == expected_server => {
                println!("OK {}: \"{}\"", description, server);
            }
            _ => {
                eprintln!(
                    "Round-robin: {}, expected \"{}\", got {:?}",
                    description, expected_server, result
                );
                passed = false;
            }
        }
    }

    // Test adding server
    println!("\nTesting add_server...");
    balancer.add_server("server4".to_string());
    let server_list = balancer.get_servers();
    if server_list.len() != 4 {
        eprintln!("Expected 4 servers after adding, got {}", server_list.len());
        passed = false;
    } else {
        println!("OK add_server works correctly");
    }

    // Test removing server
    println!("\nTesting remove_server...");
    balancer.remove_server("server2");
    let server_list = balancer.get_servers();
    if server_list.len() != 3 {
        eprintln!(
            "Expected 3 servers after removing, got {}",
            server_list.len()
        );
        passed = false;
    } else if server_list.contains(&"server2".to_string()) {
        eprintln!("server2 should have been removed");
        passed = false;
    } else {
        println!("OK remove_server works correctly");
    }

    // Test empty balancer
    println!("\nTesting empty balancer...");
    let mut empty_balancer = LoadBalancer::new(vec![]);
    let empty_result = empty_balancer.get_next_server();
    if empty_result.is_some() {
        eprintln!(
            "Expected None for empty balancer, got {:?}",
            empty_result
        );
        passed = false;
    } else {
        println!("OK Empty balancer returns None");
    }

    // Test 2: Circuit Breaker
    println!("\nTesting Circuit Breaker...");
    let mut breaker = CircuitBreaker::new(3, Duration::from_millis(100));

    // Test initial state
    if *breaker.get_state() != CircuitState::Closed {
        eprintln!(
            "Initial state should be 'CLOSED', got '{}'",
            breaker.get_state_string()
        );
        passed = false;
    } else {
        println!("OK Initial circuit breaker state: CLOSED");
    }

    // Test failures and state transitions
    println!("\nTesting failure accumulation...");
    let failing_op = || -> Result<(), &str> { Err("operation failed") };

    // First failure
    let _ = breaker.call(failing_op);
    if *breaker.get_state() != CircuitState::Closed {
        eprintln!(
            "After 1 failure, state should be 'CLOSED', got '{}'",
            breaker.get_state_string()
        );
        passed = false;
    } else {
        println!("OK After 1 failure: CLOSED");
    }

    // Second failure
    let _ = breaker.call(failing_op);
    if *breaker.get_state() != CircuitState::Closed {
        eprintln!(
            "After 2 failures, state should be 'CLOSED', got '{}'",
            breaker.get_state_string()
        );
        passed = false;
    } else {
        println!("OK After 2 failures: CLOSED");
    }

    // Third failure - should open circuit
    let _ = breaker.call(failing_op);
    if *breaker.get_state() != CircuitState::Open {
        eprintln!(
            "After 3 failures, state should be 'OPEN', got '{}'",
            breaker.get_state_string()
        );
        passed = false;
    } else {
        println!("OK After 3 failures: OPEN");
    }

    // Test that calls are rejected when circuit is open
    println!("\nTesting call rejection when circuit is open...");
    let success_op = || -> Result<(), &str> { Ok(()) };
    let result = breaker.call(success_op);
    if result.is_ok() {
        eprintln!("Expected error when circuit is open");
        passed = false;
    } else {
        println!("OK Calls rejected when circuit is open");
    }

    // Test circuit recovery after timeout
    println!("\nTesting circuit recovery after timeout...");
    std::thread::sleep(Duration::from_millis(150));
    let success_op = || -> Result<(), &str> { Ok(()) };
    let result = breaker.call(success_op);
    if result.is_err() {
        eprintln!("Expected success after timeout, got error: {:?}", result);
        passed = false;
    } else if *breaker.get_state() != CircuitState::Closed {
        eprintln!(
            "State should be CLOSED after successful call, got '{}'",
            breaker.get_state_string()
        );
        passed = false;
    } else {
        println!("OK Circuit recovered after timeout");
    }

    // Test 3: Sliding Window Rate Limiting
    println!("\nTesting Sliding Window Rate Limiting...");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let rate_limit_tests = vec![
        (
            vec![now, now - 100, now - 200],
            1000_i64,
            2_usize,
            true,
            "3 requests in 1000ms window with limit 2 should exceed",
        ),
        (
            vec![now, now - 100],
            1000_i64,
            3_usize,
            false,
            "2 requests in 1000ms window with limit 3 should not exceed",
        ),
        (
            vec![now],
            1000_i64,
            1_usize,
            false,
            "1 request in 1000ms window with limit 1 should not exceed",
        ),
        (
            vec![now, now - 500, now - 1500],
            1000_i64,
            2_usize,
            false,
            "Request outside window should not be counted",
        ),
    ];

    for (requests, window_size, limit, should_exceed, description) in rate_limit_tests {
        let result = rate_limit_sliding_window(&requests, window_size, limit);

        if result != should_exceed {
            eprintln!("{}, expected {}, got {}", description, should_exceed, result);
            passed = false;
        } else {
            println!("OK {}: {}", description, result);
        }
    }

    // Summary
    println!("\n==================================================");
    if passed {
        println!("All load balancing tests passed!");
    } else {
        eprintln!("Some load balancing tests failed.");
        std::process::exit(1);
    }

    passed
}

fn main() {
    run_check();
}

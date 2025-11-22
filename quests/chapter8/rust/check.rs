// Test runner for Chapter 8 - REST & GraphQL API Design

mod api;

use api::{Router, APIResponse};

fn main() {
    let mut passed = true;

    // Test Router::new
    println!("Testing Router::new...");
    let router = Router::new();
    let routes = router.get_routes();
    if !routes.is_empty() {
        eprintln!("  X Router::new: expected empty routes, got {} routes", routes.len());
        passed = false;
    } else {
        println!("  OK Router::new: created empty router");
    }

    // Test add_route and get_routes
    println!("\nTesting add_route and get_routes...");
    let mut router = Router::new();
    router.add_route("GET", "/users", "list_users");
    router.add_route("POST", "/users", "create_user");
    router.add_route("GET", "/users/:id", "get_user");
    router.add_route("DELETE", "/users/:id", "delete_user");

    let routes = router.get_routes();
    if routes.len() != 4 {
        eprintln!("  X get_routes: expected 4 routes but got {}", routes.len());
        passed = false;
    } else {
        println!("  OK add_route/get_routes: registered {} routes", routes.len());
    }

    // Test match_route - found cases
    println!("\nTesting match_route (found cases)...");
    let match_found_cases: Vec<(&str, &str, &str, &str)> = vec![
        ("GET", "/users", "list_users", "GET /users"),
        ("POST", "/users", "create_user", "POST /users"),
        ("GET", "/users/:id", "get_user", "GET /users/:id"),
        ("DELETE", "/users/:id", "delete_user", "DELETE /users/:id"),
    ];

    for (method, path, expected_handler, name) in &match_found_cases {
        match router.match_route(method, path) {
            Some(handler) => {
                if handler != *expected_handler {
                    eprintln!("  X match_route - {}: expected \"{}\" but got \"{}\"", name, expected_handler, handler);
                    passed = false;
                } else {
                    println!("  OK match_route - {} -> {}", name, handler);
                }
            }
            None => {
                eprintln!("  X match_route - {}: route not found", name);
                passed = false;
            }
        }
    }

    // Test match_route - not found cases
    println!("\nTesting match_route (not found cases)...");
    let match_not_found_cases: Vec<(&str, &str, &str)> = vec![
        ("PUT", "/users", "PUT /users (not registered)"),
        ("GET", "/posts", "GET /posts (different path)"),
        ("GET", "/", "GET / (root not registered)"),
    ];

    for (method, path, name) in &match_not_found_cases {
        match router.match_route(method, path) {
            Some(handler) => {
                eprintln!("  X match_route - {}: should not find route, but got \"{}\"", name, handler);
                passed = false;
            }
            None => {
                println!("  OK match_route - {}: correctly not found", name);
            }
        }
    }

    // Test APIResponse::success
    println!("\nTesting APIResponse::success...");
    let success_cases: Vec<(&str, &str)> = vec![
        ("Hello World", "String data"),
        ("", "Empty string"),
        ("{\"id\": 123}", "JSON-like data"),
    ];

    for (data, name) in &success_cases {
        let resp = APIResponse::success(data);
        if resp.status != 200 {
            eprintln!("  X success - {}: expected status 200 but got {}", name, resp.status);
            passed = false;
        } else if !resp.error.is_empty() {
            eprintln!("  X success - {}: expected empty error but got \"{}\"", name, resp.error);
            passed = false;
        } else if resp.data != Some(data.to_string()) {
            eprintln!("  X success - {}: expected data {:?} but got {:?}", name, Some(data.to_string()), resp.data);
            passed = false;
        } else {
            println!("  OK success - {}: status={}", name, resp.status);
        }
    }

    // Test APIResponse::error
    println!("\nTesting APIResponse::error...");
    let error_cases: Vec<(i32, &str, &str)> = vec![
        (400, "Bad request", "Bad Request"),
        (401, "Unauthorized", "Unauthorized"),
        (404, "Not found", "Not Found"),
        (500, "Internal server error", "Server Error"),
    ];

    for (status, message, name) in &error_cases {
        let resp = APIResponse::error(*status, message);
        if resp.status != *status {
            eprintln!("  X error - {}: expected status {} but got {}", name, status, resp.status);
            passed = false;
        } else if resp.error != *message {
            eprintln!("  X error - {}: expected error \"{}\" but got \"{}\"", name, message, resp.error);
            passed = false;
        } else if resp.data.is_some() {
            eprintln!("  X error - {}: expected None data but got {:?}", name, resp.data);
            passed = false;
        } else {
            println!("  OK error - {}: status={}, error=\"{}\"", name, resp.status, resp.error);
        }
    }

    // Test to_json
    println!("\nTesting to_json...");

    // Success response JSON
    let success_resp = APIResponse::success("test data");
    let json_str = success_resp.to_json();
    if json_str.is_empty() {
        eprintln!("  X to_json - Success response: returned empty string");
        passed = false;
    } else if !json_str.contains("200") || !json_str.contains("status") {
        eprintln!("  X to_json - Success response: missing expected fields in \"{}\"", json_str);
        passed = false;
    } else {
        println!("  OK to_json - Success response: {}", json_str);
    }

    // Error response JSON
    let error_resp = APIResponse::error(404, "Resource not found");
    let json_str = error_resp.to_json();
    if json_str.is_empty() {
        eprintln!("  X to_json - Error response: returned empty string");
        passed = false;
    } else if !json_str.contains("404") || !json_str.contains("Resource not found") {
        eprintln!("  X to_json - Error response: missing expected fields in \"{}\"", json_str);
        passed = false;
    } else {
        println!("  OK to_json - Error response: {}", json_str);
    }

    // Test empty router
    println!("\nTesting empty router...");
    let empty_router = Router::new();
    let routes = empty_router.get_routes();
    if !routes.is_empty() {
        eprintln!("  X Empty router: get_routes expected empty but got {} routes", routes.len());
        passed = false;
    } else {
        println!("  OK Empty router: get_routes returns empty slice");
    }

    match empty_router.match_route("GET", "/anything") {
        Some(handler) => {
            eprintln!("  X Empty router: match_route should return None, but got \"{}\"", handler);
            passed = false;
        }
        None => {
            println!("  OK Empty router: match_route correctly returns None");
        }
    }

    if passed {
        println!("\nAll tests passed!");
    } else {
        eprintln!("\nSome tests failed.");
        std::process::exit(1);
    }
}

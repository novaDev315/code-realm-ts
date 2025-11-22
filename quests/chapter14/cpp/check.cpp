// Test file for Chapter 14: Gate of Trials - Load Balancing

#include <iostream>
#include <vector>
#include <string>
#include <chrono>
#include "balancer.cpp"

using namespace std;

int main() {
    bool passed = true;

    // Test 1: Round-Robin Load Balancer
    cout << "Testing Round-Robin Load Balancer..." << endl;
    vector<string> servers = {"server1", "server2", "server3"};
    LoadBalancer balancer(servers);

    string expectedServers[] = {"server1", "server2", "server3", "server1", "server2"};
    string descriptions[] = {
        "First call should return first server",
        "Second call should return second server",
        "Third call should return third server",
        "Fourth call should wrap around to first server",
        "Fifth call should return second server"
    };

    for (int i = 0; i < 5; i++) {
        string result = balancer.getNextServer();
        if (result != expectedServers[i]) {
            cerr << "x Round-robin: " << descriptions[i] << ", expected \""
                 << expectedServers[i] << "\", got \"" << result << "\"" << endl;
            passed = false;
        } else {
            cout << "v " << descriptions[i] << ": \"" << result << "\"" << endl;
        }
    }

    // Test empty server list
    cout << "\nTesting empty server list..." << endl;
    LoadBalancer emptyBalancer(vector<string>());
    string emptyResult = emptyBalancer.getNextServer();
    if (!emptyResult.empty()) {
        cerr << "x Empty balancer should return empty string, got \"" << emptyResult << "\"" << endl;
        passed = false;
    } else {
        cout << "v Empty balancer returns empty string" << endl;
    }

    // Test add/remove servers
    cout << "\nTesting add/remove servers..." << endl;
    LoadBalancer dynamicBalancer(vector<string>{"server1"});
    dynamicBalancer.addServer("server2");
    if (dynamicBalancer.getServerCount() != 2) {
        cerr << "x Expected 2 servers after add, got " << dynamicBalancer.getServerCount() << endl;
        passed = false;
    } else {
        cout << "v addServer works correctly" << endl;
    }

    dynamicBalancer.removeServer("server1");
    if (dynamicBalancer.getServerCount() != 1) {
        cerr << "x Expected 1 server after remove, got " << dynamicBalancer.getServerCount() << endl;
        passed = false;
    } else {
        cout << "v removeServer works correctly" << endl;
    }

    // Test 2: Circuit Breaker
    cout << "\n" << string(50, '=') << endl;
    cout << "Testing Circuit Breaker..." << endl;

    CircuitBreaker breaker(3);

    // Test initial state
    if (breaker.getState() != CircuitState::CLOSED) {
        cerr << "x Initial state should be CLOSED, got " << breaker.getStateString() << endl;
        passed = false;
    } else {
        cout << "v Initial circuit breaker state: CLOSED" << endl;
    }

    // Test failures and state transitions
    cout << "\nTesting failure counting and state transitions..." << endl;

    // Simulate failures
    breaker.call([]() { return false; }); // Failure 1
    if (breaker.getState() != CircuitState::CLOSED) {
        cerr << "x After 1 failure, state should be CLOSED, got " << breaker.getStateString() << endl;
        passed = false;
    } else {
        cout << "v After 1 failure: state is CLOSED" << endl;
    }

    breaker.call([]() { return false; }); // Failure 2
    if (breaker.getState() != CircuitState::CLOSED) {
        cerr << "x After 2 failures, state should be CLOSED, got " << breaker.getStateString() << endl;
        passed = false;
    } else {
        cout << "v After 2 failures: state is CLOSED" << endl;
    }

    breaker.call([]() { return false; }); // Failure 3 - should trip
    if (breaker.getState() != CircuitState::OPEN) {
        cerr << "x After 3 failures, state should be OPEN, got " << breaker.getStateString() << endl;
        passed = false;
    } else {
        cout << "v After 3 failures: state is OPEN" << endl;
    }

    // Test that calls are rejected when open
    bool rejectedResult = breaker.call([]() { return true; });
    if (rejectedResult) {
        cerr << "x Circuit is OPEN, call should return false" << endl;
        passed = false;
    } else {
        cout << "v Calls are rejected when circuit is OPEN" << endl;
    }

    // Test reset
    cout << "\nTesting circuit reset..." << endl;
    breaker.reset();
    if (breaker.getState() != CircuitState::CLOSED) {
        cerr << "x After reset, state should be CLOSED, got " << breaker.getStateString() << endl;
        passed = false;
    } else {
        cout << "v Reset restores CLOSED state" << endl;
    }

    // Test success resets failure count
    cout << "\nTesting success resets failure count..." << endl;
    CircuitBreaker breaker2(3);
    breaker2.call([]() { return false; }); // Failure 1
    breaker2.call([]() { return false; }); // Failure 2
    breaker2.call([]() { return true; });  // Success - should reset count
    breaker2.call([]() { return false; }); // Failure 1 (reset)
    breaker2.call([]() { return false; }); // Failure 2

    if (breaker2.getState() != CircuitState::CLOSED) {
        cerr << "x After success reset, should still be CLOSED" << endl;
        passed = false;
    } else {
        cout << "v Success resets failure count" << endl;
    }

    // Test 3: Rate Limiting
    cout << "\n" << string(50, '=') << endl;
    cout << "Testing Sliding Window Rate Limiting..." << endl;

    auto now = chrono::duration_cast<chrono::milliseconds>(
        chrono::system_clock::now().time_since_epoch()
    ).count();

    // Test: 3 requests in window with limit 2 should exceed
    vector<long> requests1 = {now, now - 100, now - 200};
    bool exceeded1 = RateLimiter::isRateLimitExceeded(requests1, 1000, 2);
    if (!exceeded1) {
        cerr << "x 3 requests in 1000ms window with limit 2 should exceed" << endl;
        passed = false;
    } else {
        cout << "v 3 requests in 1000ms window with limit 2: exceeded" << endl;
    }

    // Test: 2 requests in window with limit 3 should not exceed
    vector<long> requests2 = {now, now - 100};
    bool exceeded2 = RateLimiter::isRateLimitExceeded(requests2, 1000, 3);
    if (exceeded2) {
        cerr << "x 2 requests in 1000ms window with limit 3 should not exceed" << endl;
        passed = false;
    } else {
        cout << "v 2 requests in 1000ms window with limit 3: not exceeded" << endl;
    }

    // Test: Request outside window should not be counted
    vector<long> requests3 = {now, now - 500, now - 1500};
    bool exceeded3 = RateLimiter::isRateLimitExceeded(requests3, 1000, 2);
    if (exceeded3) {
        cerr << "x Request outside window should not be counted" << endl;
        passed = false;
    } else {
        cout << "v Request outside window is not counted" << endl;
    }

    // Summary
    cout << "\n" << string(50, '=') << endl;
    if (passed) {
        cout << "All load balancing tests passed!" << endl;
        return 0;
    } else {
        cout << "Some load balancing tests failed." << endl;
        return 1;
    }
}

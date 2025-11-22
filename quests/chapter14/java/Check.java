// Test file for Chapter 14: Gate of Trials - Load Balancing

import java.util.*;

public class Check {
    public static void main(String[] args) {
        boolean passed = true;

        // Test 1: Round-Robin Load Balancer
        System.out.println("Testing Round-Robin Load Balancer...");
        List<String> servers = Arrays.asList("server1", "server2", "server3");
        LoadBalancer balancer = new LoadBalancer(servers);

        String[] expectedServers = {"server1", "server2", "server3", "server1", "server2"};
        String[] descriptions = {
            "First call should return first server",
            "Second call should return second server",
            "Third call should return third server",
            "Fourth call should wrap around to first server",
            "Fifth call should return second server"
        };

        for (int i = 0; i < expectedServers.length; i++) {
            String result = balancer.getNextServer();
            if (!expectedServers[i].equals(result)) {
                System.err.println("x Round-robin: " + descriptions[i] + ", expected \"" + expectedServers[i] + "\", got \"" + result + "\"");
                passed = false;
            } else {
                System.out.println("v " + descriptions[i] + ": \"" + result + "\"");
            }
        }

        // Test empty server list
        System.out.println("\nTesting empty server list...");
        LoadBalancer emptyBalancer = new LoadBalancer(new ArrayList<>());
        String emptyResult = emptyBalancer.getNextServer();
        if (emptyResult != null) {
            System.err.println("x Empty balancer should return null, got \"" + emptyResult + "\"");
            passed = false;
        } else {
            System.out.println("v Empty balancer returns null");
        }

        // Test add/remove servers
        System.out.println("\nTesting add/remove servers...");
        LoadBalancer dynamicBalancer = new LoadBalancer(Arrays.asList("server1"));
        dynamicBalancer.addServer("server2");
        if (dynamicBalancer.getServerCount() != 2) {
            System.err.println("x Expected 2 servers after add, got " + dynamicBalancer.getServerCount());
            passed = false;
        } else {
            System.out.println("v addServer works correctly");
        }

        dynamicBalancer.removeServer("server1");
        if (dynamicBalancer.getServerCount() != 1) {
            System.err.println("x Expected 1 server after remove, got " + dynamicBalancer.getServerCount());
            passed = false;
        } else {
            System.out.println("v removeServer works correctly");
        }

        // Test 2: Circuit Breaker
        System.out.println("\n" + "=".repeat(50));
        System.out.println("Testing Circuit Breaker...");

        CircuitBreaker breaker = new CircuitBreaker(3);

        // Test initial state
        if (breaker.getState() != CircuitBreaker.State.CLOSED) {
            System.err.println("x Initial state should be CLOSED, got " + breaker.getState());
            passed = false;
        } else {
            System.out.println("v Initial circuit breaker state: CLOSED");
        }

        // Test failures and state transitions
        System.out.println("\nTesting failure counting and state transitions...");

        // Simulate failures
        breaker.call(() -> false); // Failure 1
        if (breaker.getState() != CircuitBreaker.State.CLOSED) {
            System.err.println("x After 1 failure, state should be CLOSED, got " + breaker.getState());
            passed = false;
        } else {
            System.out.println("v After 1 failure: state is CLOSED");
        }

        breaker.call(() -> false); // Failure 2
        if (breaker.getState() != CircuitBreaker.State.CLOSED) {
            System.err.println("x After 2 failures, state should be CLOSED, got " + breaker.getState());
            passed = false;
        } else {
            System.out.println("v After 2 failures: state is CLOSED");
        }

        breaker.call(() -> false); // Failure 3 - should trip
        if (breaker.getState() != CircuitBreaker.State.OPEN) {
            System.err.println("x After 3 failures, state should be OPEN, got " + breaker.getState());
            passed = false;
        } else {
            System.out.println("v After 3 failures: state is OPEN");
        }

        // Test that calls are rejected when open
        boolean rejectedResult = breaker.call(() -> true);
        if (rejectedResult) {
            System.err.println("x Circuit is OPEN, call should return false");
            passed = false;
        } else {
            System.out.println("v Calls are rejected when circuit is OPEN");
        }

        // Test reset
        System.out.println("\nTesting circuit reset...");
        breaker.reset();
        if (breaker.getState() != CircuitBreaker.State.CLOSED) {
            System.err.println("x After reset, state should be CLOSED, got " + breaker.getState());
            passed = false;
        } else {
            System.out.println("v Reset restores CLOSED state");
        }

        // Test success resets failure count
        System.out.println("\nTesting success resets failure count...");
        CircuitBreaker breaker2 = new CircuitBreaker(3);
        breaker2.call(() -> false); // Failure 1
        breaker2.call(() -> false); // Failure 2
        breaker2.call(() -> true);  // Success - should reset count
        breaker2.call(() -> false); // Failure 1 (reset)
        breaker2.call(() -> false); // Failure 2

        if (breaker2.getState() != CircuitBreaker.State.CLOSED) {
            System.err.println("x After success reset, should still be CLOSED");
            passed = false;
        } else {
            System.out.println("v Success resets failure count");
        }

        // Test 3: Rate Limiting
        System.out.println("\n" + "=".repeat(50));
        System.out.println("Testing Sliding Window Rate Limiting...");

        long now = System.currentTimeMillis();

        // Test: 3 requests in window with limit 2 should exceed
        long[] requests1 = {now, now - 100, now - 200};
        boolean exceeded1 = RateLimiter.isRateLimitExceeded(requests1, 1000, 2);
        if (!exceeded1) {
            System.err.println("x 3 requests in 1000ms window with limit 2 should exceed");
            passed = false;
        } else {
            System.out.println("v 3 requests in 1000ms window with limit 2: exceeded");
        }

        // Test: 2 requests in window with limit 3 should not exceed
        long[] requests2 = {now, now - 100};
        boolean exceeded2 = RateLimiter.isRateLimitExceeded(requests2, 1000, 3);
        if (exceeded2) {
            System.err.println("x 2 requests in 1000ms window with limit 3 should not exceed");
            passed = false;
        } else {
            System.out.println("v 2 requests in 1000ms window with limit 3: not exceeded");
        }

        // Test: Request outside window should not be counted
        long[] requests3 = {now, now - 500, now - 1500};
        boolean exceeded3 = RateLimiter.isRateLimitExceeded(requests3, 1000, 2);
        if (exceeded3) {
            System.err.println("x Request outside window should not be counted");
            passed = false;
        } else {
            System.out.println("v Request outside window is not counted");
        }

        // Summary
        System.out.println("\n" + "=".repeat(50));
        if (passed) {
            System.out.println("All load balancing tests passed!");
        } else {
            System.out.println("Some load balancing tests failed.");
            System.exit(1);
        }
    }
}

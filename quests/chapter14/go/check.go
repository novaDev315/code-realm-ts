package main

import (
	"errors"
	"fmt"
	"os"
	"time"
)

func runCheck() bool {
	passed := true

	// Test 1: Round-Robin Balancer
	fmt.Println("\nTesting Round-Robin Balancer...")
	servers := []string{"server1", "server2", "server3"}
	balancer := NewLoadBalancer(servers)

	roundRobinTests := []struct {
		expectedIndex int
		description   string
	}{
		{0, "First call should return first server"},
		{1, "Second call should return second server"},
		{2, "Third call should return third server"},
		{0, "Fourth call should wrap around to first server"},
		{1, "Fifth call should return second server"},
	}

	for _, test := range roundRobinTests {
		result := balancer.GetNextServer()
		expectedServer := servers[test.expectedIndex]

		if result != expectedServer {
			fmt.Printf("Round-robin: %s, expected \"%s\", got \"%s\"\n",
				test.description, expectedServer, result)
			passed = false
		} else {
			fmt.Printf("OK %s: \"%s\"\n", test.description, result)
		}
	}

	// Test adding server
	fmt.Println("\nTesting AddServer...")
	balancer.AddServer("server4")
	serverList := balancer.GetServers()
	if len(serverList) != 4 {
		fmt.Printf("Expected 4 servers after adding, got %d\n", len(serverList))
		passed = false
	} else {
		fmt.Println("OK AddServer works correctly")
	}

	// Test removing server
	fmt.Println("\nTesting RemoveServer...")
	balancer.RemoveServer("server2")
	serverList = balancer.GetServers()
	if len(serverList) != 3 {
		fmt.Printf("Expected 3 servers after removing, got %d\n", len(serverList))
		passed = false
	} else {
		found := false
		for _, s := range serverList {
			if s == "server2" {
				found = true
				break
			}
		}
		if found {
			fmt.Println("server2 should have been removed")
			passed = false
		} else {
			fmt.Println("OK RemoveServer works correctly")
		}
	}

	// Test empty balancer
	fmt.Println("\nTesting empty balancer...")
	emptyBalancer := NewLoadBalancer([]string{})
	emptyResult := emptyBalancer.GetNextServer()
	if emptyResult != "" {
		fmt.Printf("Expected empty string for empty balancer, got \"%s\"\n", emptyResult)
		passed = false
	} else {
		fmt.Println("OK Empty balancer returns empty string")
	}

	// Test 2: Circuit Breaker
	fmt.Println("\nTesting Circuit Breaker...")
	breaker := NewCircuitBreaker(3, 100*time.Millisecond)

	// Test initial state
	if breaker.GetState() != StateClosed {
		fmt.Printf("Initial state should be 'CLOSED', got '%s'\n", breaker.GetState())
		passed = false
	} else {
		fmt.Println("OK Initial circuit breaker state: CLOSED")
	}

	// Test failures and state transitions
	fmt.Println("\nTesting failure accumulation...")
	failingOp := func() error {
		return errors.New("operation failed")
	}

	// First failure
	breaker.Call(failingOp)
	if breaker.GetState() != StateClosed {
		fmt.Printf("After 1 failure, state should be 'CLOSED', got '%s'\n", breaker.GetState())
		passed = false
	} else {
		fmt.Println("OK After 1 failure: CLOSED")
	}

	// Second failure
	breaker.Call(failingOp)
	if breaker.GetState() != StateClosed {
		fmt.Printf("After 2 failures, state should be 'CLOSED', got '%s'\n", breaker.GetState())
		passed = false
	} else {
		fmt.Println("OK After 2 failures: CLOSED")
	}

	// Third failure - should open circuit
	breaker.Call(failingOp)
	if breaker.GetState() != StateOpen {
		fmt.Printf("After 3 failures, state should be 'OPEN', got '%s'\n", breaker.GetState())
		passed = false
	} else {
		fmt.Println("OK After 3 failures: OPEN")
	}

	// Test that calls are rejected when circuit is open
	fmt.Println("\nTesting call rejection when circuit is open...")
	successOp := func() error {
		return nil
	}
	err := breaker.Call(successOp)
	if err == nil {
		fmt.Println("Expected error when circuit is open")
		passed = false
	} else {
		fmt.Println("OK Calls rejected when circuit is open")
	}

	// Test circuit recovery after timeout
	fmt.Println("\nTesting circuit recovery after timeout...")
	time.Sleep(150 * time.Millisecond)
	err = breaker.Call(successOp)
	if err != nil {
		fmt.Printf("Expected success after timeout, got error: %v\n", err)
		passed = false
	} else if breaker.GetState() != StateClosed {
		fmt.Printf("State should be CLOSED after successful call, got '%s'\n", breaker.GetState())
		passed = false
	} else {
		fmt.Println("OK Circuit recovered after timeout")
	}

	// Test 3: Sliding Window Rate Limiting
	fmt.Println("\nTesting Sliding Window Rate Limiting...")
	now := time.Now().UnixMilli()

	rateLimitTests := []struct {
		requests     []int64
		windowSize   int64
		limit        int
		shouldExceed bool
		description  string
	}{
		{
			requests:     []int64{now, now - 100, now - 200},
			windowSize:   1000,
			limit:        2,
			shouldExceed: true,
			description:  "3 requests in 1000ms window with limit 2 should exceed",
		},
		{
			requests:     []int64{now, now - 100},
			windowSize:   1000,
			limit:        3,
			shouldExceed: false,
			description:  "2 requests in 1000ms window with limit 3 should not exceed",
		},
		{
			requests:     []int64{now},
			windowSize:   1000,
			limit:        1,
			shouldExceed: false,
			description:  "1 request in 1000ms window with limit 1 should not exceed",
		},
		{
			requests:     []int64{now, now - 500, now - 1500},
			windowSize:   1000,
			limit:        2,
			shouldExceed: false,
			description:  "Request outside window should not be counted",
		},
	}

	for _, test := range rateLimitTests {
		result := RateLimitSlidingWindow(test.requests, test.windowSize, test.limit)

		if result != test.shouldExceed {
			fmt.Printf("%s, expected %v, got %v\n", test.description, test.shouldExceed, result)
			passed = false
		} else {
			fmt.Printf("OK %s: %v\n", test.description, result)
		}
	}

	// Summary
	fmt.Println("\n" + "==================================================")
	if passed {
		fmt.Println("All load balancing tests passed!")
	} else {
		fmt.Println("Some load balancing tests failed.")
		os.Exit(1)
	}

	return passed
}

func main() {
	runCheck()
}

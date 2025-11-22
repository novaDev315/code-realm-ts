// Reference solutions for Chapter 14: Gate of Trials - Load Balancing
// These are working implementations for testing purposes

package main

import (
	"errors"
	"time"
)

// Circuit breaker states
const (
	StateClosed   = "CLOSED"
	StateOpen     = "OPEN"
	StateHalfOpen = "HALF_OPEN"
)

// LoadBalancer implements round-robin load balancing across servers
type LoadBalancer struct {
	servers      []string
	currentIndex int
}

// NewLoadBalancer creates a new load balancer with the given servers
func NewLoadBalancer(servers []string) *LoadBalancer {
	// Make a copy to avoid external modifications
	serversCopy := make([]string, len(servers))
	copy(serversCopy, servers)
	return &LoadBalancer{
		servers:      serversCopy,
		currentIndex: 0,
	}
}

// GetNextServer returns the next server in round-robin fashion
func (lb *LoadBalancer) GetNextServer() string {
	// If no servers, return empty string
	if len(lb.servers) == 0 {
		return ""
	}

	// Get server at current index
	server := lb.servers[lb.currentIndex]

	// Increment index and wrap around
	lb.currentIndex = (lb.currentIndex + 1) % len(lb.servers)

	return server
}

// AddServer adds a new server to the pool
func (lb *LoadBalancer) AddServer(server string) {
	lb.servers = append(lb.servers, server)
}

// RemoveServer removes a server from the pool
func (lb *LoadBalancer) RemoveServer(server string) {
	for i, s := range lb.servers {
		if s == server {
			// Remove server at index i
			lb.servers = append(lb.servers[:i], lb.servers[i+1:]...)

			// Adjust currentIndex if needed
			if lb.currentIndex >= len(lb.servers) && len(lb.servers) > 0 {
				lb.currentIndex = 0
			}
			return
		}
	}
}

// GetServers returns the current list of servers
func (lb *LoadBalancer) GetServers() []string {
	return lb.servers
}

// CircuitBreaker implements the circuit breaker pattern to prevent cascading failures
type CircuitBreaker struct {
	state        string
	failureCount int
	threshold    int
	timeout      time.Duration
	lastFailure  time.Time
}

// NewCircuitBreaker creates a new circuit breaker with the given threshold and timeout
func NewCircuitBreaker(threshold int, timeout time.Duration) *CircuitBreaker {
	return &CircuitBreaker{
		state:        StateClosed,
		failureCount: 0,
		threshold:    threshold,
		timeout:      timeout,
	}
}

// Call executes the operation with circuit breaker protection
func (cb *CircuitBreaker) Call(operation func() error) error {
	// Check if circuit is open
	if cb.state == StateOpen {
		// Check if timeout has elapsed
		if time.Since(cb.lastFailure) >= cb.timeout {
			// Transition to half-open state
			cb.state = StateHalfOpen
		} else {
			// Circuit is still open, reject the call
			return errors.New("circuit breaker is open")
		}
	}

	// Execute the operation
	err := operation()

	if err != nil {
		// Operation failed
		cb.failureCount++
		cb.lastFailure = time.Now()

		// Check if threshold exceeded
		if cb.failureCount >= cb.threshold {
			cb.state = StateOpen
		}

		return err
	}

	// Operation succeeded - reset failure count and close circuit
	cb.failureCount = 0
	cb.state = StateClosed

	return nil
}

// GetState returns the current state of the circuit breaker
func (cb *CircuitBreaker) GetState() string {
	return cb.state
}

// Reset resets the circuit breaker to its initial state
func (cb *CircuitBreaker) Reset() {
	cb.state = StateClosed
	cb.failureCount = 0
}

// RateLimitSlidingWindow checks if the request rate exceeds the limit
// within the sliding window
func RateLimitSlidingWindow(requests []int64, windowSize int64, limit int) bool {
	// If no requests, rate is not exceeded
	if len(requests) == 0 {
		return false
	}

	// Find the maximum timestamp as "now" (most recent request)
	now := requests[0]
	for _, timestamp := range requests {
		if timestamp > now {
			now = timestamp
		}
	}
	windowStart := now - windowSize

	// Count requests within the sliding window
	count := 0
	for _, timestamp := range requests {
		if timestamp > windowStart {
			count++
		}
	}

	// Return true if limit is exceeded
	return count > limit
}

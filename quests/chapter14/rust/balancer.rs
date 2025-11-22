// Chapter 14: Gate of Trials - Load Balancing

use std::time::{Duration, Instant};

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl std::fmt::Display for CircuitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitState::Closed => write!(f, "CLOSED"),
            CircuitState::Open => write!(f, "OPEN"),
            CircuitState::HalfOpen => write!(f, "HALF_OPEN"),
        }
    }
}

/// LoadBalancer implements round-robin load balancing across servers
pub struct LoadBalancer {
    servers: Vec<String>,
    current_index: usize,
}

impl LoadBalancer {
    /// Creates a new load balancer with the given servers
    pub fn new(servers: Vec<String>) -> Self {
        LoadBalancer {
            servers,
            current_index: 0,
        }
    }

    /// Returns the next server in round-robin fashion
    pub fn get_next_server(&mut self) -> Option<String> {
        // TODO: Implement round-robin selection
        // 1. If no servers, return None
        // 2. Get server at current_index
        // 3. Increment index and wrap around using modulo
        // 4. Return the selected server
        None
    }

    /// Adds a new server to the pool
    pub fn add_server(&mut self, server: String) {
        // TODO: Add the server to the servers Vec
    }

    /// Removes a server from the pool
    pub fn remove_server(&mut self, server: &str) {
        // TODO: Remove the server from the servers Vec
        // Hint: Find the server index and remove it
        // Be careful to adjust current_index if needed
    }

    /// Returns the current list of servers
    pub fn get_servers(&self) -> &[String] {
        // TODO: Return a reference to the servers slice
        &[]
    }
}

/// CircuitBreaker implements the circuit breaker pattern to prevent cascading failures
pub struct CircuitBreaker {
    state: CircuitState,
    failure_count: u32,
    threshold: u32,
    timeout: Duration,
    last_failure: Option<Instant>,
}

impl CircuitBreaker {
    /// Creates a new circuit breaker with the given threshold and timeout
    pub fn new(threshold: u32, timeout: Duration) -> Self {
        CircuitBreaker {
            state: CircuitState::Closed,
            failure_count: 0,
            threshold,
            timeout,
            last_failure: None,
        }
    }

    /// Executes the operation with circuit breaker protection
    pub fn call<F, E>(&mut self, operation: F) -> Result<(), E>
    where
        F: FnOnce() -> Result<(), E>,
        E: std::fmt::Debug,
    {
        // TODO: Implement circuit breaker logic
        // 1. If state is Open:
        //    - Check if timeout has elapsed since last failure
        //    - If yes, transition to HalfOpen state
        //    - If no, return error immediately (circuit is open)
        // 2. Execute the operation
        // 3. If operation succeeds:
        //    - Reset failure count
        //    - Set state to Closed
        // 4. If operation fails:
        //    - Increment failure count
        //    - Record last failure time
        //    - If failure count >= threshold, set state to Open
        //    - Return the error
        Err(operation().unwrap_err())
    }

    /// Returns the current state of the circuit breaker
    pub fn get_state(&self) -> &CircuitState {
        // TODO: Return reference to current state
        &self.state
    }

    /// Returns the current state as a string
    pub fn get_state_string(&self) -> String {
        // TODO: Return the state as a string
        String::new()
    }

    /// Resets the circuit breaker to its initial state
    pub fn reset(&mut self) {
        // TODO: Reset state to Closed, failure_count to 0
    }
}

/// Checks if the request rate exceeds the limit within the sliding window
/// requests: array of timestamps (in milliseconds)
/// window_size: size of the window in milliseconds
/// limit: maximum number of requests allowed
/// Returns true if limit is exceeded
pub fn rate_limit_sliding_window(requests: &[i64], window_size: i64, limit: usize) -> bool {
    // TODO: Implement sliding window rate limiting
    // 1. If no requests, return false
    // 2. Use the latest request timestamp as "now"
    // 3. Count requests within the window (now - window_size, now]
    // 4. Return true if count > limit (rate exceeded)
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_balancer_basic() {
        let servers = vec![
            "server1".to_string(),
            "server2".to_string(),
            "server3".to_string(),
        ];
        let mut lb = LoadBalancer::new(servers);
        assert_eq!(lb.get_next_server(), Some("server1".to_string()));
        assert_eq!(lb.get_next_server(), Some("server2".to_string()));
    }

    #[test]
    fn test_circuit_breaker_initial() {
        let cb = CircuitBreaker::new(3, Duration::from_millis(100));
        assert_eq!(*cb.get_state(), CircuitState::Closed);
    }
}

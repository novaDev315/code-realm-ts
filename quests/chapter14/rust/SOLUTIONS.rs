// Reference solutions for Chapter 14: Gate of Trials - Load Balancing
// These are working implementations for testing purposes

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
        // If no servers, return None
        if self.servers.is_empty() {
            return None;
        }

        // Get server at current index
        let server = self.servers[self.current_index].clone();

        // Increment index and wrap around
        self.current_index = (self.current_index + 1) % self.servers.len();

        Some(server)
    }

    /// Adds a new server to the pool
    pub fn add_server(&mut self, server: String) {
        self.servers.push(server);
    }

    /// Removes a server from the pool
    pub fn remove_server(&mut self, server: &str) {
        if let Some(pos) = self.servers.iter().position(|s| s == server) {
            self.servers.remove(pos);

            // Adjust current_index if needed
            if !self.servers.is_empty() && self.current_index >= self.servers.len() {
                self.current_index = 0;
            }
        }
    }

    /// Returns the current list of servers
    pub fn get_servers(&self) -> &[String] {
        &self.servers
    }
}

/// Custom error type for circuit breaker
#[derive(Debug)]
pub enum CircuitBreakerError<E> {
    CircuitOpen,
    OperationError(E),
}

impl<E: std::fmt::Display> std::fmt::Display for CircuitBreakerError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitBreakerError::CircuitOpen => write!(f, "circuit breaker is open"),
            CircuitBreakerError::OperationError(e) => write!(f, "{}", e),
        }
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
    pub fn call<F, E>(&mut self, operation: F) -> Result<(), CircuitBreakerError<E>>
    where
        F: FnOnce() -> Result<(), E>,
        E: std::fmt::Debug,
    {
        // Check if circuit is open
        if self.state == CircuitState::Open {
            // Check if timeout has elapsed
            if let Some(last_failure) = self.last_failure {
                if last_failure.elapsed() >= self.timeout {
                    // Transition to half-open state
                    self.state = CircuitState::HalfOpen;
                } else {
                    // Circuit is still open, reject the call
                    return Err(CircuitBreakerError::CircuitOpen);
                }
            }
        }

        // Execute the operation
        match operation() {
            Ok(()) => {
                // Operation succeeded - reset failure count and close circuit
                self.failure_count = 0;
                self.state = CircuitState::Closed;
                Ok(())
            }
            Err(e) => {
                // Operation failed
                self.failure_count += 1;
                self.last_failure = Some(Instant::now());

                // Check if threshold exceeded
                if self.failure_count >= self.threshold {
                    self.state = CircuitState::Open;
                }

                Err(CircuitBreakerError::OperationError(e))
            }
        }
    }

    /// Returns the current state of the circuit breaker
    pub fn get_state(&self) -> &CircuitState {
        &self.state
    }

    /// Returns the current state as a string
    pub fn get_state_string(&self) -> String {
        self.state.to_string()
    }

    /// Resets the circuit breaker to its initial state
    pub fn reset(&mut self) {
        self.state = CircuitState::Closed;
        self.failure_count = 0;
        self.last_failure = None;
    }
}

/// Checks if the request rate exceeds the limit within the sliding window
/// requests: array of timestamps (in milliseconds)
/// window_size: size of the window in milliseconds
/// limit: maximum number of requests allowed
/// Returns true if limit is exceeded
pub fn rate_limit_sliding_window(requests: &[i64], window_size: i64, limit: usize) -> bool {
    // If no requests, rate is not exceeded
    if requests.is_empty() {
        return false;
    }

    // Find the maximum timestamp as "now" (most recent request)
    let now = *requests.iter().max().unwrap();
    let window_start = now - window_size;

    // Count requests within the sliding window
    let count = requests.iter().filter(|&&ts| ts > window_start).count();

    // Return true if limit is exceeded
    count > limit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_balancer_round_robin() {
        let servers = vec![
            "server1".to_string(),
            "server2".to_string(),
            "server3".to_string(),
        ];
        let mut lb = LoadBalancer::new(servers);

        assert_eq!(lb.get_next_server(), Some("server1".to_string()));
        assert_eq!(lb.get_next_server(), Some("server2".to_string()));
        assert_eq!(lb.get_next_server(), Some("server3".to_string()));
        assert_eq!(lb.get_next_server(), Some("server1".to_string()));
    }

    #[test]
    fn test_load_balancer_add_remove() {
        let mut lb = LoadBalancer::new(vec!["server1".to_string()]);

        lb.add_server("server2".to_string());
        assert_eq!(lb.get_servers().len(), 2);

        lb.remove_server("server1");
        assert_eq!(lb.get_servers().len(), 1);
        assert_eq!(lb.get_servers()[0], "server2");
    }

    #[test]
    fn test_load_balancer_empty() {
        let mut lb = LoadBalancer::new(vec![]);
        assert_eq!(lb.get_next_server(), None);
    }

    #[test]
    fn test_circuit_breaker_opens_after_threshold() {
        let mut cb = CircuitBreaker::new(3, Duration::from_millis(100));

        let failing = || -> Result<(), &str> { Err("fail") };

        // First two failures keep circuit closed
        let _ = cb.call(failing);
        assert_eq!(*cb.get_state(), CircuitState::Closed);

        let _ = cb.call(failing);
        assert_eq!(*cb.get_state(), CircuitState::Closed);

        // Third failure opens circuit
        let _ = cb.call(failing);
        assert_eq!(*cb.get_state(), CircuitState::Open);
    }

    #[test]
    fn test_circuit_breaker_rejects_when_open() {
        let mut cb = CircuitBreaker::new(1, Duration::from_secs(60));

        // Open the circuit
        let failing = || -> Result<(), &str> { Err("fail") };
        let _ = cb.call(failing);
        assert_eq!(*cb.get_state(), CircuitState::Open);

        // Verify calls are rejected
        let success = || -> Result<(), &str> { Ok(()) };
        let result = cb.call(success);
        assert!(matches!(result, Err(CircuitBreakerError::CircuitOpen)));
    }

    #[test]
    fn test_circuit_breaker_recovery() {
        let mut cb = CircuitBreaker::new(1, Duration::from_millis(10));

        // Open the circuit
        let failing = || -> Result<(), &str> { Err("fail") };
        let _ = cb.call(failing);
        assert_eq!(*cb.get_state(), CircuitState::Open);

        // Wait for timeout
        std::thread::sleep(Duration::from_millis(20));

        // Should transition to half-open and then closed on success
        let success = || -> Result<(), &str> { Ok(()) };
        let result = cb.call(success);
        assert!(result.is_ok());
        assert_eq!(*cb.get_state(), CircuitState::Closed);
    }

    #[test]
    fn test_rate_limit_exceeded() {
        let now = 1000_i64;
        let requests = vec![now, now - 100, now - 200];
        assert!(rate_limit_sliding_window(&requests, 1000, 2));
    }

    #[test]
    fn test_rate_limit_not_exceeded() {
        let now = 1000_i64;
        let requests = vec![now, now - 100];
        assert!(!rate_limit_sliding_window(&requests, 1000, 3));
    }

    #[test]
    fn test_rate_limit_outside_window() {
        let now = 1000_i64;
        let requests = vec![now, now - 500, now - 1500];
        // Only 2 requests within window, limit is 2
        assert!(!rate_limit_sliding_window(&requests, 1000, 2));
    }
}

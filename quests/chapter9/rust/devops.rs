// Chapter 9: Dungeon of DevOps - Docker & Configuration
// Player solutions will live here.
// TODO: Implement configuration parsing, health checks, and environment handling.

use std::collections::HashMap;
use std::env;

/// Config holds application configuration settings
/// Used to parse and validate configuration from various sources
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,  // Server hostname or IP address
    pub port: i32,     // Server port number
    pub debug: bool,   // Enable debug mode
}

impl Config {
    /// Creates a new Config with default values
    pub fn new() -> Self {
        Config {
            host: String::new(),
            port: 0,
            debug: false,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a configuration string in key=value format
/// Each key=value pair is separated by newlines
/// Example input: "host=localhost\nport=8080\ndebug=true"
/// Returns a Config struct or an error message if parsing fails
pub fn parse_config(config_str: &str) -> Result<Config, String> {
    // TODO: Parse the configuration string
    // - Split by newlines to get individual key=value pairs
    // - For each pair, split by '=' to get key and value
    // - Map keys to Config struct fields (host, port, debug)
    // - Convert port to i32 and debug to bool
    // - Return Err if port conversion fails
    let _ = config_str; // Prevent unused variable warning
    Ok(Config::new())
}

/// Check the status of multiple services
/// Takes a HashMap of service names to their up/down status (true = up, false = down)
/// Returns a HashMap of service names to status strings ("healthy" or "unhealthy")
/// Example: {"db": true, "cache": false} -> {"db": "healthy", "cache": "unhealthy"}
pub fn health_check(services: &HashMap<String, bool>) -> HashMap<String, String> {
    // TODO: Convert service boolean status to string status
    // - Iterate over the services map
    // - For each service, if true return "healthy", if false return "unhealthy"
    // - Return the resulting status map
    let _ = services; // Prevent unused variable warning
    HashMap::new()
}

/// Retrieve an environment variable or return a default value
/// This is a common pattern for configuration with fallback defaults
/// Example: get_env_with_default("PORT", "3000") returns "3000" if PORT is not set
pub fn get_env_with_default(key: &str, default_value: &str) -> String {
    // TODO: Get environment variable with fallback
    // - Use std::env::var to retrieve the environment variable
    // - If the value is empty or not found, return the default value
    // - Otherwise return the environment variable value
    let _ = (key, default_value, env::var); // Prevent unused warnings
    String::new()
}

/// Check if a port number is within the valid range
/// Valid ports are 1-65535 (0 is reserved, 65536+ are invalid)
/// Returns true if valid, false otherwise
pub fn validate_port(port: i32) -> bool {
    // TODO: Validate the port number
    // - Check if port is greater than 0
    // - Check if port is less than or equal to 65535
    // - Return true if both conditions are met
    let _ = port; // Prevent unused variable warning
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_basic() {
        let config = parse_config("host=localhost\nport=8080\ndebug=true").unwrap();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
        assert!(config.debug);
    }

    #[test]
    fn test_health_check_all_healthy() {
        let mut services = HashMap::new();
        services.insert("db".to_string(), true);
        services.insert("cache".to_string(), true);

        let result = health_check(&services);
        assert_eq!(result.get("db"), Some(&"healthy".to_string()));
        assert_eq!(result.get("cache"), Some(&"healthy".to_string()));
    }

    #[test]
    fn test_validate_port_valid() {
        assert!(validate_port(80));
        assert!(validate_port(443));
        assert!(validate_port(8080));
        assert!(validate_port(65535));
    }

    #[test]
    fn test_validate_port_invalid() {
        assert!(!validate_port(0));
        assert!(!validate_port(-1));
        assert!(!validate_port(65536));
    }
}

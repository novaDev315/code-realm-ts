// Reference solutions for Chapter 9 - DevOps Configuration & Health Checks

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
pub fn parse_config(config_str: &str) -> Result<Config, String> {
    let mut config = Config::new();

    // Split by newlines to get individual key=value pairs
    for line in config_str.lines() {
        // Skip empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Split by '=' to get key and value
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            // Map keys to Config struct fields
            match key {
                "host" => config.host = value.to_string(),
                "port" => {
                    config.port = value.parse::<i32>()
                        .map_err(|e| format!("Invalid port: {}", e))?;
                }
                "debug" => config.debug = value == "true",
                _ => {} // Ignore unknown keys
            }
        }
    }

    Ok(config)
}

/// Check the status of multiple services
/// Takes a HashMap of service names to their up/down status (true = up, false = down)
/// Returns a HashMap of service names to status strings ("healthy" or "unhealthy")
pub fn health_check(services: &HashMap<String, bool>) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for (name, is_up) in services {
        let status = if *is_up { "healthy" } else { "unhealthy" };
        result.insert(name.clone(), status.to_string());
    }

    result
}

/// Retrieve an environment variable or return a default value
/// This is a common pattern for configuration with fallback defaults
pub fn get_env_with_default(key: &str, default_value: &str) -> String {
    match env::var(key) {
        Ok(value) if !value.is_empty() => value,
        _ => default_value.to_string(),
    }
}

/// Check if a port number is within the valid range
/// Valid ports are 1-65535 (0 is reserved, 65536+ are invalid)
pub fn validate_port(port: i32) -> bool {
    port > 0 && port <= 65535
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
    fn test_parse_config_production() {
        let config = parse_config("host=0.0.0.0\nport=3000\ndebug=false").unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert!(!config.debug);
    }

    #[test]
    fn test_parse_config_different_order() {
        let config = parse_config("port=9000\nhost=server.local\ndebug=true").unwrap();
        assert_eq!(config.host, "server.local");
        assert_eq!(config.port, 9000);
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
    fn test_health_check_all_unhealthy() {
        let mut services = HashMap::new();
        services.insert("db".to_string(), false);
        services.insert("cache".to_string(), false);

        let result = health_check(&services);
        assert_eq!(result.get("db"), Some(&"unhealthy".to_string()));
        assert_eq!(result.get("cache"), Some(&"unhealthy".to_string()));
    }

    #[test]
    fn test_health_check_mixed() {
        let mut services = HashMap::new();
        services.insert("api".to_string(), true);
        services.insert("db".to_string(), false);

        let result = health_check(&services);
        assert_eq!(result.get("api"), Some(&"healthy".to_string()));
        assert_eq!(result.get("db"), Some(&"unhealthy".to_string()));
    }

    #[test]
    fn test_health_check_empty() {
        let services: HashMap<String, bool> = HashMap::new();
        let result = health_check(&services);
        assert!(result.is_empty());
    }

    #[test]
    fn test_validate_port_valid() {
        assert!(validate_port(80));
        assert!(validate_port(443));
        assert!(validate_port(8080));
        assert!(validate_port(3000));
        assert!(validate_port(65535));
        assert!(validate_port(1));
    }

    #[test]
    fn test_validate_port_invalid() {
        assert!(!validate_port(0));
        assert!(!validate_port(-1));
        assert!(!validate_port(65536));
        assert!(!validate_port(100000));
    }

    #[test]
    fn test_get_env_with_default_unset() {
        let result = get_env_with_default("NONEXISTENT_VAR_TEST", "default");
        assert_eq!(result, "default");
    }

    #[test]
    fn test_get_env_with_default_set() {
        env::set_var("TEST_ENV_VAR_SOLUTIONS", "custom");
        let result = get_env_with_default("TEST_ENV_VAR_SOLUTIONS", "default");
        assert_eq!(result, "custom");
        env::remove_var("TEST_ENV_VAR_SOLUTIONS");
    }
}

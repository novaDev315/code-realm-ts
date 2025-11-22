// Chapter 9: Dungeon of DevOps - Docker & Configuration
// Player solutions will live here.
// TODO: Implement configuration parsing, health checks, and environment handling.

package main

import (
	"os"
)

// Config holds application configuration settings
// Used to parse and validate configuration from various sources
type Config struct {
	Host  string // Server hostname or IP address
	Port  int    // Server port number
	Debug bool   // Enable debug mode
}

// ParseConfig parses a configuration string in key=value format
// Each key=value pair is separated by newlines
// Example input: "host=localhost\nport=8080\ndebug=true"
// Returns a Config struct or an error if parsing fails
func ParseConfig(configStr string) (*Config, error) {
	// TODO: Parse the configuration string
	// - Split by newlines to get individual key=value pairs
	// - For each pair, split by '=' to get key and value
	// - Map keys to Config struct fields (host, port, debug)
	// - Convert port to int and debug to bool
	// - Return error if port conversion fails
	return nil, nil
}

// HealthCheck checks the status of multiple services
// Takes a map of service names to their up/down status (true = up, false = down)
// Returns a map of service names to status strings ("healthy" or "unhealthy")
// Example: {"db": true, "cache": false} -> {"db": "healthy", "cache": "unhealthy"}
func HealthCheck(services map[string]bool) map[string]string {
	// TODO: Convert service boolean status to string status
	// - Iterate over the services map
	// - For each service, if true return "healthy", if false return "unhealthy"
	// - Return the resulting status map
	return nil
}

// GetEnvWithDefault retrieves an environment variable or returns a default value
// This is a common pattern for configuration with fallback defaults
// Example: GetEnvWithDefault("PORT", "3000") returns "3000" if PORT is not set
func GetEnvWithDefault(key, defaultValue string) string {
	// TODO: Get environment variable with fallback
	// - Use os.Getenv to retrieve the environment variable
	// - If the value is empty, return the default value
	// - Otherwise return the environment variable value
	_ = os // Prevent unused import error
	return ""
}

// ValidatePort checks if a port number is within the valid range
// Valid ports are 1-65535 (0 is reserved, 65536+ are invalid)
// Returns true if valid, false otherwise
func ValidatePort(port int) bool {
	// TODO: Validate the port number
	// - Check if port is greater than 0
	// - Check if port is less than or equal to 65535
	// - Return true if both conditions are met
	return false
}

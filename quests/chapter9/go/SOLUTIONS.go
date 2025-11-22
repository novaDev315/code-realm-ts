// Reference solutions for Chapter 9 - DevOps Configuration & Health Checks

package main

import (
	"os"
	"strconv"
	"strings"
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
func ParseConfig(configStr string) (*Config, error) {
	config := &Config{}

	// Split by newlines to get individual key=value pairs
	lines := strings.Split(configStr, "\n")

	for _, line := range lines {
		// Skip empty lines
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		// Split by '=' to get key and value
		parts := strings.SplitN(line, "=", 2)
		if len(parts) != 2 {
			continue
		}

		key := strings.TrimSpace(parts[0])
		value := strings.TrimSpace(parts[1])

		// Map keys to Config struct fields
		switch key {
		case "host":
			config.Host = value
		case "port":
			port, err := strconv.Atoi(value)
			if err != nil {
				return nil, err
			}
			config.Port = port
		case "debug":
			config.Debug = value == "true"
		}
	}

	return config, nil
}

// HealthCheck checks the status of multiple services
// Takes a map of service names to their up/down status (true = up, false = down)
// Returns a map of service names to status strings ("healthy" or "unhealthy")
func HealthCheck(services map[string]bool) map[string]string {
	result := make(map[string]string)

	for name, isUp := range services {
		if isUp {
			result[name] = "healthy"
		} else {
			result[name] = "unhealthy"
		}
	}

	return result
}

// GetEnvWithDefault retrieves an environment variable or returns a default value
// This is a common pattern for configuration with fallback defaults
func GetEnvWithDefault(key, defaultValue string) string {
	value := os.Getenv(key)
	if value == "" {
		return defaultValue
	}
	return value
}

// ValidatePort checks if a port number is within the valid range
// Valid ports are 1-65535 (0 is reserved, 65536+ are invalid)
func ValidatePort(port int) bool {
	return port > 0 && port <= 65535
}

// Test runner for Chapter 9 - DevOps Configuration & Health Checks

package main

import (
	"fmt"
	"os"
)

func main() {
	passed := true

	// Test ParseConfig
	fmt.Println("Testing ParseConfig...")
	parseConfigCases := []struct {
		input      string
		expectHost string
		expectPort int
		expectDbg  bool
		name       string
	}{
		{"host=localhost\nport=8080\ndebug=true", "localhost", 8080, true, "Basic config"},
		{"host=0.0.0.0\nport=3000\ndebug=false", "0.0.0.0", 3000, false, "Production config"},
		{"host=api.example.com\nport=443\ndebug=false", "api.example.com", 443, false, "HTTPS config"},
		{"port=9000\nhost=server.local\ndebug=true", "server.local", 9000, true, "Different order"},
	}

	for _, tc := range parseConfigCases {
		result, err := ParseConfig(tc.input)
		if err != nil {
			fmt.Printf("  X ParseConfig - %s: unexpected error: %v\n", tc.name, err)
			passed = false
		} else if result == nil {
			fmt.Printf("  X ParseConfig - %s: got nil result\n", tc.name)
			passed = false
		} else if result.Host != tc.expectHost || result.Port != tc.expectPort || result.Debug != tc.expectDbg {
			fmt.Printf("  X ParseConfig - %s: expected {%s, %d, %v} but got {%s, %d, %v}\n",
				tc.name, tc.expectHost, tc.expectPort, tc.expectDbg, result.Host, result.Port, result.Debug)
			passed = false
		} else {
			fmt.Printf("  OK ParseConfig - %s: {Host: %s, Port: %d, Debug: %v}\n",
				tc.name, result.Host, result.Port, result.Debug)
		}
	}

	// Test HealthCheck
	fmt.Println("\nTesting HealthCheck...")
	healthCheckCases := []struct {
		input  map[string]bool
		expect map[string]string
		name   string
	}{
		{
			map[string]bool{"db": true, "cache": true},
			map[string]string{"db": "healthy", "cache": "healthy"},
			"All healthy",
		},
		{
			map[string]bool{"db": false, "cache": false},
			map[string]string{"db": "unhealthy", "cache": "unhealthy"},
			"All unhealthy",
		},
		{
			map[string]bool{"api": true, "db": false, "cache": true},
			map[string]string{"api": "healthy", "db": "unhealthy", "cache": "healthy"},
			"Mixed status",
		},
		{
			map[string]bool{},
			map[string]string{},
			"Empty services",
		},
	}

	for _, tc := range healthCheckCases {
		result := HealthCheck(tc.input)
		if result == nil {
			fmt.Printf("  X HealthCheck - %s: got nil result\n", tc.name)
			passed = false
			continue
		}

		match := len(result) == len(tc.expect)
		if match {
			for k, v := range tc.expect {
				if result[k] != v {
					match = false
					break
				}
			}
		}

		if !match {
			fmt.Printf("  X HealthCheck - %s: expected %v but got %v\n", tc.name, tc.expect, result)
			passed = false
		} else {
			fmt.Printf("  OK HealthCheck - %s: %v\n", tc.name, result)
		}
	}

	// Test GetEnvWithDefault
	fmt.Println("\nTesting GetEnvWithDefault...")

	// Test with unset variable
	result := GetEnvWithDefault("NONEXISTENT_VAR_12345", "default_value")
	if result != "default_value" {
		fmt.Printf("  X GetEnvWithDefault - Unset var: expected \"default_value\" but got \"%s\"\n", result)
		passed = false
	} else {
		fmt.Printf("  OK GetEnvWithDefault - Unset var returns default: \"%s\"\n", result)
	}

	// Test with set variable
	os.Setenv("TEST_VAR_CHAPTER9", "custom_value")
	result = GetEnvWithDefault("TEST_VAR_CHAPTER9", "default_value")
	if result != "custom_value" {
		fmt.Printf("  X GetEnvWithDefault - Set var: expected \"custom_value\" but got \"%s\"\n", result)
		passed = false
	} else {
		fmt.Printf("  OK GetEnvWithDefault - Set var returns value: \"%s\"\n", result)
	}
	os.Unsetenv("TEST_VAR_CHAPTER9")

	// Test with empty string variable
	os.Setenv("EMPTY_VAR_CHAPTER9", "")
	result = GetEnvWithDefault("EMPTY_VAR_CHAPTER9", "fallback")
	if result != "fallback" {
		fmt.Printf("  X GetEnvWithDefault - Empty var: expected \"fallback\" but got \"%s\"\n", result)
		passed = false
	} else {
		fmt.Printf("  OK GetEnvWithDefault - Empty var returns default: \"%s\"\n", result)
	}
	os.Unsetenv("EMPTY_VAR_CHAPTER9")

	// Test ValidatePort
	fmt.Println("\nTesting ValidatePort...")
	validatePortCases := []struct {
		input  int
		expect bool
		name   string
	}{
		{80, true, "HTTP port"},
		{443, true, "HTTPS port"},
		{8080, true, "Common dev port"},
		{3000, true, "Node.js port"},
		{65535, true, "Max valid port"},
		{1, true, "Min valid port"},
		{0, false, "Zero (reserved)"},
		{-1, false, "Negative port"},
		{65536, false, "Above max port"},
		{100000, false, "Way above max"},
	}

	for _, tc := range validatePortCases {
		result := ValidatePort(tc.input)
		if result != tc.expect {
			fmt.Printf("  X ValidatePort(%d) - %s: expected %v but got %v\n", tc.input, tc.name, tc.expect, result)
			passed = false
		} else {
			fmt.Printf("  OK ValidatePort(%d) = %v (%s)\n", tc.input, result, tc.name)
		}
	}

	if passed {
		fmt.Println("\nAll tests passed!")
	} else {
		fmt.Println("\nSome tests failed.")
		os.Exit(1)
	}
}

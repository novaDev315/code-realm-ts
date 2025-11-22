// Test runner for Chapter 8 - REST & GraphQL API Design

package main

import (
	"encoding/json"
	"fmt"
	"os"
	"strings"
)

func main() {
	passed := true

	// Test NewRouter
	fmt.Println("Testing NewRouter...")
	router := NewRouter()
	if router == nil {
		fmt.Println("  X NewRouter: returned nil")
		passed = false
	} else {
		fmt.Println("  OK NewRouter: created router instance")
	}

	// Test AddRoute and GetRoutes
	fmt.Println("\nTesting AddRoute and GetRoutes...")
	if router != nil {
		router.AddRoute("GET", "/users", "listUsers")
		router.AddRoute("POST", "/users", "createUser")
		router.AddRoute("GET", "/users/:id", "getUser")
		router.AddRoute("DELETE", "/users/:id", "deleteUser")

		routes := router.GetRoutes()
		if routes == nil {
			fmt.Println("  X GetRoutes: returned nil")
			passed = false
		} else if len(routes) != 4 {
			fmt.Printf("  X GetRoutes: expected 4 routes but got %d\n", len(routes))
			passed = false
		} else {
			fmt.Printf("  OK AddRoute/GetRoutes: registered %d routes\n", len(routes))
		}
	}

	// Test Match - found cases
	fmt.Println("\nTesting Match (found cases)...")
	matchFoundCases := []struct {
		method  string
		path    string
		handler string
		name    string
	}{
		{"GET", "/users", "listUsers", "GET /users"},
		{"POST", "/users", "createUser", "POST /users"},
		{"GET", "/users/:id", "getUser", "GET /users/:id"},
		{"DELETE", "/users/:id", "deleteUser", "DELETE /users/:id"},
	}

	for _, tc := range matchFoundCases {
		if router != nil {
			handler, found := router.Match(tc.method, tc.path)
			if !found {
				fmt.Printf("  X Match - %s: route not found\n", tc.name)
				passed = false
			} else if handler != tc.handler {
				fmt.Printf("  X Match - %s: expected handler \"%s\" but got \"%s\"\n", tc.name, tc.handler, handler)
				passed = false
			} else {
				fmt.Printf("  OK Match - %s -> %s\n", tc.name, handler)
			}
		}
	}

	// Test Match - not found cases
	fmt.Println("\nTesting Match (not found cases)...")
	matchNotFoundCases := []struct {
		method string
		path   string
		name   string
	}{
		{"PUT", "/users", "PUT /users (not registered)"},
		{"GET", "/posts", "GET /posts (different path)"},
		{"GET", "/", "GET / (root not registered)"},
	}

	for _, tc := range matchNotFoundCases {
		if router != nil {
			handler, found := router.Match(tc.method, tc.path)
			if found {
				fmt.Printf("  X Match - %s: should not find route, but got handler \"%s\"\n", tc.name, handler)
				passed = false
			} else {
				fmt.Printf("  OK Match - %s: correctly not found\n", tc.name)
			}
		}
	}

	// Test SuccessResponse
	fmt.Println("\nTesting SuccessResponse...")
	successCases := []struct {
		data   interface{}
		name   string
	}{
		{map[string]string{"message": "Hello"}, "Map data"},
		{[]int{1, 2, 3}, "Slice data"},
		{"simple string", "String data"},
		{nil, "Nil data"},
	}

	for _, tc := range successCases {
		resp := SuccessResponse(tc.data)
		if resp.Status != 200 {
			fmt.Printf("  X SuccessResponse - %s: expected status 200 but got %d\n", tc.name, resp.Status)
			passed = false
		} else if resp.Error != "" {
			fmt.Printf("  X SuccessResponse - %s: expected empty error but got \"%s\"\n", tc.name, resp.Error)
			passed = false
		} else {
			fmt.Printf("  OK SuccessResponse - %s: status=%d\n", tc.name, resp.Status)
		}
	}

	// Test ErrorResponse
	fmt.Println("\nTesting ErrorResponse...")
	errorCases := []struct {
		status  int
		message string
		name    string
	}{
		{400, "Bad request", "Bad Request"},
		{401, "Unauthorized", "Unauthorized"},
		{404, "Not found", "Not Found"},
		{500, "Internal server error", "Server Error"},
	}

	for _, tc := range errorCases {
		resp := ErrorResponse(tc.status, tc.message)
		if resp.Status != tc.status {
			fmt.Printf("  X ErrorResponse - %s: expected status %d but got %d\n", tc.name, tc.status, resp.Status)
			passed = false
		} else if resp.Error != tc.message {
			fmt.Printf("  X ErrorResponse - %s: expected error \"%s\" but got \"%s\"\n", tc.name, tc.message, resp.Error)
			passed = false
		} else if resp.Data != nil {
			fmt.Printf("  X ErrorResponse - %s: expected nil data but got %v\n", tc.name, resp.Data)
			passed = false
		} else {
			fmt.Printf("  OK ErrorResponse - %s: status=%d, error=\"%s\"\n", tc.name, resp.Status, resp.Error)
		}
	}

	// Test ToJSON
	fmt.Println("\nTesting ToJSON...")

	// Success response JSON
	successResp := SuccessResponse(map[string]string{"id": "123", "name": "Test"})
	jsonStr := successResp.ToJSON()
	if jsonStr == "" {
		fmt.Println("  X ToJSON - Success response: returned empty string")
		passed = false
	} else if !strings.Contains(jsonStr, "200") || !strings.Contains(jsonStr, "status") {
		fmt.Printf("  X ToJSON - Success response: missing expected fields in \"%s\"\n", jsonStr)
		passed = false
	} else {
		// Verify it's valid JSON
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(jsonStr), &parsed); err != nil {
			fmt.Printf("  X ToJSON - Success response: invalid JSON: %v\n", err)
			passed = false
		} else {
			fmt.Printf("  OK ToJSON - Success response: %s\n", jsonStr)
		}
	}

	// Error response JSON
	errorResp := ErrorResponse(404, "Resource not found")
	jsonStr = errorResp.ToJSON()
	if jsonStr == "" {
		fmt.Println("  X ToJSON - Error response: returned empty string")
		passed = false
	} else if !strings.Contains(jsonStr, "404") || !strings.Contains(jsonStr, "Resource not found") {
		fmt.Printf("  X ToJSON - Error response: missing expected fields in \"%s\"\n", jsonStr)
		passed = false
	} else {
		// Verify it's valid JSON
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(jsonStr), &parsed); err != nil {
			fmt.Printf("  X ToJSON - Error response: invalid JSON: %v\n", err)
			passed = false
		} else {
			fmt.Printf("  OK ToJSON - Error response: %s\n", jsonStr)
		}
	}

	// Test empty router
	fmt.Println("\nTesting empty router...")
	emptyRouter := NewRouter()
	if emptyRouter != nil {
		routes := emptyRouter.GetRoutes()
		if routes == nil || len(routes) != 0 {
			if routes == nil {
				fmt.Println("  X Empty router: GetRoutes returned nil (should return empty slice)")
				passed = false
			} else {
				fmt.Printf("  X Empty router: expected 0 routes but got %d\n", len(routes))
				passed = false
			}
		} else {
			fmt.Println("  OK Empty router: GetRoutes returns empty slice")
		}

		handler, found := emptyRouter.Match("GET", "/anything")
		if found {
			fmt.Printf("  X Empty router: Match should return false, but got handler \"%s\"\n", handler)
			passed = false
		} else {
			fmt.Println("  OK Empty router: Match correctly returns not found")
		}
	}

	if passed {
		fmt.Println("\nAll tests passed!")
	} else {
		fmt.Println("\nSome tests failed.")
		os.Exit(1)
	}
}

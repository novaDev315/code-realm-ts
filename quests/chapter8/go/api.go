// Chapter 8: Realm of APIs - REST & GraphQL API Design
// Player solutions will live here.
// TODO: Implement API routing and response handling.

package main

import (
	"encoding/json"
)

// Route represents a single API route configuration
// Used to define method, path, and handler mappings
type Route struct {
	Method  string // HTTP method (GET, POST, PUT, DELETE, etc.)
	Path    string // URL path pattern
	Handler string // Handler function name
}

// Router manages a collection of API routes
// Provides route registration and matching functionality
type Router struct {
	routes []Route // Collection of registered routes
}

// NewRouter creates and returns a new Router instance
// Initializes an empty routes collection
func NewRouter() *Router {
	// TODO: Create a new Router with an empty routes slice
	// - Allocate a new Router struct
	// - Initialize the routes slice (can be nil or empty)
	// - Return the pointer to the new Router
	return nil
}

// AddRoute registers a new route with the router
// Takes HTTP method, URL path, and handler name as parameters
// Example: router.AddRoute("GET", "/users", "listUsers")
func (r *Router) AddRoute(method, path, handler string) {
	// TODO: Add a new route to the router
	// - Create a new Route struct with the provided values
	// - Append the route to the router's routes slice
}

// Match finds a handler for the given method and path
// Returns the handler name and true if found, empty string and false otherwise
// Example: handler, found := router.Match("GET", "/users")
func (r *Router) Match(method, path string) (string, bool) {
	// TODO: Find a matching route
	// - Iterate through all registered routes
	// - Compare method and path (exact match)
	// - Return handler and true if found
	// - Return empty string and false if no match
	return "", false
}

// GetRoutes returns all registered routes
// Useful for debugging and introspection
func (r *Router) GetRoutes() []Route {
	// TODO: Return the routes slice
	// - Simply return the internal routes collection
	return nil
}

// APIResponse represents a standard API response structure
// Used for consistent response formatting across all endpoints
type APIResponse struct {
	Status int         `json:"status"` // HTTP status code
	Data   interface{} `json:"data"`   // Response payload (success)
	Error  string      `json:"error"`  // Error message (failure)
}

// SuccessResponse creates a successful API response
// Takes any data payload and returns an APIResponse with status 200
// Example: resp := SuccessResponse(map[string]string{"message": "OK"})
func SuccessResponse(data interface{}) APIResponse {
	// TODO: Create a success response
	// - Set Status to 200
	// - Set Data to the provided data
	// - Leave Error empty
	// - Return the APIResponse
	_ = data // Prevent unused variable warning
	return APIResponse{}
}

// ErrorResponse creates an error API response
// Takes a status code and error message
// Example: resp := ErrorResponse(404, "User not found")
func ErrorResponse(status int, message string) APIResponse {
	// TODO: Create an error response
	// - Set Status to the provided status code
	// - Set Error to the provided message
	// - Leave Data as nil
	// - Return the APIResponse
	_ = status  // Prevent unused variable warning
	_ = message // Prevent unused variable warning
	return APIResponse{}
}

// ToJSON converts the APIResponse to a JSON string
// Returns the JSON representation or an error JSON if marshaling fails
// Example: jsonStr := resp.ToJSON()
func (resp APIResponse) ToJSON() string {
	// TODO: Convert response to JSON string
	// - Use json.Marshal to convert the struct to JSON bytes
	// - Convert bytes to string and return
	// - If marshaling fails, return a simple error JSON string
	_ = json.Marshal // Prevent unused import warning
	return ""
}

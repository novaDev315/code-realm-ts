// Reference solutions for Chapter 8 - REST & GraphQL API Design

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
	return &Router{
		routes: []Route{},
	}
}

// AddRoute registers a new route with the router
// Takes HTTP method, URL path, and handler name as parameters
func (r *Router) AddRoute(method, path, handler string) {
	route := Route{
		Method:  method,
		Path:    path,
		Handler: handler,
	}
	r.routes = append(r.routes, route)
}

// Match finds a handler for the given method and path
// Returns the handler name and true if found, empty string and false otherwise
func (r *Router) Match(method, path string) (string, bool) {
	for _, route := range r.routes {
		if route.Method == method && route.Path == path {
			return route.Handler, true
		}
	}
	return "", false
}

// GetRoutes returns all registered routes
// Useful for debugging and introspection
func (r *Router) GetRoutes() []Route {
	return r.routes
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
func SuccessResponse(data interface{}) APIResponse {
	return APIResponse{
		Status: 200,
		Data:   data,
		Error:  "",
	}
}

// ErrorResponse creates an error API response
// Takes a status code and error message
func ErrorResponse(status int, message string) APIResponse {
	return APIResponse{
		Status: status,
		Data:   nil,
		Error:  message,
	}
}

// ToJSON converts the APIResponse to a JSON string
// Returns the JSON representation or an error JSON if marshaling fails
func (resp APIResponse) ToJSON() string {
	bytes, err := json.Marshal(resp)
	if err != nil {
		return `{"status":500,"error":"JSON marshaling failed"}`
	}
	return string(bytes)
}

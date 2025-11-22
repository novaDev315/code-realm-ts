// Chapter 8: Realm of APIs - REST & GraphQL API Design
// Player solutions will live here.
// TODO: Implement API routing and response handling.

/// Route represents a single API route configuration
/// Used to define method, path, and handler mappings
#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub method: String,  // HTTP method (GET, POST, PUT, DELETE, etc.)
    pub path: String,    // URL path pattern
    pub handler: String, // Handler function name
}

/// Router manages a collection of API routes
/// Provides route registration and matching functionality
#[derive(Debug, Clone)]
pub struct Router {
    routes: Vec<Route>, // Collection of registered routes
}

impl Router {
    /// Creates and returns a new Router instance
    /// Initializes an empty routes collection
    pub fn new() -> Self {
        // TODO: Create a new Router with an empty routes vector
        // - Return a Router struct with an empty Vec<Route>
        Router {
            routes: Vec::new(),
        }
    }

    /// Registers a new route with the router
    /// Takes HTTP method, URL path, and handler name as parameters
    /// Example: router.add_route("GET", "/users", "list_users")
    pub fn add_route(&mut self, method: &str, path: &str, handler: &str) {
        // TODO: Add a new route to the router
        // - Create a new Route struct with the provided values (convert to String)
        // - Push the route to the router's routes vector
        let _ = (method, path, handler); // Prevent unused variable warnings
    }

    /// Finds a handler for the given method and path
    /// Returns Some(handler) if found, None otherwise
    /// Example: let handler = router.match_route("GET", "/users")
    pub fn match_route(&self, method: &str, path: &str) -> Option<String> {
        // TODO: Find a matching route
        // - Iterate through all registered routes
        // - Compare method and path (exact match)
        // - Return Some(handler.clone()) if found
        // - Return None if no match
        let _ = (method, path); // Prevent unused variable warnings
        None
    }

    /// Returns all registered routes
    /// Useful for debugging and introspection
    pub fn get_routes(&self) -> &[Route] {
        // TODO: Return a slice reference to the routes vector
        // - Simply return &self.routes
        &[]
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

/// APIResponse represents a standard API response structure
/// Used for consistent response formatting across all endpoints
#[derive(Debug, Clone, PartialEq)]
pub struct APIResponse {
    pub status: i32,             // HTTP status code
    pub data: Option<String>,    // Response payload as string (success)
    pub error: String,           // Error message (failure)
}

impl APIResponse {
    /// Creates a successful API response
    /// Takes data string and returns an APIResponse with status 200
    /// Example: let resp = APIResponse::success("user data")
    pub fn success(data: &str) -> Self {
        // TODO: Create a success response
        // - Set status to 200
        // - Set data to Some(data.to_string())
        // - Set error to empty string
        // - Return the APIResponse
        let _ = data; // Prevent unused variable warning
        APIResponse {
            status: 0,
            data: None,
            error: String::new(),
        }
    }

    /// Creates an error API response
    /// Takes a status code and error message
    /// Example: let resp = APIResponse::error(404, "User not found")
    pub fn error(status: i32, message: &str) -> Self {
        // TODO: Create an error response
        // - Set status to the provided status code
        // - Set data to None
        // - Set error to the provided message
        // - Return the APIResponse
        let _ = (status, message); // Prevent unused variable warnings
        APIResponse {
            status: 0,
            data: None,
            error: String::new(),
        }
    }

    /// Converts the APIResponse to a JSON string
    /// Returns the JSON representation
    /// Example: {"status":200,"data":"hello","error":""}
    pub fn to_json(&self) -> String {
        // TODO: Convert response to JSON string manually
        // - Build a JSON string with status, data, and error fields
        // - Handle the Option<String> for data (use null if None)
        // - Escape any special characters in strings
        String::new()
    }
}

/// Helper function to escape special JSON characters in a string
fn escape_json_string(s: &str) -> String {
    // TODO: Escape special characters for JSON
    // - Replace backslash with \\
    // - Replace double quote with \"
    // - Replace newline with \n
    // - Replace tab with \t
    let _ = s; // Prevent unused variable warning
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_router() {
        let router = Router::new();
        assert!(router.get_routes().is_empty());
    }

    #[test]
    fn test_add_route() {
        let mut router = Router::new();
        router.add_route("GET", "/users", "list_users");
        assert_eq!(router.get_routes().len(), 1);
    }

    #[test]
    fn test_match_route_found() {
        let mut router = Router::new();
        router.add_route("GET", "/users", "list_users");
        let handler = router.match_route("GET", "/users");
        assert_eq!(handler, Some("list_users".to_string()));
    }

    #[test]
    fn test_match_route_not_found() {
        let mut router = Router::new();
        router.add_route("GET", "/users", "list_users");
        let handler = router.match_route("POST", "/users");
        assert_eq!(handler, None);
    }

    #[test]
    fn test_success_response() {
        let resp = APIResponse::success("test data");
        assert_eq!(resp.status, 200);
        assert_eq!(resp.data, Some("test data".to_string()));
        assert!(resp.error.is_empty());
    }

    #[test]
    fn test_error_response() {
        let resp = APIResponse::error(404, "Not found");
        assert_eq!(resp.status, 404);
        assert_eq!(resp.error, "Not found");
        assert!(resp.data.is_none());
    }

    #[test]
    fn test_to_json() {
        let resp = APIResponse::success("hello");
        let json = resp.to_json();
        assert!(json.contains("200"));
        assert!(json.contains("hello"));
    }
}

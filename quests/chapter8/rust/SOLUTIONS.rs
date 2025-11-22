// Reference solutions for Chapter 8 - REST & GraphQL API Design

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
        Router {
            routes: Vec::new(),
        }
    }

    /// Registers a new route with the router
    /// Takes HTTP method, URL path, and handler name as parameters
    pub fn add_route(&mut self, method: &str, path: &str, handler: &str) {
        let route = Route {
            method: method.to_string(),
            path: path.to_string(),
            handler: handler.to_string(),
        };
        self.routes.push(route);
    }

    /// Finds a handler for the given method and path
    /// Returns Some(handler) if found, None otherwise
    pub fn match_route(&self, method: &str, path: &str) -> Option<String> {
        for route in &self.routes {
            if route.method == method && route.path == path {
                return Some(route.handler.clone());
            }
        }
        None
    }

    /// Returns all registered routes
    /// Useful for debugging and introspection
    pub fn get_routes(&self) -> &[Route] {
        &self.routes
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
    pub fn success(data: &str) -> Self {
        APIResponse {
            status: 200,
            data: Some(data.to_string()),
            error: String::new(),
        }
    }

    /// Creates an error API response
    /// Takes a status code and error message
    pub fn error(status: i32, message: &str) -> Self {
        APIResponse {
            status,
            data: None,
            error: message.to_string(),
        }
    }

    /// Converts the APIResponse to a JSON string
    /// Returns the JSON representation
    pub fn to_json(&self) -> String {
        let data_str = match &self.data {
            Some(d) => format!("\"{}\"", escape_json_string(d)),
            None => "null".to_string(),
        };

        let error_str = escape_json_string(&self.error);

        format!(
            "{{\"status\":{},\"data\":{},\"error\":\"{}\"}}",
            self.status, data_str, error_str
        )
    }
}

/// Helper function to escape special JSON characters in a string
fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result
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
    fn test_add_single_route() {
        let mut router = Router::new();
        router.add_route("GET", "/users", "list_users");

        let routes = router.get_routes();
        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0].method, "GET");
        assert_eq!(routes[0].path, "/users");
        assert_eq!(routes[0].handler, "list_users");
    }

    #[test]
    fn test_add_multiple_routes() {
        let mut router = Router::new();
        router.add_route("GET", "/users", "list_users");
        router.add_route("POST", "/users", "create_user");
        router.add_route("GET", "/users/:id", "get_user");
        router.add_route("DELETE", "/users/:id", "delete_user");

        assert_eq!(router.get_routes().len(), 4);
    }

    #[test]
    fn test_match_route_get() {
        let mut router = Router::new();
        router.add_route("GET", "/users", "list_users");
        router.add_route("POST", "/users", "create_user");

        let handler = router.match_route("GET", "/users");
        assert_eq!(handler, Some("list_users".to_string()));
    }

    #[test]
    fn test_match_route_post() {
        let mut router = Router::new();
        router.add_route("GET", "/users", "list_users");
        router.add_route("POST", "/users", "create_user");

        let handler = router.match_route("POST", "/users");
        assert_eq!(handler, Some("create_user".to_string()));
    }

    #[test]
    fn test_match_route_not_found_wrong_method() {
        let mut router = Router::new();
        router.add_route("GET", "/users", "list_users");

        let handler = router.match_route("PUT", "/users");
        assert_eq!(handler, None);
    }

    #[test]
    fn test_match_route_not_found_wrong_path() {
        let mut router = Router::new();
        router.add_route("GET", "/users", "list_users");

        let handler = router.match_route("GET", "/posts");
        assert_eq!(handler, None);
    }

    #[test]
    fn test_match_route_empty_router() {
        let router = Router::new();
        let handler = router.match_route("GET", "/anything");
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
    fn test_success_response_empty_data() {
        let resp = APIResponse::success("");
        assert_eq!(resp.status, 200);
        assert_eq!(resp.data, Some("".to_string()));
        assert!(resp.error.is_empty());
    }

    #[test]
    fn test_error_response_400() {
        let resp = APIResponse::error(400, "Bad request");
        assert_eq!(resp.status, 400);
        assert_eq!(resp.error, "Bad request");
        assert!(resp.data.is_none());
    }

    #[test]
    fn test_error_response_404() {
        let resp = APIResponse::error(404, "Not found");
        assert_eq!(resp.status, 404);
        assert_eq!(resp.error, "Not found");
        assert!(resp.data.is_none());
    }

    #[test]
    fn test_error_response_500() {
        let resp = APIResponse::error(500, "Internal server error");
        assert_eq!(resp.status, 500);
        assert_eq!(resp.error, "Internal server error");
        assert!(resp.data.is_none());
    }

    #[test]
    fn test_success_to_json() {
        let resp = APIResponse::success("hello");
        let json = resp.to_json();
        assert!(json.contains("200"));
        assert!(json.contains("hello"));
        assert!(json.contains("status"));
    }

    #[test]
    fn test_error_to_json() {
        let resp = APIResponse::error(404, "Resource not found");
        let json = resp.to_json();
        assert!(json.contains("404"));
        assert!(json.contains("Resource not found"));
    }

    #[test]
    fn test_escape_json_string() {
        assert_eq!(escape_json_string("hello"), "hello");
        assert_eq!(escape_json_string("hello\"world"), "hello\\\"world");
        assert_eq!(escape_json_string("line1\nline2"), "line1\\nline2");
        assert_eq!(escape_json_string("path\\to\\file"), "path\\\\to\\\\file");
    }
}

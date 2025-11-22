import java.util.ArrayList;
import java.util.List;

/**
 * Chapter 8: Realm of APIs - REST & GraphQL API Design
 *
 * Implement API routing and response handling concepts:
 * - Route: Represents a single API endpoint
 * - Router: Manages multiple routes and matches incoming requests
 * - APIResponse: Standardized API response format
 */

/**
 * Represents a single API route with method, path, and handler.
 */
class Route {
    public String method;
    public String path;
    public String handler;

    public Route(String method, String path, String handler) {
        this.method = method;
        this.path = path;
        this.handler = handler;
    }
}

/**
 * Router class for managing API routes.
 */
class Router {
    private List<Route> routes;

    public Router() {
        this.routes = new ArrayList<>();
    }

    /**
     * Adds a new route to the router.
     *
     * @param method HTTP method (GET, POST, PUT, DELETE, etc.)
     * @param path URL path for the route
     * @param handler Handler identifier for this route
     */
    public void addRoute(String method, String path, String handler) {
        // TODO: Add the route to the routes list
        // Hint: Store the method in uppercase for consistent matching
        throw new UnsupportedOperationException("addRoute not implemented yet");
    }

    /**
     * Matches an incoming request to a registered route.
     *
     * @param method HTTP method of the request
     * @param path URL path of the request
     * @return Handler string if matched, null otherwise
     */
    public String match(String method, String path) {
        // TODO: Find a route matching the method and path
        // Hint: Compare methods case-insensitively
        // Hint: Path should match exactly
        throw new UnsupportedOperationException("match not implemented yet");
    }

    /**
     * Returns all registered routes.
     *
     * @return List of all routes
     */
    public List<Route> getRoutes() {
        // TODO: Return the list of routes
        throw new UnsupportedOperationException("getRoutes not implemented yet");
    }
}

/**
 * Standardized API response class.
 */
class APIResponse {
    public int status;
    public Object data;
    public String error;

    public APIResponse(int status, Object data, String error) {
        this.status = status;
        this.data = data;
        this.error = error;
    }

    /**
     * Creates a successful API response.
     *
     * @param data Response data
     * @return APIResponse with status 200 and the provided data
     */
    public static APIResponse success(Object data) {
        // TODO: Return a new APIResponse with status 200, data, and null error
        throw new UnsupportedOperationException("success not implemented yet");
    }

    /**
     * Creates an error API response.
     *
     * @param status HTTP status code
     * @param message Error message
     * @return APIResponse with the given status, null data, and error message
     */
    public static APIResponse error(int status, String message) {
        // TODO: Return a new APIResponse with the given status, null data, and error message
        throw new UnsupportedOperationException("error not implemented yet");
    }

    /**
     * Converts the response to a JSON string.
     *
     * @return JSON representation of the response
     */
    public String toJSON() {
        // TODO: Return a JSON string representation
        // Format: {"status":200,"data":"...","error":null}
        // Hint: Handle null values appropriately
        throw new UnsupportedOperationException("toJSON not implemented yet");
    }
}

/**
 * Main class for Chapter 8 API Design
 */
public class API {
    public static void main(String[] args) {
        System.out.println("Chapter 8: Realm of APIs - REST & GraphQL API Design");
        System.out.println("Implement the Router and APIResponse classes!");
    }
}

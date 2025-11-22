#include <string>
#include <vector>
#include <stdexcept>

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
struct Route {
    std::string method;
    std::string path;
    std::string handler;

    Route(const std::string& m, const std::string& p, const std::string& h)
        : method(m), path(p), handler(h) {}
};

/**
 * Router class for managing API routes.
 */
class Router {
private:
    std::vector<Route> routes;

public:
    Router() {}

    /**
     * Adds a new route to the router.
     *
     * @param method HTTP method (GET, POST, PUT, DELETE, etc.)
     * @param path URL path for the route
     * @param handler Handler identifier for this route
     */
    void addRoute(const std::string& method, const std::string& path, const std::string& handler) {
        // TODO: Add the route to the routes vector
        // Hint: Store the method in uppercase for consistent matching
        throw std::runtime_error("addRoute not implemented yet");
    }

    /**
     * Matches an incoming request to a registered route.
     *
     * @param method HTTP method of the request
     * @param path URL path of the request
     * @return Handler string if matched, empty string otherwise
     */
    std::string match(const std::string& method, const std::string& path) {
        // TODO: Find a route matching the method and path
        // Hint: Compare methods case-insensitively
        // Hint: Path should match exactly
        // Hint: Return empty string "" if no match found
        throw std::runtime_error("match not implemented yet");
    }

    /**
     * Returns all registered routes.
     *
     * @return Vector of all routes
     */
    std::vector<Route> getRoutes() const {
        // TODO: Return the vector of routes
        throw std::runtime_error("getRoutes not implemented yet");
    }
};

/**
 * Standardized API response class.
 */
class APIResponse {
public:
    int status;
    std::string data;
    std::string errorMsg;
    bool hasData;
    bool hasError;

    APIResponse(int s, const std::string& d, const std::string& e, bool hd, bool he)
        : status(s), data(d), errorMsg(e), hasData(hd), hasError(he) {}

    /**
     * Creates a successful API response.
     *
     * @param data Response data
     * @return APIResponse with status 200 and the provided data
     */
    static APIResponse success(const std::string& data) {
        // TODO: Return a new APIResponse with status 200, data, and no error
        throw std::runtime_error("success not implemented yet");
    }

    /**
     * Creates an error API response.
     *
     * @param status HTTP status code
     * @param message Error message
     * @return APIResponse with the given status, no data, and error message
     */
    static APIResponse error(int status, const std::string& message) {
        // TODO: Return a new APIResponse with the given status, no data, and error message
        throw std::runtime_error("error not implemented yet");
    }

    /**
     * Converts the response to a JSON string.
     *
     * @return JSON representation of the response
     */
    std::string toJSON() const {
        // TODO: Return a JSON string representation
        // Format: {"status":200,"data":"...","error":null}
        // Hint: Handle null values appropriately
        throw std::runtime_error("toJSON not implemented yet");
    }
};

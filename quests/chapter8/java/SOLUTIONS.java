import java.util.ArrayList;
import java.util.List;

/**
 * Reference solutions for Chapter 8: Realm of APIs
 */

/**
 * Represents a single API route with method, path, and handler.
 */
class SolRoute {
    public String method;
    public String path;
    public String handler;

    public SolRoute(String method, String path, String handler) {
        this.method = method.toUpperCase();
        this.path = path;
        this.handler = handler;
    }
}

/**
 * Router class for managing API routes.
 */
class SolRouter {
    private List<SolRoute> routes;

    public SolRouter() {
        this.routes = new ArrayList<>();
    }

    /**
     * Adds a new route to the router.
     */
    public void addRoute(String method, String path, String handler) {
        routes.add(new SolRoute(method, path, handler));
    }

    /**
     * Matches an incoming request to a registered route.
     */
    public String match(String method, String path) {
        String upperMethod = method.toUpperCase();
        for (SolRoute route : routes) {
            if (route.method.equals(upperMethod) && route.path.equals(path)) {
                return route.handler;
            }
        }
        return null;
    }

    /**
     * Returns all registered routes.
     */
    public List<SolRoute> getRoutes() {
        return routes;
    }
}

/**
 * Standardized API response class.
 */
class SolAPIResponse {
    public int status;
    public Object data;
    public String error;

    public SolAPIResponse(int status, Object data, String error) {
        this.status = status;
        this.data = data;
        this.error = error;
    }

    /**
     * Creates a successful API response.
     */
    public static SolAPIResponse success(Object data) {
        return new SolAPIResponse(200, data, null);
    }

    /**
     * Creates an error API response.
     */
    public static SolAPIResponse error(int status, String message) {
        return new SolAPIResponse(status, null, message);
    }

    /**
     * Converts the response to a JSON string.
     */
    public String toJSON() {
        StringBuilder sb = new StringBuilder();
        sb.append("{");
        sb.append("\"status\":").append(status);
        sb.append(",\"data\":");
        if (data == null) {
            sb.append("null");
        } else if (data instanceof String) {
            sb.append("\"").append(escapeJson((String) data)).append("\"");
        } else if (data instanceof Number || data instanceof Boolean) {
            sb.append(data);
        } else {
            sb.append("\"").append(escapeJson(data.toString())).append("\"");
        }
        sb.append(",\"error\":");
        if (error == null) {
            sb.append("null");
        } else {
            sb.append("\"").append(escapeJson(error)).append("\"");
        }
        sb.append("}");
        return sb.toString();
    }

    /**
     * Escapes special characters in a JSON string.
     */
    private String escapeJson(String input) {
        if (input == null) return "";
        return input
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\t", "\\t");
    }
}

/**
 * Main class for testing solutions
 */
public class SOLUTIONS {
    public static void main(String[] args) {
        // Test Router
        SolRouter router = new SolRouter();
        router.addRoute("GET", "/users", "listUsers");
        router.addRoute("POST", "/users", "createUser");
        router.addRoute("GET", "/users/123", "getUser");

        System.out.println("Router tests:");
        System.out.println("GET /users -> " + router.match("GET", "/users"));
        System.out.println("POST /users -> " + router.match("POST", "/users"));
        System.out.println("get /users -> " + router.match("get", "/users"));
        System.out.println("DELETE /users -> " + router.match("DELETE", "/users"));

        // Test APIResponse
        System.out.println("\nAPIResponse tests:");
        SolAPIResponse successResp = SolAPIResponse.success("User created");
        System.out.println("Success: " + successResp.toJSON());

        SolAPIResponse errorResp = SolAPIResponse.error(404, "Not found");
        System.out.println("Error: " + errorResp.toJSON());

        SolAPIResponse numResp = SolAPIResponse.success(42);
        System.out.println("Number: " + numResp.toJSON());
    }
}

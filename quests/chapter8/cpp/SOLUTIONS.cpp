#include <string>
#include <vector>
#include <algorithm>
#include <cctype>
#include <sstream>

/**
 * Reference solutions for Chapter 8: Realm of APIs
 */

/**
 * Converts a string to uppercase.
 */
std::string toUpperCase(const std::string& str) {
    std::string result = str;
    std::transform(result.begin(), result.end(), result.begin(), ::toupper);
    return result;
}

/**
 * Escapes special characters in a JSON string.
 */
std::string escapeJson(const std::string& input) {
    std::string result;
    for (char c : input) {
        switch (c) {
            case '\\': result += "\\\\"; break;
            case '"': result += "\\\""; break;
            case '\n': result += "\\n"; break;
            case '\r': result += "\\r"; break;
            case '\t': result += "\\t"; break;
            default: result += c;
        }
    }
    return result;
}

/**
 * Represents a single API route with method, path, and handler.
 */
struct SolRoute {
    std::string method;
    std::string path;
    std::string handler;

    SolRoute(const std::string& m, const std::string& p, const std::string& h)
        : method(toUpperCase(m)), path(p), handler(h) {}
};

/**
 * Router class for managing API routes.
 */
class SolRouter {
private:
    std::vector<SolRoute> routes;

public:
    SolRouter() {}

    /**
     * Adds a new route to the router.
     */
    void addRoute(const std::string& method, const std::string& path, const std::string& handler) {
        routes.push_back(SolRoute(method, path, handler));
    }

    /**
     * Matches an incoming request to a registered route.
     */
    std::string match(const std::string& method, const std::string& path) {
        std::string upperMethod = toUpperCase(method);
        for (const auto& route : routes) {
            if (route.method == upperMethod && route.path == path) {
                return route.handler;
            }
        }
        return "";  // No match found
    }

    /**
     * Returns all registered routes.
     */
    std::vector<SolRoute> getRoutes() const {
        return routes;
    }
};

/**
 * Standardized API response class.
 */
class SolAPIResponse {
public:
    int status;
    std::string data;
    std::string errorMsg;
    bool hasData;
    bool hasError;

    SolAPIResponse(int s, const std::string& d, const std::string& e, bool hd, bool he)
        : status(s), data(d), errorMsg(e), hasData(hd), hasError(he) {}

    /**
     * Creates a successful API response.
     */
    static SolAPIResponse success(const std::string& data) {
        return SolAPIResponse(200, data, "", true, false);
    }

    /**
     * Creates an error API response.
     */
    static SolAPIResponse error(int status, const std::string& message) {
        return SolAPIResponse(status, "", message, false, true);
    }

    /**
     * Converts the response to a JSON string.
     */
    std::string toJSON() const {
        std::ostringstream ss;
        ss << "{";
        ss << "\"status\":" << status;
        ss << ",\"data\":";
        if (hasData) {
            ss << "\"" << escapeJson(data) << "\"";
        } else {
            ss << "null";
        }
        ss << ",\"error\":";
        if (hasError) {
            ss << "\"" << escapeJson(errorMsg) << "\"";
        } else {
            ss << "null";
        }
        ss << "}";
        return ss.str();
    }
};

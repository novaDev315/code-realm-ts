// Reference solutions for Chapter 9: Dungeon of DevOps - Docker & Configuration
// Configuration parsing, health checks, and environment handling

#include <string>
#include <map>
#include <sstream>
#include <cstdlib>

/**
 * Configuration struct to hold application settings - SOLUTION
 */
struct Config {
    std::string host;
    int port;
    bool debug;

    Config() : host(""), port(0), debug(false) {}
    Config(const std::string& h, int p, bool d) : host(h), port(p), debug(d) {}
};

/**
 * Parse a configuration string into a Config object - SOLUTION
 *
 * Parses newline-separated key=value pairs and extracts
 * host, port, and debug values.
 *
 * @param configStr The configuration string with key=value pairs
 * @return A Config object with parsed values
 */
Config parseConfig(const std::string& configStr) {
    Config config;

    if (configStr.empty()) {
        return config;
    }

    // Use stringstream to split by newline
    std::istringstream stream(configStr);
    std::string line;

    while (std::getline(stream, line)) {
        // Skip empty lines
        if (line.empty()) {
            continue;
        }

        // Find the '=' separator
        size_t equalsPos = line.find('=');
        if (equalsPos == std::string::npos) {
            continue; // Skip lines without '='
        }

        // Extract key and value
        std::string key = line.substr(0, equalsPos);
        std::string value = line.substr(equalsPos + 1);

        // Trim whitespace (simple implementation)
        while (!key.empty() && (key.back() == ' ' || key.back() == '\t')) {
            key.pop_back();
        }
        while (!value.empty() && (value.front() == ' ' || value.front() == '\t')) {
            value.erase(0, 1);
        }

        // Match keys to Config fields
        if (key == "host") {
            config.host = value;
        } else if (key == "port") {
            try {
                config.port = std::stoi(value);
            } catch (...) {
                // Keep default value of 0 on parse error
            }
        } else if (key == "debug") {
            config.debug = (value == "true" || value == "1" || value == "yes");
        }
        // Ignore unknown keys
    }

    return config;
}

/**
 * Perform health checks on a set of services - SOLUTION
 *
 * Converts boolean health status to human-readable strings.
 *
 * @param services Map of service names to their health status (true/false)
 * @return Map of service names to status strings ("healthy"/"unhealthy")
 */
std::map<std::string, std::string> healthCheck(const std::map<std::string, bool>& services) {
    std::map<std::string, std::string> result;

    for (const auto& entry : services) {
        const std::string& serviceName = entry.first;
        bool isHealthy = entry.second;

        // Convert boolean to status string
        std::string status = isHealthy ? "healthy" : "unhealthy";
        result[serviceName] = status;
    }

    return result;
}

/**
 * Get an environment variable with a default fallback - SOLUTION
 *
 * Retrieves environment variable value or returns default if not set.
 *
 * @param key The environment variable name
 * @param defaultValue The value to return if env var is not set
 * @return The environment variable value or the default
 */
std::string getEnvWithDefault(const std::string& key, const std::string& defaultValue) {
    // Get the environment variable
    const char* value = std::getenv(key.c_str());

    // Return default if null or empty
    if (value == nullptr || value[0] == '\0') {
        return defaultValue;
    }

    return std::string(value);
}

/**
 * Validate that a port number is within valid range - SOLUTION
 *
 * Valid TCP/UDP ports range from 1 to 65535.
 * Port 0 is reserved and cannot be used.
 *
 * @param port The port number to validate
 * @return true if valid, false otherwise
 */
bool validatePort(int port) {
    return port >= 1 && port <= 65535;
}

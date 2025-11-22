// Chapter 9: Dungeon of DevOps - Docker & Configuration
// Your task: Implement configuration parsing, health checks, and environment handling

import java.util.Map;
import java.util.HashMap;

public class DevOps {

    /**
     * Configuration class to hold application settings
     *
     * In DevOps, configuration management is essential for:
     * - Separating code from environment-specific settings
     * - Enabling different configurations for dev/staging/production
     * - Managing sensitive data through environment variables
     */
    public static class Config {
        public String host;
        public int port;
        public boolean debug;

        public Config() {
            this.host = "";
            this.port = 0;
            this.debug = false;
        }

        public Config(String host, int port, boolean debug) {
            this.host = host;
            this.port = port;
            this.debug = debug;
        }
    }

    /**
     * Parse a configuration string into a Config object
     *
     * Configuration files often use key=value format for simplicity.
     * This is common in .env files, Docker configurations, and system configs.
     *
     * TODO: Implement configuration parsing
     * - Parse newline-separated key=value pairs
     * - Extract "host", "port", and "debug" values
     * - Convert port to integer and debug to boolean
     * - Handle missing keys gracefully (use defaults: "", 0, false)
     *
     * Example:
     *   parseConfig("host=localhost\nport=8080\ndebug=true")
     *   -> Config { host: "localhost", port: 8080, debug: true }
     *
     *   parseConfig("host=api.example.com\nport=443")
     *   -> Config { host: "api.example.com", port: 443, debug: false }
     *
     * @param configStr The configuration string with key=value pairs
     * @return A Config object with parsed values
     */
    public static Config parseConfig(String configStr) {
        // TODO: Implement configuration parsing
        // Hint:
        // 1. Split by newline to get individual lines
        // 2. For each line, split by "=" to get key and value
        // 3. Match keys to Config fields
        // 4. Handle type conversions (Integer.parseInt, Boolean.parseBoolean)
        throw new UnsupportedOperationException("Not implemented yet");
    }

    /**
     * Perform health checks on a set of services
     *
     * Health checks are critical in containerized environments:
     * - Kubernetes uses them to determine pod readiness
     * - Load balancers use them to route traffic
     * - Orchestrators use them for auto-healing
     *
     * TODO: Implement health check aggregation
     * - Take a map of service names to their health status (true = healthy)
     * - Return a map with the same service names
     * - Values should be "healthy" or "unhealthy" strings
     *
     * Example:
     *   healthCheck({"db": true, "cache": false, "api": true})
     *   -> {"db": "healthy", "cache": "unhealthy", "api": "healthy"}
     *
     * @param services Map of service names to their health status (true/false)
     * @return Map of service names to status strings ("healthy"/"unhealthy")
     */
    public static Map<String, String> healthCheck(Map<String, Boolean> services) {
        // TODO: Implement health check status conversion
        // Hint:
        // 1. Create a new HashMap for results
        // 2. Iterate through each service entry
        // 3. Convert boolean to "healthy" or "unhealthy"
        // 4. Return the result map
        throw new UnsupportedOperationException("Not implemented yet");
    }

    /**
     * Get an environment variable with a default fallback
     *
     * Environment variables are the preferred way to configure containers:
     * - Docker injects them via -e or env_file
     * - Kubernetes uses ConfigMaps and Secrets
     * - They keep sensitive data out of code
     *
     * TODO: Implement environment variable retrieval with default
     * - Try to get the environment variable by key
     * - If not found or empty, return the default value
     * - If found, return the actual value
     *
     * Example:
     *   getEnvWithDefault("PORT", "3000") -> "8080" (if PORT=8080 is set)
     *   getEnvWithDefault("MISSING_VAR", "default") -> "default"
     *
     * @param key The environment variable name
     * @param defaultValue The value to return if env var is not set
     * @return The environment variable value or the default
     */
    public static String getEnvWithDefault(String key, String defaultValue) {
        // TODO: Implement environment variable lookup with default
        // Hint:
        // 1. Use System.getenv(key) to get the environment variable
        // 2. Check if the result is null or empty
        // 3. Return default if null/empty, otherwise return the value
        throw new UnsupportedOperationException("Not implemented yet");
    }

    /**
     * Validate that a port number is within valid range
     *
     * Port validation is important for network configurations:
     * - Valid ports range from 1 to 65535
     * - Ports 1-1023 are privileged (require root on Unix)
     * - Common application ports: 80 (HTTP), 443 (HTTPS), 8080, 3000
     *
     * TODO: Implement port validation
     * - Return true if port is between 1 and 65535 (inclusive)
     * - Return false otherwise
     *
     * Example:
     *   validatePort(8080) -> true
     *   validatePort(0) -> false
     *   validatePort(-1) -> false
     *   validatePort(65536) -> false
     *   validatePort(443) -> true
     *
     * @param port The port number to validate
     * @return true if valid, false otherwise
     */
    public static boolean validatePort(int port) {
        // TODO: Implement port validation
        // Hint: Check if port >= 1 && port <= 65535
        throw new UnsupportedOperationException("Not implemented yet");
    }
}

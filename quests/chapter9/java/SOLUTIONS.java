// Reference solutions for Chapter 9: Dungeon of DevOps - Docker & Configuration
// Configuration parsing, health checks, and environment handling

import java.util.Map;
import java.util.HashMap;

public class SOLUTIONS {

    /**
     * Configuration class to hold application settings - SOLUTION
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
     * Parse a configuration string into a Config object - SOLUTION
     *
     * Parses newline-separated key=value pairs and extracts
     * host, port, and debug values.
     *
     * @param configStr The configuration string with key=value pairs
     * @return A Config object with parsed values
     */
    public static Config parseConfig(String configStr) {
        Config config = new Config();

        if (configStr == null || configStr.isEmpty()) {
            return config;
        }

        // Split by newline to get individual lines
        String[] lines = configStr.split("\n");

        for (String line : lines) {
            // Skip empty lines
            if (line.trim().isEmpty()) {
                continue;
            }

            // Split by '=' to get key and value
            int equalsIndex = line.indexOf('=');
            if (equalsIndex == -1) {
                continue; // Skip lines without '='
            }

            String key = line.substring(0, equalsIndex).trim();
            String value = line.substring(equalsIndex + 1).trim();

            // Match keys to Config fields
            switch (key) {
                case "host":
                    config.host = value;
                    break;
                case "port":
                    try {
                        config.port = Integer.parseInt(value);
                    } catch (NumberFormatException e) {
                        // Keep default value of 0
                    }
                    break;
                case "debug":
                    config.debug = Boolean.parseBoolean(value);
                    break;
                // Ignore unknown keys
            }
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
    public static Map<String, String> healthCheck(Map<String, Boolean> services) {
        Map<String, String> result = new HashMap<>();

        if (services == null) {
            return result;
        }

        for (Map.Entry<String, Boolean> entry : services.entrySet()) {
            String serviceName = entry.getKey();
            Boolean isHealthy = entry.getValue();

            // Convert boolean to status string
            String status = (isHealthy != null && isHealthy) ? "healthy" : "unhealthy";
            result.put(serviceName, status);
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
    public static String getEnvWithDefault(String key, String defaultValue) {
        // Get the environment variable
        String value = System.getenv(key);

        // Return default if null or empty
        if (value == null || value.isEmpty()) {
            return defaultValue;
        }

        return value;
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
    public static boolean validatePort(int port) {
        return port >= 1 && port <= 65535;
    }
}

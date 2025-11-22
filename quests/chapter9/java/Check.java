// Test file for Chapter 9: Dungeon of DevOps - Docker & Configuration

import java.util.Map;
import java.util.HashMap;

public class Check {
    public static void main(String[] args) {
        boolean passed = true;

        // Test 1: parseConfig
        System.out.println("Testing parseConfig...");

        // Test basic configuration parsing
        String configStr1 = "host=localhost\nport=8080\ndebug=true";
        DevOps.Config config1 = DevOps.parseConfig(configStr1);
        if (!"localhost".equals(config1.host)) {
            System.err.println("parseConfig: expected host='localhost', got '" + config1.host + "'");
            passed = false;
        } else {
            System.out.println("  parseConfig parsed host correctly: " + config1.host);
        }

        if (config1.port != 8080) {
            System.err.println("parseConfig: expected port=8080, got " + config1.port);
            passed = false;
        } else {
            System.out.println("  parseConfig parsed port correctly: " + config1.port);
        }

        if (!config1.debug) {
            System.err.println("parseConfig: expected debug=true, got " + config1.debug);
            passed = false;
        } else {
            System.out.println("  parseConfig parsed debug correctly: " + config1.debug);
        }

        // Test production-like configuration
        String configStr2 = "host=api.example.com\nport=443\ndebug=false";
        DevOps.Config config2 = DevOps.parseConfig(configStr2);
        if (!"api.example.com".equals(config2.host) || config2.port != 443 || config2.debug) {
            System.err.println("parseConfig: failed to parse production config");
            passed = false;
        } else {
            System.out.println("  parseConfig handles production config correctly");
        }

        // Test partial configuration (missing debug)
        String configStr3 = "host=test.local\nport=3000";
        DevOps.Config config3 = DevOps.parseConfig(configStr3);
        if (!"test.local".equals(config3.host) || config3.port != 3000) {
            System.err.println("parseConfig: failed to parse partial config");
            passed = false;
        } else {
            System.out.println("  parseConfig handles partial config correctly");
        }

        // Test 2: healthCheck
        System.out.println("\nTesting healthCheck...");

        Map<String, Boolean> services1 = new HashMap<>();
        services1.put("database", true);
        services1.put("cache", false);
        services1.put("api", true);

        Map<String, String> health1 = DevOps.healthCheck(services1);

        if (!"healthy".equals(health1.get("database"))) {
            System.err.println("healthCheck: database should be 'healthy'");
            passed = false;
        } else {
            System.out.println("  healthCheck(database=true) = 'healthy'");
        }

        if (!"unhealthy".equals(health1.get("cache"))) {
            System.err.println("healthCheck: cache should be 'unhealthy'");
            passed = false;
        } else {
            System.out.println("  healthCheck(cache=false) = 'unhealthy'");
        }

        if (!"healthy".equals(health1.get("api"))) {
            System.err.println("healthCheck: api should be 'healthy'");
            passed = false;
        } else {
            System.out.println("  healthCheck(api=true) = 'healthy'");
        }

        // Test all healthy services
        Map<String, Boolean> services2 = new HashMap<>();
        services2.put("web", true);
        services2.put("worker", true);

        Map<String, String> health2 = DevOps.healthCheck(services2);
        if (!"healthy".equals(health2.get("web")) || !"healthy".equals(health2.get("worker"))) {
            System.err.println("healthCheck: all services should be 'healthy'");
            passed = false;
        } else {
            System.out.println("  healthCheck handles all healthy services correctly");
        }

        // Test empty services
        Map<String, Boolean> services3 = new HashMap<>();
        Map<String, String> health3 = DevOps.healthCheck(services3);
        if (!health3.isEmpty()) {
            System.err.println("healthCheck: empty input should return empty map");
            passed = false;
        } else {
            System.out.println("  healthCheck handles empty services correctly");
        }

        // Test 3: getEnvWithDefault
        System.out.println("\nTesting getEnvWithDefault...");

        // Test with existing environment variable (PATH should exist)
        String pathValue = DevOps.getEnvWithDefault("PATH", "/default/path");
        if (pathValue.equals("/default/path")) {
            System.out.println("  Note: PATH env var might be empty, using default");
        } else {
            System.out.println("  getEnvWithDefault(\"PATH\", ...) returns actual PATH value");
        }

        // Test with non-existent environment variable
        String missingValue = DevOps.getEnvWithDefault("DEFINITELY_NOT_SET_12345", "fallback");
        if (!"fallback".equals(missingValue)) {
            System.err.println("getEnvWithDefault: should return default for missing var, got '" + missingValue + "'");
            passed = false;
        } else {
            System.out.println("  getEnvWithDefault for missing var returns 'fallback'");
        }

        // Test with another non-existent variable
        String defaultPort = DevOps.getEnvWithDefault("MY_CUSTOM_PORT_XYZ", "3000");
        if (!"3000".equals(defaultPort)) {
            System.err.println("getEnvWithDefault: expected '3000', got '" + defaultPort + "'");
            passed = false;
        } else {
            System.out.println("  getEnvWithDefault for custom var returns default '3000'");
        }

        // Test 4: validatePort
        System.out.println("\nTesting validatePort...");

        // Valid ports
        if (!DevOps.validatePort(80)) {
            System.err.println("validatePort(80): should return true");
            passed = false;
        } else {
            System.out.println("  validatePort(80) = true");
        }

        if (!DevOps.validatePort(443)) {
            System.err.println("validatePort(443): should return true");
            passed = false;
        } else {
            System.out.println("  validatePort(443) = true");
        }

        if (!DevOps.validatePort(8080)) {
            System.err.println("validatePort(8080): should return true");
            passed = false;
        } else {
            System.out.println("  validatePort(8080) = true");
        }

        if (!DevOps.validatePort(65535)) {
            System.err.println("validatePort(65535): should return true");
            passed = false;
        } else {
            System.out.println("  validatePort(65535) = true");
        }

        if (!DevOps.validatePort(1)) {
            System.err.println("validatePort(1): should return true");
            passed = false;
        } else {
            System.out.println("  validatePort(1) = true");
        }

        // Invalid ports
        if (DevOps.validatePort(0)) {
            System.err.println("validatePort(0): should return false");
            passed = false;
        } else {
            System.out.println("  validatePort(0) = false");
        }

        if (DevOps.validatePort(-1)) {
            System.err.println("validatePort(-1): should return false");
            passed = false;
        } else {
            System.out.println("  validatePort(-1) = false");
        }

        if (DevOps.validatePort(65536)) {
            System.err.println("validatePort(65536): should return false");
            passed = false;
        } else {
            System.out.println("  validatePort(65536) = false");
        }

        if (DevOps.validatePort(100000)) {
            System.err.println("validatePort(100000): should return false");
            passed = false;
        } else {
            System.out.println("  validatePort(100000) = false");
        }

        // Summary
        System.out.println("\n" + "=".repeat(50));
        if (passed) {
            System.out.println("All DevOps tests passed!");
        } else {
            System.out.println("Some DevOps tests failed.");
            System.exit(1);
        }
    }
}

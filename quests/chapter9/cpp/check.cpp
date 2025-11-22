// Test file for Chapter 9: Dungeon of DevOps - Docker & Configuration

#include <iostream>
#include <string>
#include <map>
#include "devops.cpp"

using namespace std;

int main() {
    bool passed = true;

    // Test 1: parseConfig
    cout << "Testing parseConfig..." << endl;

    // Test basic configuration parsing
    string configStr1 = "host=localhost\nport=8080\ndebug=true";
    Config config1 = parseConfig(configStr1);
    if (config1.host != "localhost") {
        cerr << "parseConfig: expected host='localhost', got '" << config1.host << "'" << endl;
        passed = false;
    } else {
        cout << "  parseConfig parsed host correctly: " << config1.host << endl;
    }

    if (config1.port != 8080) {
        cerr << "parseConfig: expected port=8080, got " << config1.port << endl;
        passed = false;
    } else {
        cout << "  parseConfig parsed port correctly: " << config1.port << endl;
    }

    if (!config1.debug) {
        cerr << "parseConfig: expected debug=true, got " << (config1.debug ? "true" : "false") << endl;
        passed = false;
    } else {
        cout << "  parseConfig parsed debug correctly: " << (config1.debug ? "true" : "false") << endl;
    }

    // Test production-like configuration
    string configStr2 = "host=api.example.com\nport=443\ndebug=false";
    Config config2 = parseConfig(configStr2);
    if (config2.host != "api.example.com" || config2.port != 443 || config2.debug) {
        cerr << "parseConfig: failed to parse production config" << endl;
        passed = false;
    } else {
        cout << "  parseConfig handles production config correctly" << endl;
    }

    // Test partial configuration (missing debug)
    string configStr3 = "host=test.local\nport=3000";
    Config config3 = parseConfig(configStr3);
    if (config3.host != "test.local" || config3.port != 3000) {
        cerr << "parseConfig: failed to parse partial config" << endl;
        passed = false;
    } else {
        cout << "  parseConfig handles partial config correctly" << endl;
    }

    // Test 2: healthCheck
    cout << "\nTesting healthCheck..." << endl;

    map<string, bool> services1;
    services1["database"] = true;
    services1["cache"] = false;
    services1["api"] = true;

    map<string, string> health1 = healthCheck(services1);

    if (health1["database"] != "healthy") {
        cerr << "healthCheck: database should be 'healthy'" << endl;
        passed = false;
    } else {
        cout << "  healthCheck(database=true) = 'healthy'" << endl;
    }

    if (health1["cache"] != "unhealthy") {
        cerr << "healthCheck: cache should be 'unhealthy'" << endl;
        passed = false;
    } else {
        cout << "  healthCheck(cache=false) = 'unhealthy'" << endl;
    }

    if (health1["api"] != "healthy") {
        cerr << "healthCheck: api should be 'healthy'" << endl;
        passed = false;
    } else {
        cout << "  healthCheck(api=true) = 'healthy'" << endl;
    }

    // Test all healthy services
    map<string, bool> services2;
    services2["web"] = true;
    services2["worker"] = true;

    map<string, string> health2 = healthCheck(services2);
    if (health2["web"] != "healthy" || health2["worker"] != "healthy") {
        cerr << "healthCheck: all services should be 'healthy'" << endl;
        passed = false;
    } else {
        cout << "  healthCheck handles all healthy services correctly" << endl;
    }

    // Test empty services
    map<string, bool> services3;
    map<string, string> health3 = healthCheck(services3);
    if (!health3.empty()) {
        cerr << "healthCheck: empty input should return empty map" << endl;
        passed = false;
    } else {
        cout << "  healthCheck handles empty services correctly" << endl;
    }

    // Test 3: getEnvWithDefault
    cout << "\nTesting getEnvWithDefault..." << endl;

    // Test with existing environment variable (PATH should exist on most systems)
    string pathValue = getEnvWithDefault("PATH", "/default/path");
    if (pathValue == "/default/path") {
        cout << "  Note: PATH env var might be empty, using default" << endl;
    } else {
        cout << "  getEnvWithDefault(\"PATH\", ...) returns actual PATH value" << endl;
    }

    // Test with non-existent environment variable
    string missingValue = getEnvWithDefault("DEFINITELY_NOT_SET_12345", "fallback");
    if (missingValue != "fallback") {
        cerr << "getEnvWithDefault: should return default for missing var, got '" << missingValue << "'" << endl;
        passed = false;
    } else {
        cout << "  getEnvWithDefault for missing var returns 'fallback'" << endl;
    }

    // Test with another non-existent variable
    string defaultPort = getEnvWithDefault("MY_CUSTOM_PORT_XYZ", "3000");
    if (defaultPort != "3000") {
        cerr << "getEnvWithDefault: expected '3000', got '" << defaultPort << "'" << endl;
        passed = false;
    } else {
        cout << "  getEnvWithDefault for custom var returns default '3000'" << endl;
    }

    // Test 4: validatePort
    cout << "\nTesting validatePort..." << endl;

    // Valid ports
    if (!validatePort(80)) {
        cerr << "validatePort(80): should return true" << endl;
        passed = false;
    } else {
        cout << "  validatePort(80) = true" << endl;
    }

    if (!validatePort(443)) {
        cerr << "validatePort(443): should return true" << endl;
        passed = false;
    } else {
        cout << "  validatePort(443) = true" << endl;
    }

    if (!validatePort(8080)) {
        cerr << "validatePort(8080): should return true" << endl;
        passed = false;
    } else {
        cout << "  validatePort(8080) = true" << endl;
    }

    if (!validatePort(65535)) {
        cerr << "validatePort(65535): should return true" << endl;
        passed = false;
    } else {
        cout << "  validatePort(65535) = true" << endl;
    }

    if (!validatePort(1)) {
        cerr << "validatePort(1): should return true" << endl;
        passed = false;
    } else {
        cout << "  validatePort(1) = true" << endl;
    }

    // Invalid ports
    if (validatePort(0)) {
        cerr << "validatePort(0): should return false" << endl;
        passed = false;
    } else {
        cout << "  validatePort(0) = false" << endl;
    }

    if (validatePort(-1)) {
        cerr << "validatePort(-1): should return false" << endl;
        passed = false;
    } else {
        cout << "  validatePort(-1) = false" << endl;
    }

    if (validatePort(65536)) {
        cerr << "validatePort(65536): should return false" << endl;
        passed = false;
    } else {
        cout << "  validatePort(65536) = false" << endl;
    }

    if (validatePort(100000)) {
        cerr << "validatePort(100000): should return false" << endl;
        passed = false;
    } else {
        cout << "  validatePort(100000) = false" << endl;
    }

    // Summary
    cout << endl << string(50, '=') << endl;
    if (passed) {
        cout << "All DevOps tests passed!" << endl;
        return 0;
    } else {
        cout << "Some DevOps tests failed." << endl;
        return 1;
    }
}

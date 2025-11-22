#include <iostream>
#include <string>
#include <vector>
#include "SOLUTIONS.cpp"

/**
 * Test suite for Chapter 8: Realm of APIs
 */

int passed = 0;
int failed = 0;

void assertMatch(const std::string& testName, const std::string& actual, const std::string& expected) {
    if (actual == expected) {
        std::cout << "  PASS: " << testName << std::endl;
        passed++;
    } else {
        std::cout << "  FAIL: " << testName << std::endl;
        std::cout << "    Expected: " << (expected.empty() ? "(empty)" : expected) << std::endl;
        std::cout << "    Actual:   " << (actual.empty() ? "(empty)" : actual) << std::endl;
        failed++;
    }
}

void assertTrue(const std::string& testName, bool condition) {
    if (condition) {
        std::cout << "  PASS: " << testName << std::endl;
        passed++;
    } else {
        std::cout << "  FAIL: " << testName << std::endl;
        failed++;
    }
}

void testRouter() {
    std::cout << "Testing Router...\n" << std::endl;

    // Test 1: Add and match routes
    SolRouter router;
    router.addRoute("GET", "/users", "listUsers");
    router.addRoute("POST", "/users", "createUser");
    router.addRoute("GET", "/users/123", "getUser");
    router.addRoute("DELETE", "/users/123", "deleteUser");

    // Test exact match
    assertMatch("GET /users match", router.match("GET", "/users"), "listUsers");
    assertMatch("POST /users match", router.match("POST", "/users"), "createUser");
    assertMatch("GET /users/123 match", router.match("GET", "/users/123"), "getUser");
    assertMatch("DELETE /users/123 match", router.match("DELETE", "/users/123"), "deleteUser");

    // Test case-insensitive method matching
    assertMatch("get (lowercase) match", router.match("get", "/users"), "listUsers");
    assertMatch("Post (mixed case) match", router.match("Post", "/users"), "createUser");

    // Test no match
    assertMatch("No match for PUT", router.match("PUT", "/users"), "");
    assertMatch("No match for wrong path", router.match("GET", "/products"), "");

    // Test getRoutes
    std::vector<SolRoute> routes = router.getRoutes();
    assertTrue("getRoutes returns routes", !routes.empty());
    assertTrue("getRoutes returns 4 routes", routes.size() == 4);
}

void testAPIResponse() {
    std::cout << "\nTesting APIResponse...\n" << std::endl;

    // Test success response
    SolAPIResponse successResp = SolAPIResponse::success("User created");
    assertTrue("success() sets status 200", successResp.status == 200);
    assertTrue("success() sets data", successResp.data == "User created");
    assertTrue("success() has data flag", successResp.hasData == true);
    assertTrue("success() no error flag", successResp.hasError == false);

    // Test error response
    SolAPIResponse errorResp = SolAPIResponse::error(404, "Not found");
    assertTrue("error() sets status", errorResp.status == 404);
    assertTrue("error() has no data flag", errorResp.hasData == false);
    assertTrue("error() has error flag", errorResp.hasError == true);
    assertTrue("error() sets error message", errorResp.errorMsg == "Not found");

    // Test different status codes
    SolAPIResponse badRequest = SolAPIResponse::error(400, "Bad request");
    assertTrue("error() 400 status", badRequest.status == 400);

    SolAPIResponse serverError = SolAPIResponse::error(500, "Internal error");
    assertTrue("error() 500 status", serverError.status == 500);

    // Test toJSON for success
    std::string successJson = successResp.toJSON();
    assertTrue("toJSON contains status", successJson.find("\"status\":200") != std::string::npos);
    assertTrue("toJSON contains data", successJson.find("\"data\":") != std::string::npos);
    assertTrue("toJSON contains error null", successJson.find("\"error\":null") != std::string::npos);

    // Test toJSON for error
    std::string errorJson = errorResp.toJSON();
    assertTrue("toJSON error status", errorJson.find("\"status\":404") != std::string::npos);
    assertTrue("toJSON data null", errorJson.find("\"data\":null") != std::string::npos);
    assertTrue("toJSON error message", errorJson.find("\"error\":\"Not found\"") != std::string::npos);

    // Test success with empty data
    SolAPIResponse emptyResp = SolAPIResponse::success("");
    assertTrue("success() with empty data", emptyResp.status == 200);
    std::string emptyJson = emptyResp.toJSON();
    assertTrue("toJSON with empty data", emptyJson.find("\"data\":\"\"") != std::string::npos);
}

int main() {
    testRouter();
    testAPIResponse();

    std::cout << "\n========================================" << std::endl;
    if (failed == 0) {
        std::cout << "All tests passed! (" << passed << "/" << (passed + failed) << ")" << std::endl;
        return 0;
    } else {
        std::cout << "Some tests failed. (" << passed << "/" << (passed + failed) << " passed)" << std::endl;
        return 1;
    }
}

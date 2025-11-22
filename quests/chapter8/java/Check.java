import java.util.List;

/**
 * Test suite for Chapter 8: Realm of APIs
 *
 * Run with: javac SOLUTIONS.java Check.java && java Check
 */
public class Check {

    private static int passed = 0;
    private static int failed = 0;

    public static void main(String[] args) {
        System.out.println("Testing Router...\n");
        testRouter();

        System.out.println("\nTesting APIResponse...\n");
        testAPIResponse();

        System.out.println("\n" + "=".repeat(40));
        if (failed == 0) {
            System.out.println("All tests passed! (" + passed + "/" + (passed + failed) + ")");
        } else {
            System.out.println("Some tests failed. (" + passed + "/" + (passed + failed) + " passed)");
            System.exit(1);
        }
    }

    private static void testRouter() {
        // Test 1: Add and match routes
        SolRouter router = new SolRouter();
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
        assertMatch("No match for PUT", router.match("PUT", "/users"), null);
        assertMatch("No match for wrong path", router.match("GET", "/products"), null);

        // Test getRoutes
        List<SolRoute> routes = router.getRoutes();
        assertTrue("getRoutes returns list", routes != null);
        assertTrue("getRoutes returns 4 routes", routes.size() == 4);
    }

    private static void testAPIResponse() {
        // Test success response
        SolAPIResponse successResp = SolAPIResponse.success("User created");
        assertTrue("success() sets status 200", successResp.status == 200);
        assertTrue("success() sets data", "User created".equals(successResp.data));
        assertTrue("success() sets error null", successResp.error == null);

        // Test error response
        SolAPIResponse errorResp = SolAPIResponse.error(404, "Not found");
        assertTrue("error() sets status", errorResp.status == 404);
        assertTrue("error() sets data null", errorResp.data == null);
        assertTrue("error() sets error message", "Not found".equals(errorResp.error));

        // Test different status codes
        SolAPIResponse badRequest = SolAPIResponse.error(400, "Bad request");
        assertTrue("error() 400 status", badRequest.status == 400);

        SolAPIResponse serverError = SolAPIResponse.error(500, "Internal error");
        assertTrue("error() 500 status", serverError.status == 500);

        // Test toJSON for success
        String successJson = successResp.toJSON();
        assertTrue("toJSON contains status", successJson.contains("\"status\":200"));
        assertTrue("toJSON contains data", successJson.contains("\"data\":"));
        assertTrue("toJSON contains error null", successJson.contains("\"error\":null"));

        // Test toJSON for error
        String errorJson = errorResp.toJSON();
        assertTrue("toJSON error status", errorJson.contains("\"status\":404"));
        assertTrue("toJSON data null", errorJson.contains("\"data\":null"));
        assertTrue("toJSON error message", errorJson.contains("\"error\":\"Not found\""));

        // Test success with number data
        SolAPIResponse numResp = SolAPIResponse.success(42);
        assertTrue("success() with number", numResp.status == 200);
        String numJson = numResp.toJSON();
        assertTrue("toJSON with number data", numJson.contains("\"data\":42"));

        // Test success with null data
        SolAPIResponse nullDataResp = SolAPIResponse.success(null);
        assertTrue("success() with null data status", nullDataResp.status == 200);
        String nullJson = nullDataResp.toJSON();
        assertTrue("toJSON with null data", nullJson.contains("\"data\":null"));
    }

    private static void assertMatch(String testName, String actual, String expected) {
        boolean match = (expected == null) ? (actual == null) : expected.equals(actual);
        if (match) {
            System.out.println("  PASS: " + testName);
            passed++;
        } else {
            System.out.println("  FAIL: " + testName);
            System.out.println("    Expected: " + expected);
            System.out.println("    Actual:   " + actual);
            failed++;
        }
    }

    private static void assertTrue(String testName, boolean condition) {
        if (condition) {
            System.out.println("  PASS: " + testName);
            passed++;
        } else {
            System.out.println("  FAIL: " + testName);
            failed++;
        }
    }
}

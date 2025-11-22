// Test suite for Chapter 7: Vault of Data - SQL Query Builder

public class Check {
    public static void main(String[] args) {
        boolean passed = true;

        System.out.println("Testing SQL Query Builder...\n");

        // Test 1: Basic SELECT query
        System.out.println("Test 1: Basic SELECT query");
        try {
            String query1 = new Database.QueryBuilder()
                .select("id", "name", "email")
                .from("users")
                .build();

            if (!containsKeywords(query1, "SELECT", "id", "name", "email", "FROM", "users")) {
                System.err.println("  X Basic SELECT failed. Got: " + query1);
                passed = false;
            } else {
                System.out.println("  + Basic SELECT passed: " + query1);
            }
        } catch (Exception e) {
            System.err.println("  X Basic SELECT threw exception: " + e.getMessage());
            passed = false;
        }

        // Test 2: SELECT with WHERE clause
        System.out.println("\nTest 2: SELECT with WHERE clause");
        try {
            String query2 = new Database.QueryBuilder()
                .select("*")
                .from("users")
                .where("active = true")
                .build();

            if (!containsKeywords(query2, "SELECT", "*", "FROM", "users", "WHERE", "active")) {
                System.err.println("  X SELECT with WHERE failed. Got: " + query2);
                passed = false;
            } else {
                System.out.println("  + SELECT with WHERE passed: " + query2);
            }
        } catch (Exception e) {
            System.err.println("  X SELECT with WHERE threw exception: " + e.getMessage());
            passed = false;
        }

        // Test 3: SELECT with ORDER BY
        System.out.println("\nTest 3: SELECT with ORDER BY");
        try {
            String query3 = new Database.QueryBuilder()
                .select("id", "name")
                .from("products")
                .orderBy("price", "DESC")
                .build();

            if (!containsKeywords(query3, "SELECT", "FROM", "products", "ORDER BY", "price", "DESC")) {
                System.err.println("  X SELECT with ORDER BY failed. Got: " + query3);
                passed = false;
            } else {
                System.out.println("  + SELECT with ORDER BY passed: " + query3);
            }
        } catch (Exception e) {
            System.err.println("  X SELECT with ORDER BY threw exception: " + e.getMessage());
            passed = false;
        }

        // Test 4: SELECT with LIMIT
        System.out.println("\nTest 4: SELECT with LIMIT");
        try {
            String query4 = new Database.QueryBuilder()
                .select("id")
                .from("orders")
                .limit(10)
                .build();

            if (!containsKeywords(query4, "SELECT", "FROM", "orders", "LIMIT", "10")) {
                System.err.println("  X SELECT with LIMIT failed. Got: " + query4);
                passed = false;
            } else {
                System.out.println("  + SELECT with LIMIT passed: " + query4);
            }
        } catch (Exception e) {
            System.err.println("  X SELECT with LIMIT threw exception: " + e.getMessage());
            passed = false;
        }

        // Test 5: Full query with all clauses
        System.out.println("\nTest 5: Full query with all clauses");
        try {
            String query5 = new Database.QueryBuilder()
                .select("id", "name", "email", "created_at")
                .from("users")
                .where("status = 'active'")
                .orderBy("created_at", "ASC")
                .limit(50)
                .build();

            if (!containsKeywords(query5, "SELECT", "id", "name", "email", "created_at",
                                  "FROM", "users", "WHERE", "status", "ORDER BY", "ASC", "LIMIT", "50")) {
                System.err.println("  X Full query failed. Got: " + query5);
                passed = false;
            } else {
                System.out.println("  + Full query passed: " + query5);
            }
        } catch (Exception e) {
            System.err.println("  X Full query threw exception: " + e.getMessage());
            passed = false;
        }

        // Test 6: Method chaining in different order
        System.out.println("\nTest 6: Method chaining in different order");
        try {
            String query6 = new Database.QueryBuilder()
                .from("inventory")
                .where("quantity > 0")
                .select("product_id", "quantity")
                .limit(100)
                .orderBy("quantity", "DESC")
                .build();

            if (!containsKeywords(query6, "SELECT", "product_id", "quantity", "FROM", "inventory",
                                  "WHERE", "ORDER BY", "DESC", "LIMIT", "100")) {
                System.err.println("  X Different order chaining failed. Got: " + query6);
                passed = false;
            } else {
                System.out.println("  + Different order chaining passed: " + query6);
            }
        } catch (Exception e) {
            System.err.println("  X Different order chaining threw exception: " + e.getMessage());
            passed = false;
        }

        // Print final result
        if (passed) {
            System.out.println("\n[SUCCESS] All SQL Query Builder tests passed!");
        } else {
            System.out.println("\n[FAILED] Some SQL Query Builder tests failed.");
            System.exit(1);
        }
    }

    /**
     * Helper method to check if a query contains all required keywords (case-insensitive)
     */
    private static boolean containsKeywords(String query, String... keywords) {
        if (query == null || query.isEmpty()) {
            return false;
        }
        String upperQuery = query.toUpperCase();
        for (String keyword : keywords) {
            if (!upperQuery.contains(keyword.toUpperCase())) {
                return false;
            }
        }
        return true;
    }
}

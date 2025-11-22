// Test suite for Chapter 7: Vault of Data - SQL Query Builder

#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include "database.cpp"

// Helper function to convert string to uppercase
std::string toUpper(const std::string& str) {
    std::string result = str;
    std::transform(result.begin(), result.end(), result.begin(), ::toupper);
    return result;
}

// Helper function to check if a query contains all required keywords (case-insensitive)
bool containsKeywords(const std::string& query, std::vector<std::string> keywords) {
    if (query.empty()) {
        return false;
    }
    std::string upperQuery = toUpper(query);
    for (const auto& keyword : keywords) {
        if (upperQuery.find(toUpper(keyword)) == std::string::npos) {
            return false;
        }
    }
    return true;
}

int main() {
    bool passed = true;

    std::cout << "Testing SQL Query Builder...\n" << std::endl;

    // Test 1: Basic SELECT query
    std::cout << "Test 1: Basic SELECT query" << std::endl;
    try {
        std::string query1 = QueryBuilder()
            .select({"id", "name", "email"})
            .from("users")
            .build();

        if (!containsKeywords(query1, {"SELECT", "id", "name", "email", "FROM", "users"})) {
            std::cerr << "  X Basic SELECT failed. Got: " << query1 << std::endl;
            passed = false;
        } else {
            std::cout << "  + Basic SELECT passed: " << query1 << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "  X Basic SELECT threw exception: " << e.what() << std::endl;
        passed = false;
    }

    // Test 2: SELECT with WHERE clause
    std::cout << "\nTest 2: SELECT with WHERE clause" << std::endl;
    try {
        std::string query2 = QueryBuilder()
            .select({"*"})
            .from("users")
            .where("active = true")
            .build();

        if (!containsKeywords(query2, {"SELECT", "*", "FROM", "users", "WHERE", "active"})) {
            std::cerr << "  X SELECT with WHERE failed. Got: " << query2 << std::endl;
            passed = false;
        } else {
            std::cout << "  + SELECT with WHERE passed: " << query2 << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "  X SELECT with WHERE threw exception: " << e.what() << std::endl;
        passed = false;
    }

    // Test 3: SELECT with ORDER BY
    std::cout << "\nTest 3: SELECT with ORDER BY" << std::endl;
    try {
        std::string query3 = QueryBuilder()
            .select({"id", "name"})
            .from("products")
            .orderBy("price", "DESC")
            .build();

        if (!containsKeywords(query3, {"SELECT", "FROM", "products", "ORDER BY", "price", "DESC"})) {
            std::cerr << "  X SELECT with ORDER BY failed. Got: " << query3 << std::endl;
            passed = false;
        } else {
            std::cout << "  + SELECT with ORDER BY passed: " << query3 << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "  X SELECT with ORDER BY threw exception: " << e.what() << std::endl;
        passed = false;
    }

    // Test 4: SELECT with LIMIT
    std::cout << "\nTest 4: SELECT with LIMIT" << std::endl;
    try {
        std::string query4 = QueryBuilder()
            .select({"id"})
            .from("orders")
            .limit(10)
            .build();

        if (!containsKeywords(query4, {"SELECT", "FROM", "orders", "LIMIT", "10"})) {
            std::cerr << "  X SELECT with LIMIT failed. Got: " << query4 << std::endl;
            passed = false;
        } else {
            std::cout << "  + SELECT with LIMIT passed: " << query4 << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "  X SELECT with LIMIT threw exception: " << e.what() << std::endl;
        passed = false;
    }

    // Test 5: Full query with all clauses
    std::cout << "\nTest 5: Full query with all clauses" << std::endl;
    try {
        std::string query5 = QueryBuilder()
            .select({"id", "name", "email", "created_at"})
            .from("users")
            .where("status = 'active'")
            .orderBy("created_at", "ASC")
            .limit(50)
            .build();

        if (!containsKeywords(query5, {"SELECT", "id", "name", "email", "created_at",
                                       "FROM", "users", "WHERE", "status", "ORDER BY", "ASC", "LIMIT", "50"})) {
            std::cerr << "  X Full query failed. Got: " << query5 << std::endl;
            passed = false;
        } else {
            std::cout << "  + Full query passed: " << query5 << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "  X Full query threw exception: " << e.what() << std::endl;
        passed = false;
    }

    // Test 6: Method chaining in different order
    std::cout << "\nTest 6: Method chaining in different order" << std::endl;
    try {
        std::string query6 = QueryBuilder()
            .from("inventory")
            .where("quantity > 0")
            .select({"product_id", "quantity"})
            .limit(100)
            .orderBy("quantity", "DESC")
            .build();

        if (!containsKeywords(query6, {"SELECT", "product_id", "quantity", "FROM", "inventory",
                                       "WHERE", "ORDER BY", "DESC", "LIMIT", "100"})) {
            std::cerr << "  X Different order chaining failed. Got: " << query6 << std::endl;
            passed = false;
        } else {
            std::cout << "  + Different order chaining passed: " << query6 << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "  X Different order chaining threw exception: " << e.what() << std::endl;
        passed = false;
    }

    // Print final result
    if (passed) {
        std::cout << "\n[SUCCESS] All SQL Query Builder tests passed!" << std::endl;
        return 0;
    } else {
        std::cout << "\n[FAILED] Some SQL Query Builder tests failed." << std::endl;
        return 1;
    }
}

// Chapter 7: Vault of Data - Database Design (SQL Query Builder)
// Your task: Implement a fluent SQL query builder using the builder pattern

#include <string>
#include <vector>
#include <stdexcept>
#include <sstream>

/**
 * QueryBuilder - A fluent SQL query builder
 *
 * This class allows building SQL SELECT queries using method chaining (builder pattern).
 * The builder should construct valid SQL query strings.
 *
 * Example usage:
 *   std::string query = QueryBuilder()
 *       .select({"id", "name", "email"})
 *       .from("users")
 *       .where("active = true")
 *       .orderBy("created_at", "DESC")
 *       .limit(10)
 *       .build();
 *   // Result: "SELECT id, name, email FROM users WHERE active = true ORDER BY created_at DESC LIMIT 10"
 */
class QueryBuilder {
private:
    // TODO: Add private member variables to store query parts
    // - columns (for SELECT) - use std::vector<std::string>
    // - table (for FROM) - use std::string
    // - condition (for WHERE) - use std::string
    // - orderColumn and orderDirection (for ORDER BY) - use std::string
    // - limitCount (for LIMIT) - use int (-1 for unset)
    // - hasLimit (to track if limit was set) - use bool

public:
    /**
     * Specify columns to select
     * TODO: Store the columns and return *this for method chaining
     *
     * @param cols Vector of column names
     * @return Reference to this QueryBuilder for method chaining
     */
    QueryBuilder& select(std::vector<std::string> cols) {
        // TODO: Store columns in a member variable
        // Hint: this->columns = cols;
        throw std::runtime_error("Not implemented yet");
    }

    /**
     * Specify the table to query from
     * TODO: Store the table name and return *this for method chaining
     *
     * @param tableName Table name
     * @return Reference to this QueryBuilder for method chaining
     */
    QueryBuilder& from(std::string tableName) {
        // TODO: Store table name in a member variable
        throw std::runtime_error("Not implemented yet");
    }

    /**
     * Add a WHERE condition
     * TODO: Store the condition and return *this for method chaining
     *
     * @param cond SQL condition (e.g., "active = true")
     * @return Reference to this QueryBuilder for method chaining
     */
    QueryBuilder& where(std::string cond) {
        // TODO: Store condition in a member variable
        throw std::runtime_error("Not implemented yet");
    }

    /**
     * Add ORDER BY clause
     * TODO: Store the column and direction, return *this for method chaining
     *
     * @param column Column to order by
     * @param direction Sort direction ("ASC" or "DESC")
     * @return Reference to this QueryBuilder for method chaining
     */
    QueryBuilder& orderBy(std::string column, std::string direction) {
        // TODO: Store order column and direction in member variables
        throw std::runtime_error("Not implemented yet");
    }

    /**
     * Add LIMIT clause
     * TODO: Store the limit count and return *this for method chaining
     *
     * @param n Maximum number of rows to return
     * @return Reference to this QueryBuilder for method chaining
     */
    QueryBuilder& limit(int n) {
        // TODO: Store limit count in a member variable
        throw std::runtime_error("Not implemented yet");
    }

    /**
     * Build the final SQL query string
     * TODO: Combine all stored parts into a valid SQL query
     *
     * The query should be built in this order:
     * 1. SELECT columns (or * if none specified)
     * 2. FROM table
     * 3. WHERE condition (if specified)
     * 4. ORDER BY column direction (if specified)
     * 5. LIMIT n (if specified)
     *
     * @return The complete SQL query string
     */
    std::string build() {
        // TODO: Build and return the SQL query string
        // Hint: Use std::ostringstream for building strings
        // Hint: Only include clauses that were specified
        throw std::runtime_error("Not implemented yet");
    }
};

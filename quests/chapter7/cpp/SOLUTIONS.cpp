// Reference solutions for Chapter 7: Vault of Data - SQL Query Builder

#include <string>
#include <vector>
#include <sstream>
#include <iostream>

/**
 * QueryBuilder - A fluent SQL query builder (SOLUTION)
 *
 * This implementation demonstrates the builder pattern for constructing SQL queries.
 */
class QueryBuilder {
private:
    std::vector<std::string> columns;
    std::string table;
    std::string condition;
    std::string orderColumn;
    std::string orderDirection;
    int limitCount;
    bool hasLimit;

public:
    // Constructor - initialize defaults
    QueryBuilder() : limitCount(-1), hasLimit(false) {}

    /**
     * Specify columns to select - SOLUTION
     */
    QueryBuilder& select(std::vector<std::string> cols) {
        columns = cols;
        return *this;
    }

    /**
     * Specify the table to query from - SOLUTION
     */
    QueryBuilder& from(std::string tableName) {
        table = tableName;
        return *this;
    }

    /**
     * Add a WHERE condition - SOLUTION
     */
    QueryBuilder& where(std::string cond) {
        condition = cond;
        return *this;
    }

    /**
     * Add ORDER BY clause - SOLUTION
     */
    QueryBuilder& orderBy(std::string column, std::string direction) {
        orderColumn = column;
        orderDirection = direction;
        return *this;
    }

    /**
     * Add LIMIT clause - SOLUTION
     */
    QueryBuilder& limit(int n) {
        limitCount = n;
        hasLimit = true;
        return *this;
    }

    /**
     * Build the final SQL query string - SOLUTION
     */
    std::string build() {
        std::ostringstream query;

        // SELECT clause
        query << "SELECT ";
        if (columns.empty()) {
            query << "*";
        } else {
            for (size_t i = 0; i < columns.size(); ++i) {
                if (i > 0) {
                    query << ", ";
                }
                query << columns[i];
            }
        }

        // FROM clause
        if (!table.empty()) {
            query << " FROM " << table;
        }

        // WHERE clause (optional)
        if (!condition.empty()) {
            query << " WHERE " << condition;
        }

        // ORDER BY clause (optional)
        if (!orderColumn.empty() && !orderDirection.empty()) {
            query << " ORDER BY " << orderColumn << " " << orderDirection;
        }

        // LIMIT clause (optional)
        if (hasLimit) {
            query << " LIMIT " << limitCount;
        }

        return query.str();
    }
};

// Main function to demonstrate usage
int main() {
    std::cout << "SQL Query Builder - Solution Demo\n" << std::endl;

    // Example 1: Simple query
    std::string query1 = QueryBuilder()
        .select({"id", "name", "email"})
        .from("users")
        .build();
    std::cout << "Query 1: " << query1 << std::endl;

    // Example 2: Query with WHERE
    std::string query2 = QueryBuilder()
        .select({"*"})
        .from("users")
        .where("active = true")
        .build();
    std::cout << "Query 2: " << query2 << std::endl;

    // Example 3: Full query
    std::string query3 = QueryBuilder()
        .select({"id", "name", "email", "created_at"})
        .from("users")
        .where("status = 'active'")
        .orderBy("created_at", "DESC")
        .limit(10)
        .build();
    std::cout << "Query 3: " << query3 << std::endl;

    // Example 4: Different method order (builder pattern flexibility)
    std::string query4 = QueryBuilder()
        .from("products")
        .limit(5)
        .select({"name", "price"})
        .where("in_stock = true")
        .orderBy("price", "ASC")
        .build();
    std::cout << "Query 4: " << query4 << std::endl;

    return 0;
}

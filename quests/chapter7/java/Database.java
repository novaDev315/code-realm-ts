// Chapter 7: Vault of Data - Database Design (SQL Query Builder)
// Your task: Implement a fluent SQL query builder using the builder pattern

/**
 * QueryBuilder - A fluent SQL query builder
 *
 * This class allows building SQL SELECT queries using method chaining (builder pattern).
 * The builder should construct valid SQL query strings.
 *
 * Example usage:
 *   String query = new QueryBuilder()
 *       .select("id", "name", "email")
 *       .from("users")
 *       .where("active = true")
 *       .orderBy("created_at", "DESC")
 *       .limit(10)
 *       .build();
 *   // Result: "SELECT id, name, email FROM users WHERE active = true ORDER BY created_at DESC LIMIT 10"
 */
public class Database {

    public static class QueryBuilder {
        // TODO: Add private fields to store query parts
        // - columns (for SELECT)
        // - table (for FROM)
        // - condition (for WHERE)
        // - orderColumn and orderDirection (for ORDER BY)
        // - limitCount (for LIMIT)

        /**
         * Specify columns to select
         * TODO: Store the columns and return this for method chaining
         *
         * @param columns Variable number of column names
         * @return this QueryBuilder for method chaining
         */
        public QueryBuilder select(String... columns) {
            // TODO: Store columns in a field
            // Hint: You can use String.join(", ", columns) when building
            throw new UnsupportedOperationException("Not implemented yet");
        }

        /**
         * Specify the table to query from
         * TODO: Store the table name and return this for method chaining
         *
         * @param table Table name
         * @return this QueryBuilder for method chaining
         */
        public QueryBuilder from(String table) {
            // TODO: Store table name in a field
            throw new UnsupportedOperationException("Not implemented yet");
        }

        /**
         * Add a WHERE condition
         * TODO: Store the condition and return this for method chaining
         *
         * @param condition SQL condition (e.g., "active = true")
         * @return this QueryBuilder for method chaining
         */
        public QueryBuilder where(String condition) {
            // TODO: Store condition in a field
            throw new UnsupportedOperationException("Not implemented yet");
        }

        /**
         * Add ORDER BY clause
         * TODO: Store the column and direction, return this for method chaining
         *
         * @param column Column to order by
         * @param direction Sort direction ("ASC" or "DESC")
         * @return this QueryBuilder for method chaining
         */
        public QueryBuilder orderBy(String column, String direction) {
            // TODO: Store order column and direction in fields
            throw new UnsupportedOperationException("Not implemented yet");
        }

        /**
         * Add LIMIT clause
         * TODO: Store the limit count and return this for method chaining
         *
         * @param n Maximum number of rows to return
         * @return this QueryBuilder for method chaining
         */
        public QueryBuilder limit(int n) {
            // TODO: Store limit count in a field
            throw new UnsupportedOperationException("Not implemented yet");
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
        public String build() {
            // TODO: Build and return the SQL query string
            // Hint: Use StringBuilder for efficiency
            // Hint: Only include clauses that were specified
            throw new UnsupportedOperationException("Not implemented yet");
        }
    }
}

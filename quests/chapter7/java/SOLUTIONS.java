// Reference solutions for Chapter 7: Vault of Data - SQL Query Builder

import java.util.ArrayList;
import java.util.List;

/**
 * QueryBuilder - A fluent SQL query builder (SOLUTION)
 *
 * This implementation demonstrates the builder pattern for constructing SQL queries.
 */
public class SOLUTIONS {

    public static class QueryBuilder {
        private List<String> columns = new ArrayList<>();
        private String table = null;
        private String condition = null;
        private String orderColumn = null;
        private String orderDirection = null;
        private Integer limitCount = null;

        /**
         * Specify columns to select - SOLUTION
         */
        public QueryBuilder select(String... cols) {
            columns.clear();
            for (String col : cols) {
                columns.add(col);
            }
            return this;
        }

        /**
         * Specify the table to query from - SOLUTION
         */
        public QueryBuilder from(String tableName) {
            this.table = tableName;
            return this;
        }

        /**
         * Add a WHERE condition - SOLUTION
         */
        public QueryBuilder where(String cond) {
            this.condition = cond;
            return this;
        }

        /**
         * Add ORDER BY clause - SOLUTION
         */
        public QueryBuilder orderBy(String column, String direction) {
            this.orderColumn = column;
            this.orderDirection = direction;
            return this;
        }

        /**
         * Add LIMIT clause - SOLUTION
         */
        public QueryBuilder limit(int n) {
            this.limitCount = n;
            return this;
        }

        /**
         * Build the final SQL query string - SOLUTION
         */
        public String build() {
            StringBuilder query = new StringBuilder();

            // SELECT clause
            query.append("SELECT ");
            if (columns.isEmpty()) {
                query.append("*");
            } else {
                query.append(String.join(", ", columns));
            }

            // FROM clause
            if (table != null) {
                query.append(" FROM ").append(table);
            }

            // WHERE clause (optional)
            if (condition != null) {
                query.append(" WHERE ").append(condition);
            }

            // ORDER BY clause (optional)
            if (orderColumn != null && orderDirection != null) {
                query.append(" ORDER BY ").append(orderColumn).append(" ").append(orderDirection);
            }

            // LIMIT clause (optional)
            if (limitCount != null) {
                query.append(" LIMIT ").append(limitCount);
            }

            return query.toString();
        }
    }

    // Main method to demonstrate usage
    public static void main(String[] args) {
        System.out.println("SQL Query Builder - Solution Demo\n");

        // Example 1: Simple query
        String query1 = new QueryBuilder()
            .select("id", "name", "email")
            .from("users")
            .build();
        System.out.println("Query 1: " + query1);

        // Example 2: Query with WHERE
        String query2 = new QueryBuilder()
            .select("*")
            .from("users")
            .where("active = true")
            .build();
        System.out.println("Query 2: " + query2);

        // Example 3: Full query
        String query3 = new QueryBuilder()
            .select("id", "name", "email", "created_at")
            .from("users")
            .where("status = 'active'")
            .orderBy("created_at", "DESC")
            .limit(10)
            .build();
        System.out.println("Query 3: " + query3);

        // Example 4: Different method order (builder pattern flexibility)
        String query4 = new QueryBuilder()
            .from("products")
            .limit(5)
            .select("name", "price")
            .where("in_stock = true")
            .orderBy("price", "ASC")
            .build();
        System.out.println("Query 4: " + query4);
    }
}

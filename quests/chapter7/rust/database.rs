// Chapter 7: Vault of Data - Database Design (SQL Query Builder)
// This chapter implements a fluent SQL query builder pattern.

/// QueryBuilder builds SQL queries using a fluent interface (builder pattern)
#[allow(dead_code)]
pub struct QueryBuilder {
    columns: Vec<String>,
    table: String,
    condition: String,
    order_col: String,
    order_dir: String,
    limit_n: Option<i32>,
}

impl QueryBuilder {
    /// Creates a new QueryBuilder instance
    /// TODO: Initialize and return a new QueryBuilder with default values
    pub fn new() -> Self {
        // TODO: Implement this function
        QueryBuilder {
            columns: Vec::new(),
            table: String::new(),
            condition: String::new(),
            order_col: String::new(),
            order_dir: String::new(),
            limit_n: None,
        }
    }

    /// Specifies the columns to select
    /// If no columns are specified, should default to "*"
    /// TODO: Store the columns and return self for chaining
    #[allow(unused_variables, unused_mut)]
    pub fn select(mut self, columns: Vec<&str>) -> Self {
        // TODO: Implement this method
        self
    }

    /// Specifies the table to query from
    /// TODO: Store the table name and return self for chaining
    #[allow(unused_variables, unused_mut)]
    pub fn from(mut self, table: &str) -> Self {
        // TODO: Implement this method
        self
    }

    /// Specifies the WHERE condition
    /// TODO: Store the condition and return self for chaining
    #[allow(unused_variables, unused_mut)]
    pub fn where_clause(mut self, condition: &str) -> Self {
        // TODO: Implement this method
        self
    }

    /// Specifies the ORDER BY column and direction (ASC or DESC)
    /// TODO: Store the order info and return self for chaining
    #[allow(unused_variables, unused_mut)]
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        // TODO: Implement this method
        self
    }

    /// Specifies the maximum number of rows to return
    /// TODO: Store the limit value and return self for chaining
    #[allow(unused_variables, unused_mut)]
    pub fn limit(mut self, n: i32) -> Self {
        // TODO: Implement this method
        self
    }

    /// Constructs and returns the SQL query string
    /// TODO: Combine all parts into a valid SQL SELECT statement
    /// Format: SELECT columns FROM table [WHERE condition] [ORDER BY col dir] [LIMIT n]
    pub fn build(&self) -> String {
        // TODO: Implement this method
        String::new()
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Chapter 7: Vault of Data - Database Design (SQL Query Builder)
// Reference solutions for the SQL Query Builder implementation.

/// QueryBuilderSolution builds SQL queries using a fluent interface (builder pattern)
pub struct QueryBuilderSolution {
    columns: Vec<String>,
    table: String,
    condition: String,
    order_col: String,
    order_dir: String,
    limit_n: Option<i32>,
}

impl QueryBuilderSolution {
    /// Creates a new QueryBuilderSolution instance
    pub fn new() -> Self {
        QueryBuilderSolution {
            columns: Vec::new(),
            table: String::new(),
            condition: String::new(),
            order_col: String::new(),
            order_dir: String::new(),
            limit_n: None,
        }
    }

    /// Specifies the columns to select
    /// If no columns are specified, defaults to "*"
    pub fn select(mut self, columns: Vec<&str>) -> Self {
        if columns.is_empty() {
            self.columns = vec!["*".to_string()];
        } else {
            self.columns = columns.iter().map(|s| s.to_string()).collect();
        }
        self
    }

    /// Specifies the table to query from
    pub fn from(mut self, table: &str) -> Self {
        self.table = table.to_string();
        self
    }

    /// Specifies the WHERE condition
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.condition = condition.to_string();
        self
    }

    /// Specifies the ORDER BY column and direction (ASC or DESC)
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order_col = column.to_string();
        self.order_dir = direction.to_string();
        self
    }

    /// Specifies the maximum number of rows to return
    pub fn limit(mut self, n: i32) -> Self {
        self.limit_n = Some(n);
        self
    }

    /// Constructs and returns the SQL query string
    pub fn build(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        // SELECT clause
        let cols = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };
        parts.push(format!("SELECT {}", cols));

        // FROM clause
        if !self.table.is_empty() {
            parts.push(format!("FROM {}", self.table));
        }

        // WHERE clause
        if !self.condition.is_empty() {
            parts.push(format!("WHERE {}", self.condition));
        }

        // ORDER BY clause
        if !self.order_col.is_empty() {
            let mut order_clause = format!("ORDER BY {}", self.order_col);
            if !self.order_dir.is_empty() {
                order_clause.push(' ');
                order_clause.push_str(&self.order_dir);
            }
            parts.push(order_clause);
        }

        // LIMIT clause
        if let Some(limit) = self.limit_n {
            parts.push(format!("LIMIT {}", limit));
        }

        parts.join(" ")
    }
}

impl Default for QueryBuilderSolution {
    fn default() -> Self {
        Self::new()
    }
}

package main

// Chapter 7: Vault of Data - Database Design (SQL Query Builder)
// This chapter implements a fluent SQL query builder pattern.

// QueryBuilder builds SQL queries using a fluent interface
type QueryBuilder struct {
	columns   []string
	table     string
	condition string
	orderCol  string
	orderDir  string
	limitN    int
	hasLimit  bool
}

// NewQueryBuilder creates a new QueryBuilder instance
// TODO: Initialize and return a new QueryBuilder
func NewQueryBuilder() *QueryBuilder {
	// TODO: Implement this function
	return nil
}

// Select specifies the columns to select
// If no columns are specified, should default to "*"
// TODO: Store the columns and return the builder for chaining
func (qb *QueryBuilder) Select(columns ...string) *QueryBuilder {
	// TODO: Implement this method
	return qb
}

// From specifies the table to query from
// TODO: Store the table name and return the builder for chaining
func (qb *QueryBuilder) From(table string) *QueryBuilder {
	// TODO: Implement this method
	return qb
}

// Where specifies the WHERE condition
// TODO: Store the condition and return the builder for chaining
func (qb *QueryBuilder) Where(condition string) *QueryBuilder {
	// TODO: Implement this method
	return qb
}

// OrderBy specifies the ORDER BY column and direction (ASC or DESC)
// TODO: Store the order info and return the builder for chaining
func (qb *QueryBuilder) OrderBy(column string, direction string) *QueryBuilder {
	// TODO: Implement this method
	return qb
}

// Limit specifies the maximum number of rows to return
// TODO: Store the limit value and return the builder for chaining
func (qb *QueryBuilder) Limit(n int) *QueryBuilder {
	// TODO: Implement this method
	return qb
}

// Build constructs and returns the SQL query string
// TODO: Combine all parts into a valid SQL SELECT statement
// Format: SELECT columns FROM table [WHERE condition] [ORDER BY col dir] [LIMIT n]
func (qb *QueryBuilder) Build() string {
	// TODO: Implement this method
	return ""
}

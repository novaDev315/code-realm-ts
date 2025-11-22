package main

import (
	"strconv"
	"strings"
)

// Chapter 7: Vault of Data - Database Design (SQL Query Builder)
// Reference solutions for the SQL Query Builder implementation.

// QueryBuilderSolution builds SQL queries using a fluent interface
type QueryBuilderSolution struct {
	columns   []string
	table     string
	condition string
	orderCol  string
	orderDir  string
	limitN    int
	hasLimit  bool
}

// NewQueryBuilderSolution creates a new QueryBuilderSolution instance
func NewQueryBuilderSolution() *QueryBuilderSolution {
	return &QueryBuilderSolution{
		columns:  []string{},
		hasLimit: false,
	}
}

// Select specifies the columns to select
// If no columns are specified, defaults to "*"
func (qb *QueryBuilderSolution) Select(columns ...string) *QueryBuilderSolution {
	if len(columns) == 0 {
		qb.columns = []string{"*"}
	} else {
		qb.columns = columns
	}
	return qb
}

// From specifies the table to query from
func (qb *QueryBuilderSolution) From(table string) *QueryBuilderSolution {
	qb.table = table
	return qb
}

// Where specifies the WHERE condition
func (qb *QueryBuilderSolution) Where(condition string) *QueryBuilderSolution {
	qb.condition = condition
	return qb
}

// OrderBy specifies the ORDER BY column and direction (ASC or DESC)
func (qb *QueryBuilderSolution) OrderBy(column string, direction string) *QueryBuilderSolution {
	qb.orderCol = column
	qb.orderDir = direction
	return qb
}

// Limit specifies the maximum number of rows to return
func (qb *QueryBuilderSolution) Limit(n int) *QueryBuilderSolution {
	qb.limitN = n
	qb.hasLimit = true
	return qb
}

// Build constructs and returns the SQL query string
func (qb *QueryBuilderSolution) Build() string {
	var parts []string

	// SELECT clause
	cols := "*"
	if len(qb.columns) > 0 {
		cols = strings.Join(qb.columns, ", ")
	}
	parts = append(parts, "SELECT "+cols)

	// FROM clause
	if qb.table != "" {
		parts = append(parts, "FROM "+qb.table)
	}

	// WHERE clause
	if qb.condition != "" {
		parts = append(parts, "WHERE "+qb.condition)
	}

	// ORDER BY clause
	if qb.orderCol != "" {
		orderClause := "ORDER BY " + qb.orderCol
		if qb.orderDir != "" {
			orderClause += " " + qb.orderDir
		}
		parts = append(parts, orderClause)
	}

	// LIMIT clause
	if qb.hasLimit {
		parts = append(parts, "LIMIT "+strconv.Itoa(qb.limitN))
	}

	return strings.Join(parts, " ")
}

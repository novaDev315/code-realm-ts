package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	passed := true

	fmt.Println("Testing SQL Query Builder...")
	fmt.Println()

	// Test 1: Basic SELECT * FROM users
	fmt.Println("Test 1: Basic SELECT * query")
	qb1 := NewQueryBuilder()
	if qb1 == nil {
		fmt.Println("  [X] NewQueryBuilder returned nil")
		passed = false
	} else {
		query1 := qb1.Select().From("users").Build()
		queryUpper1 := strings.ToUpper(query1)
		if !strings.Contains(queryUpper1, "SELECT") || !strings.Contains(queryUpper1, "FROM") || !strings.Contains(queryUpper1, "USERS") {
			fmt.Printf("  [X] Expected SELECT * FROM users, got: %s\n", query1)
			passed = false
		} else {
			fmt.Printf("  [OK] Query: %s\n", query1)
		}
	}

	// Test 2: SELECT specific columns
	fmt.Println("\nTest 2: SELECT specific columns")
	qb2 := NewQueryBuilder()
	if qb2 == nil {
		fmt.Println("  [X] NewQueryBuilder returned nil")
		passed = false
	} else {
		query2 := qb2.Select("id", "name", "email").From("users").Build()
		queryUpper2 := strings.ToUpper(query2)
		if !strings.Contains(queryUpper2, "ID") || !strings.Contains(queryUpper2, "NAME") || !strings.Contains(queryUpper2, "EMAIL") {
			fmt.Printf("  [X] Expected columns id, name, email in query, got: %s\n", query2)
			passed = false
		} else {
			fmt.Printf("  [OK] Query: %s\n", query2)
		}
	}

	// Test 3: SELECT with WHERE clause
	fmt.Println("\nTest 3: SELECT with WHERE clause")
	qb3 := NewQueryBuilder()
	if qb3 == nil {
		fmt.Println("  [X] NewQueryBuilder returned nil")
		passed = false
	} else {
		query3 := qb3.Select("*").From("users").Where("active = true").Build()
		queryUpper3 := strings.ToUpper(query3)
		if !strings.Contains(queryUpper3, "WHERE") || !strings.Contains(queryUpper3, "ACTIVE") {
			fmt.Printf("  [X] Expected WHERE clause, got: %s\n", query3)
			passed = false
		} else {
			fmt.Printf("  [OK] Query: %s\n", query3)
		}
	}

	// Test 4: SELECT with ORDER BY
	fmt.Println("\nTest 4: SELECT with ORDER BY")
	qb4 := NewQueryBuilder()
	if qb4 == nil {
		fmt.Println("  [X] NewQueryBuilder returned nil")
		passed = false
	} else {
		query4 := qb4.Select("*").From("users").OrderBy("created_at", "DESC").Build()
		queryUpper4 := strings.ToUpper(query4)
		if !strings.Contains(queryUpper4, "ORDER BY") || !strings.Contains(queryUpper4, "DESC") {
			fmt.Printf("  [X] Expected ORDER BY clause, got: %s\n", query4)
			passed = false
		} else {
			fmt.Printf("  [OK] Query: %s\n", query4)
		}
	}

	// Test 5: SELECT with LIMIT
	fmt.Println("\nTest 5: SELECT with LIMIT")
	qb5 := NewQueryBuilder()
	if qb5 == nil {
		fmt.Println("  [X] NewQueryBuilder returned nil")
		passed = false
	} else {
		query5 := qb5.Select("*").From("users").Limit(10).Build()
		queryUpper5 := strings.ToUpper(query5)
		if !strings.Contains(queryUpper5, "LIMIT") || !strings.Contains(query5, "10") {
			fmt.Printf("  [X] Expected LIMIT 10, got: %s\n", query5)
			passed = false
		} else {
			fmt.Printf("  [OK] Query: %s\n", query5)
		}
	}

	// Test 6: Complex query with all clauses
	fmt.Println("\nTest 6: Complex query with all clauses")
	qb6 := NewQueryBuilder()
	if qb6 == nil {
		fmt.Println("  [X] NewQueryBuilder returned nil")
		passed = false
	} else {
		query6 := qb6.Select("id", "name").From("users").Where("active = true").OrderBy("name", "ASC").Limit(5).Build()
		queryUpper6 := strings.ToUpper(query6)
		hasSelect := strings.Contains(queryUpper6, "SELECT")
		hasFrom := strings.Contains(queryUpper6, "FROM")
		hasWhere := strings.Contains(queryUpper6, "WHERE")
		hasOrderBy := strings.Contains(queryUpper6, "ORDER BY")
		hasLimit := strings.Contains(queryUpper6, "LIMIT")

		if !hasSelect || !hasFrom || !hasWhere || !hasOrderBy || !hasLimit {
			fmt.Printf("  [X] Missing required clauses in: %s\n", query6)
			passed = false
		} else {
			fmt.Printf("  [OK] Query: %s\n", query6)
		}
	}

	// Test 7: Method chaining returns correct builder
	fmt.Println("\nTest 7: Method chaining")
	qb7 := NewQueryBuilder()
	if qb7 == nil {
		fmt.Println("  [X] NewQueryBuilder returned nil")
		passed = false
	} else {
		result := qb7.Select("*").From("orders").Where("total > 100").OrderBy("total", "DESC").Limit(20)
		if result != qb7 {
			fmt.Println("  [X] Method chaining did not return the same builder instance")
			passed = false
		} else {
			query7 := result.Build()
			fmt.Printf("  [OK] Chained query: %s\n", query7)
		}
	}

	fmt.Println()
	if passed {
		fmt.Println("[OK] All SQL Query Builder tests passed!")
	} else {
		fmt.Println("[X] Some tests failed.")
		os.Exit(1)
	}
}

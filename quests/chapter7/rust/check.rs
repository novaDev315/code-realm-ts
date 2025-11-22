mod database;

use database::QueryBuilder;

fn main() {
    let mut passed = true;

    println!("Testing SQL Query Builder...");
    println!();

    // Test 1: Basic SELECT * FROM users
    println!("Test 1: Basic SELECT * query");
    let query1 = QueryBuilder::new()
        .select(vec![])
        .from("users")
        .build();
    let query_upper1 = query1.to_uppercase();
    if !query_upper1.contains("SELECT") || !query_upper1.contains("FROM") || !query_upper1.contains("USERS") {
        eprintln!("  [X] Expected SELECT * FROM users, got: {}", query1);
        passed = false;
    } else {
        println!("  [OK] Query: {}", query1);
    }

    // Test 2: SELECT specific columns
    println!("\nTest 2: SELECT specific columns");
    let query2 = QueryBuilder::new()
        .select(vec!["id", "name", "email"])
        .from("users")
        .build();
    let query_upper2 = query2.to_uppercase();
    if !query_upper2.contains("ID") || !query_upper2.contains("NAME") || !query_upper2.contains("EMAIL") {
        eprintln!("  [X] Expected columns id, name, email in query, got: {}", query2);
        passed = false;
    } else {
        println!("  [OK] Query: {}", query2);
    }

    // Test 3: SELECT with WHERE clause
    println!("\nTest 3: SELECT with WHERE clause");
    let query3 = QueryBuilder::new()
        .select(vec!["*"])
        .from("users")
        .where_clause("active = true")
        .build();
    let query_upper3 = query3.to_uppercase();
    if !query_upper3.contains("WHERE") || !query_upper3.contains("ACTIVE") {
        eprintln!("  [X] Expected WHERE clause, got: {}", query3);
        passed = false;
    } else {
        println!("  [OK] Query: {}", query3);
    }

    // Test 4: SELECT with ORDER BY
    println!("\nTest 4: SELECT with ORDER BY");
    let query4 = QueryBuilder::new()
        .select(vec!["*"])
        .from("users")
        .order_by("created_at", "DESC")
        .build();
    let query_upper4 = query4.to_uppercase();
    if !query_upper4.contains("ORDER BY") || !query_upper4.contains("DESC") {
        eprintln!("  [X] Expected ORDER BY clause, got: {}", query4);
        passed = false;
    } else {
        println!("  [OK] Query: {}", query4);
    }

    // Test 5: SELECT with LIMIT
    println!("\nTest 5: SELECT with LIMIT");
    let query5 = QueryBuilder::new()
        .select(vec!["*"])
        .from("users")
        .limit(10)
        .build();
    let query_upper5 = query5.to_uppercase();
    if !query_upper5.contains("LIMIT") || !query5.contains("10") {
        eprintln!("  [X] Expected LIMIT 10, got: {}", query5);
        passed = false;
    } else {
        println!("  [OK] Query: {}", query5);
    }

    // Test 6: Complex query with all clauses
    println!("\nTest 6: Complex query with all clauses");
    let query6 = QueryBuilder::new()
        .select(vec!["id", "name"])
        .from("users")
        .where_clause("active = true")
        .order_by("name", "ASC")
        .limit(5)
        .build();
    let query_upper6 = query6.to_uppercase();
    let has_select = query_upper6.contains("SELECT");
    let has_from = query_upper6.contains("FROM");
    let has_where = query_upper6.contains("WHERE");
    let has_order_by = query_upper6.contains("ORDER BY");
    let has_limit = query_upper6.contains("LIMIT");

    if !has_select || !has_from || !has_where || !has_order_by || !has_limit {
        eprintln!("  [X] Missing required clauses in: {}", query6);
        passed = false;
    } else {
        println!("  [OK] Query: {}", query6);
    }

    // Test 7: Different table query
    println!("\nTest 7: Different table query");
    let query7 = QueryBuilder::new()
        .select(vec!["order_id", "total", "status"])
        .from("orders")
        .where_clause("total > 100")
        .order_by("total", "DESC")
        .limit(20)
        .build();
    let query_upper7 = query7.to_uppercase();
    if !query_upper7.contains("ORDERS") || !query_upper7.contains("TOTAL") {
        eprintln!("  [X] Expected orders table query, got: {}", query7);
        passed = false;
    } else {
        println!("  [OK] Query: {}", query7);
    }

    println!();
    if passed {
        println!("[OK] All SQL Query Builder tests passed!");
    } else {
        eprintln!("[X] Some tests failed.");
        std::process::exit(1);
    }
}

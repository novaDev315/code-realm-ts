#!/usr/bin/env python3
# Chapter 7: Vault of Data - Database Design validation

from database import create_user_table, select_active_users, join_users_orders, create_index


def run_check() -> bool:
    """Validate SQL queries contain required keywords and structure."""
    passed = True

    test_cases = [
        {
            "query": create_user_table(),
            "name": "create_user_table",
            "required_keywords": ["CREATE", "TABLE", "users", "id", "name", "email", "created_at"],
        },
        {
            "query": select_active_users(),
            "name": "select_active_users",
            "required_keywords": ["SELECT", "FROM", "users", "ORDER BY"],
        },
        {
            "query": join_users_orders(),
            "name": "join_users_orders",
            "required_keywords": ["SELECT", "FROM", "users", "JOIN", "orders", "COUNT", "GROUP BY"],
        },
        {
            "query": create_index(),
            "name": "create_index",
            "required_keywords": ["CREATE", "INDEX", "users", "email"],
        },
    ]

    print("Testing SQL Query Structure...\n")

    for tc in test_cases:
        if not tc["query"] or tc["query"] == "":
            print(f"❌ {tc['name']}: Query is empty")
            passed = False
            continue

        test_passed = True
        query_upper = tc["query"].upper()

        for keyword in tc["required_keywords"]:
            if keyword.upper() not in query_upper:
                print(f"❌ {tc['name']}: Missing required keyword \"{keyword}\"")
                test_passed = False
                passed = False

        if test_passed:
            query_preview = tc["query"][:80] + ("..." if len(tc["query"]) > 80 else "")
            print(f"✓ {tc['name']}")
            print(f"  Query: {query_preview}")

    if passed:
        print("\n✅ All SQL queries are properly structured!")
    else:
        print("\n❌ Some SQL queries are missing required components.")
        exit(1)

    return passed


if __name__ == "__main__":
    run_check()

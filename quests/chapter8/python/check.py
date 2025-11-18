"""Test suite for Chapter 8: Realm of APIs"""

import sys
from api import create_rest_endpoint, parse_graphql_query, validate_api_key


def run_check() -> bool:
    """Run all test cases and return True if all pass"""
    passed = True

    # Test createRestEndpoint
    endpoint_cases = [
        {
            "input": {"method": "GET", "path": "/users", "handler": lambda: None},
            "expect": {"method": "GET", "path": "/users"},
            "description": "Create GET endpoint"
        },
        {
            "input": {"method": "post", "path": "/api/create", "handler": lambda: None},
            "expect": {"method": "POST", "path": "/api/create"},
            "description": "Create POST endpoint (lowercase method)"
        },
        {
            "input": {"method": "DELETE", "path": "/items/123", "handler": lambda: None},
            "expect": {"method": "DELETE", "path": "/items/123"},
            "description": "Create DELETE endpoint"
        }
    ]

    for c in endpoint_cases:
        result = create_rest_endpoint(c["input"]["method"], c["input"]["path"], c["input"]["handler"])
        if (result.get("method") != c["expect"]["method"] or
            result.get("path") != c["expect"]["path"] or
            not callable(result.get("handler"))):
            print(
                f"❌ create_rest_endpoint failed: {c['description']}\n"
                f"   Expected: method={c['expect']['method']}, path={c['expect']['path']}, handler=function\n"
                f"   Got: method={result.get('method')}, path={result.get('path')}, handler={type(result.get('handler'))}"
            )
            passed = False

    # Test parseGraphQLQuery
    query_parse_cases = [
        {
            "input": "query { user { id name } }",
            "expect": {"operation": "query", "fields": ["user"]},
            "description": "Parse simple query"
        },
        {
            "input": "mutation { createUser { id email } }",
            "expect": {"operation": "mutation", "fields": ["createUser"]},
            "description": "Parse mutation"
        },
        {
            "input": "{ user posts comments }",
            "expect": {"operation": "query", "fields": ["user", "posts", "comments"]},
            "description": "Parse implicit query with multiple fields"
        },
        {
            "input": "subscription { userUpdated }",
            "expect": {"operation": "subscription", "fields": ["userUpdated"]},
            "description": "Parse subscription"
        }
    ]

    for c in query_parse_cases:
        result = parse_graphql_query(c["input"])
        fields_match = (result["fields"] == c["expect"]["fields"])

        if result["operation"] != c["expect"]["operation"] or not fields_match:
            print(
                f"❌ parse_graphql_query failed: {c['description']}\n"
                f"   Input: \"{c['input']}\"\n"
                f"   Expected: operation=\"{c['expect']['operation']}\", fields={c['expect']['fields']}\n"
                f"   Got: operation=\"{result['operation']}\", fields={result['fields']}"
            )
            passed = False

    # Test validateApiKey
    api_key_cases = [
        {
            "input": "abcdef1234567890abcdef1234567890",
            "expect": True,
            "description": "Valid 32-char alphanumeric key"
        },
        {
            "input": "ABCDEF1234567890ABCDEF1234567890",
            "expect": True,
            "description": "Valid uppercase alphanumeric key"
        },
        {
            "input": "MixedCase12345678901234567890abc",
            "expect": True,
            "description": "Valid mixed case key"
        },
        {
            "input": "toolshort",
            "expect": False,
            "description": "Invalid: too short"
        },
        {
            "input": "abcdef1234567890abcdef1234567890extra",
            "expect": False,
            "description": "Invalid: too long"
        },
        {
            "input": "abcdef1234567890abcdef1234567890-!",
            "expect": False,
            "description": "Invalid: contains special characters"
        },
        {
            "input": "",
            "expect": False,
            "description": "Invalid: empty string"
        }
    ]

    for c in api_key_cases:
        result = validate_api_key(c["input"])
        if result != c["expect"]:
            print(
                f"❌ validate_api_key failed: {c['description']}\n"
                f"   Input: \"{c['input']}\"\n"
                f"   Expected: {c['expect']}\n"
                f"   Got: {result}"
            )
            passed = False

    if passed:
        print("✅ All tests passed!")
    else:
        print("❌ Some tests failed.")
        sys.exit(1)

    return passed


if __name__ == "__main__":
    run_check()

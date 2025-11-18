"""Chapter 8: Realm of APIs - Reference Solutions for Python"""

import re
from typing import TypeVar, Generic, Optional, Dict, List, Callable, Any

T = TypeVar('T')


class ApiResponse(Generic[T]):
    """API response wrapper"""
    def __init__(self, status: int, data: Optional[T] = None, error: Optional[str] = None):
        self.data = data
        self.error = error
        self.status = status


def create_rest_endpoint(method: str, path: str, handler: Callable) -> Dict[str, Any]:
    """
    Create a REST endpoint configuration

    Args:
        method: HTTP method (GET, POST, DELETE, etc.)
        path: URL path for the endpoint
        handler: Request handler function

    Returns:
        Endpoint configuration dictionary
    """
    return {
        "method": method.upper(),
        "path": path,
        "handler": handler
    }


def parse_graphql_query(query: str) -> Dict[str, Any]:
    """
    Parse a GraphQL query string

    Args:
        query: GraphQL query string

    Returns:
        Dictionary with 'operation' and 'fields' keys
    """
    # Extract operation type (query, mutation, subscription)
    operation_match = re.match(r'^\s*(query|mutation|subscription)', query, re.IGNORECASE)
    operation = operation_match.group(1).lower() if operation_match else "query"

    # Extract top-level field names from the query
    fields = []
    brace_depth = 0
    current_field = ""

    for char in query:
        if char == "{":
            brace_depth += 1
            if brace_depth == 2:
                # We're entering nested content, save the field name
                field_name = current_field.strip()
                if field_name and re.match(r'^[a-zA-Z_]', field_name):
                    fields.append(field_name)
                current_field = ""
        elif char == "}":
            brace_depth -= 1
        elif brace_depth == 1 and char in (" ", "\n", "\t"):
            # Whitespace at first level - field separator
            field_name = current_field.strip()
            if field_name and re.match(r'^[a-zA-Z_]', field_name):
                fields.append(field_name)
            current_field = ""
        elif brace_depth == 1:
            current_field += char

    # Add any remaining field
    field_name = current_field.strip()
    if field_name and re.match(r'^[a-zA-Z_]', field_name):
        fields.append(field_name)

    return {"operation": operation, "fields": fields[:10]}


def validate_api_key(api_key: str) -> bool:
    """
    Validate API key format

    Args:
        api_key: API key to validate

    Returns:
        True if valid (32 chars alphanumeric), False otherwise
    """
    # Validate API key format: 32 chars alphanumeric
    if not api_key or not isinstance(api_key, str):
        return False

    # Check length and character composition
    return len(api_key) == 32 and bool(re.match(r'^[a-zA-Z0-9]+$', api_key))

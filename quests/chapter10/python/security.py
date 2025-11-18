# Reference solutions for Chapter 10: Citadel of Firewalls
# These are working implementations for testing purposes

import hashlib
import json
import base64
import re
from typing import Dict, Any, List


def hash_password(password: str, salt: str) -> str:
    """Hash a password with a salt using SHA256.

    Args:
        password: The password to hash
        salt: The salt to combine with password

    Returns:
        Hex string of the hash
    """
    combined = password + salt
    hash_obj = hashlib.sha256(combined.encode())
    return hash_obj.hexdigest()


def validate_jwt(token: str) -> Dict[str, Any]:
    """Validate JWT token structure and decode payload.

    JWT format: header.payload.signature

    Args:
        token: The JWT token to validate

    Returns:
        Dict with 'valid' boolean and 'payload' if valid
    """
    parts = token.split(".")

    if len(parts) != 3:
        return {"valid": False}

    try:
        header_b64, payload_b64, signature = parts

        # Validate that parts are non-empty
        if not header_b64 or not payload_b64 or not signature:
            return {"valid": False}

        # Decode header (should be valid JSON)
        header_json = base64.urlsafe_b64decode(header_b64 + "==")
        header = json.loads(header_json)

        # Decode payload (should be valid JSON)
        payload_json = base64.urlsafe_b64decode(payload_b64 + "==")
        payload = json.loads(payload_json)

        # Basic validation passed
        return {"valid": True, "payload": payload}
    except Exception:
        return {"valid": False}


def rate_limit(requests: List[Dict[str, Any]], limit: int, window_ms: int) -> bool:
    """Check if rate limit is exceeded for requests.

    Args:
        requests: List of dicts with 'ip' and 'timestamp' keys
        limit: Maximum number of requests allowed in window
        window_ms: Time window in milliseconds

    Returns:
        True if rate limit is exceeded, False otherwise
    """
    import time

    # Calculate the earliest timestamp in the window
    now = int(time.time() * 1000)  # Current time in milliseconds
    window_start = now - window_ms

    # Count requests within the window
    request_count = sum(1 for req in requests if req.get("timestamp", 0) >= window_start)

    # Return true if limit is exceeded
    return request_count > limit


def sanitize_input(input_str: str) -> str:
    """Sanitize input to prevent XSS attacks.

    Removes script tags and HTML tags, escapes special characters.

    Args:
        input_str: The input string to sanitize

    Returns:
        Sanitized string safe from XSS attacks
    """
    # Remove script tags and their content
    sanitized = re.sub(r"<script\b[^<]*(?:(?!</script>)<[^<]*)*</script>", "", input_str, flags=re.IGNORECASE)

    # Remove other HTML tags
    sanitized = re.sub(r"<[^>]*>", "", sanitized)

    # Escape dangerous characters
    sanitized = sanitized.replace('"', "&quot;").replace("'", "&#39;")

    # Fix ampersands (must be done last to avoid double-escaping)
    sanitized = re.sub(r"&(?!(?:[a-z]+|#[0-9]+);)", "&amp;", sanitized, flags=re.IGNORECASE)

    return sanitized

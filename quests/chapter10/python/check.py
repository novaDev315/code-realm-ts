#!/usr/bin/env python3
"""Comprehensive security tests for Chapter 10."""

import sys
import time
from security import hash_password, validate_jwt, rate_limit, sanitize_input


def run_check() -> bool:
    """Run all security test cases for Chapter 10."""
    passed = True

    # Test 1: Password Hashing
    print("\n🔐 Testing Password Hashing...")
    hash_cases = [
        {"password": "secret123", "salt": "salt1"},
        {"password": "mypassword", "salt": "salt2"}
    ]

    for case in hash_cases:
        result = hash_password(case["password"], case["salt"])
        # Should return a non-empty hash string
        if not isinstance(result, str) or len(result) == 0:
            print(f"hash_password('{case['password']}', '{case['salt']}') should return a non-empty hash string, got '{result}'")
            passed = False
        else:
            print(f"✓ hash_password('{case['password']}', '{case['salt']}') = '{result[:16]}...'")

    # Test 2: JWT Validation
    print("\n🎫 Testing JWT Validation...")
    valid_jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
    invalid_jwt = "invalid.token.structure"
    malformed_jwt = "eyJhbGciOiJIUzI1NiJ9.invalid"

    jwt_tests = [
        {"input": valid_jwt, "should_be_valid": True},
        {"input": invalid_jwt, "should_be_valid": False},
        {"input": malformed_jwt, "should_be_valid": False}
    ]

    for test in jwt_tests:
        result = validate_jwt(test["input"])
        is_valid = result.get("valid", False)
        expected_valid = test["should_be_valid"]

        if is_valid != expected_valid:
            print(f"validate_jwt('{test['input'][:20]}...') expected valid={expected_valid}, got {is_valid}")
            passed = False
        else:
            print(f"✓ validate_jwt('{test['input'][:20]}...') = {is_valid}")
            if is_valid and "payload" in result:
                print(f"  Payload: {result['payload']}")

    # Test 3: Rate Limiting
    print("\n⏱️  Testing Rate Limiting...")
    now = int(time.time() * 1000)  # Current time in milliseconds
    requests1 = [
        {"ip": "192.168.1.1", "timestamp": now},
        {"ip": "192.168.1.1", "timestamp": now - 100},
        {"ip": "192.168.1.1", "timestamp": now - 200}
    ]

    # 3 requests within 1 second window with limit of 2 should exceed
    result1 = rate_limit(requests1, 2, 1000)
    if result1 != True:
        print(f"rate_limit([3 requests], limit=2, window=1000ms) expected true (exceeded), got {result1}")
        passed = False
    else:
        print(f"✓ rate_limit([3 requests], limit=2, window=1000ms) = true (exceeded)")

    # 2 requests within 1 second window with limit of 3 should not exceed
    requests2 = [
        {"ip": "192.168.1.2", "timestamp": now},
        {"ip": "192.168.1.2", "timestamp": now - 100}
    ]
    result2 = rate_limit(requests2, 3, 1000)
    if result2 != False:
        print(f"rate_limit([2 requests], limit=3, window=1000ms) expected false (not exceeded), got {result2}")
        passed = False
    else:
        print(f"✓ rate_limit([2 requests], limit=3, window=1000ms) = false (not exceeded)")

    # Test 4: Input Sanitization
    print("\n🛡️  Testing Input Sanitization...")
    sanitize_cases = [
        {"input": "<script>alert('XSS')</script>", "contains_dangerous": True},
        {"input": "Hello<img src=x onerror='alert(1)'>World", "contains_dangerous": True},
        {"input": "Safe input with no tags", "contains_dangerous": False},
        {"input": "<div>Content</div>", "contains_dangerous": True},
        {"input": "Test & Demo \"quoted\"", "contains_dangerous": False}
    ]

    for case in sanitize_cases:
        result = sanitize_input(case["input"])
        # Should remove dangerous tags
        has_dangerous_content = "<" in result or ">" in result or "script" in result

        if has_dangerous_content != case["contains_dangerous"]:
            print(f"sanitize_input('{case['input'][:30]}...') should remove dangerous HTML tags, got '{result}'")
            passed = False
        else:
            print(f"✓ sanitize_input('{case['input'][:30]}...') = '{result[:30]}...'")

    # Summary
    print("\n" + "=" * 50)
    if passed:
        print("✅ All security tests passed!")
    else:
        print("❌ Some security tests failed.")
        sys.exit(1)

    return passed


if __name__ == "__main__":
    run_check()

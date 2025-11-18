from urlshortener import encode_base62, decode_base62, generate_short_code, URLShortener


def run_check() -> bool:
    """Run all tests for URL shortener."""
    passed = True

    # Test encodeBase62
    print("Testing encode_base62...")
    encode_tests = [
        {"input": 0, "expect": "0", "name": "Zero"},
        {"input": 9, "expect": "9", "name": "Single digit"},
        {"input": 10, "expect": "a", "name": "Ten (should be 'a')"},
        {"input": 35, "expect": "z", "name": "35 (lowercase z)"},
        {"input": 36, "expect": "A", "name": "36 (uppercase A)"},
        {"input": 61, "expect": "Z", "name": "61 (uppercase Z)"},
        {"input": 62, "expect": "10", "name": "62 (should be '10')"},
        {"input": 3843, "expect": "ZZ", "name": "3843 (Z*62+Z)"},
    ]

    for test in encode_tests:
        result = encode_base62(test["input"])
        if result != test["expect"]:
            print(
                f'encode_base62({test["input"]}) - {test["name"]}: '
                f'expected "{test["expect"]}" but got "{result}"'
            )
            passed = False

    # Test decodeBase62
    print("Testing decode_base62...")
    decode_tests = [
        {"input": "0", "expect": 0, "name": "Zero"},
        {"input": "9", "expect": 9, "name": "Single digit"},
        {"input": "a", "expect": 10, "name": "Letter 'a' (lowercase)"},
        {"input": "z", "expect": 35, "name": "Letter 'z' (lowercase)"},
        {"input": "A", "expect": 36, "name": "Letter 'A' (uppercase)"},
        {"input": "Z", "expect": 61, "name": "Letter 'Z' (uppercase)"},
        {"input": "10", "expect": 62, "name": "Two chars '10'"},
        {"input": "ZZ", "expect": 3843, "name": "Two Z's"},
    ]

    for test in decode_tests:
        result = decode_base62(test["input"])
        if result != test["expect"]:
            print(
                f'decode_base62("{test["input"]}") - {test["name"]}: '
                f'expected {test["expect"]} but got {result}'
            )
            passed = False

    # Test encoding/decoding round-trip
    print("Testing encode/decode round-trip...")
    round_trip_tests = [0, 1, 10, 35, 36, 61, 62, 100, 1000, 10000, 3843]
    for num in round_trip_tests:
        encoded = encode_base62(num)
        decoded = decode_base62(encoded)
        if decoded != num:
            print(
                f"Round-trip failed for {num}: encode -> \"{encoded}\" -> decode -> {decoded}"
            )
            passed = False

    # Test URLShortener
    print("Testing URLShortener class...")
    shortener = URLShortener()

    # Test shortening URLs
    url1 = "https://www.example.com/very/long/path/that/needs/shortening"
    url2 = "https://github.com/anthropics/code-realm-ts"
    url3 = "https://docs.anthropic.com/en/api/getting-started"

    short1 = shortener.shorten(url1)
    short2 = shortener.shorten(url2)
    short3 = shortener.shorten(url3)

    if not short1 or not short2 or not short3:
        print("URLShortener.shorten() returned empty string")
        passed = False

    # Check that different URLs get different short codes
    if short1 and short2 and short1 == short2:
        print("Different URLs received the same short code")
        passed = False

    # Test expanding URLs
    if short1:
        expanded1 = shortener.expand(short1)
        if expanded1 != url1:
            print(
                f'expand("{short1}"): expected "{url1}" but got "{expanded1}"'
            )
            passed = False

    if short2:
        expanded2 = shortener.expand(short2)
        if expanded2 != url2:
            print(
                f'expand("{short2}"): expected "{url2}" but got "{expanded2}"'
            )
            passed = False

    if short3:
        expanded3 = shortener.expand(short3)
        if expanded3 != url3:
            print(
                f'expand("{short3}"): expected "{url3}" but got "{expanded3}"'
            )
            passed = False

    # Test expanding non-existent code
    non_existent = shortener.expand("nonexistent123")
    if non_existent is not None:
        print(
            f'expand("nonexistent123") should return None, got "{non_existent}"'
        )
        passed = False

    # Test that same URL returns same short code (idempotent)
    shortener_idempotent = URLShortener()
    url_idempotent = "https://example.com"
    short1a = shortener_idempotent.shorten(url_idempotent)
    short1b = shortener_idempotent.shorten(url_idempotent)
    if short1a != short1b:
        print(
            f'Shortening same URL twice returned different codes: '
            f'"{short1a}" vs "{short1b}"'
        )
        passed = False

    if passed:
        print("✅ All tests passed!")
    else:
        print("❌ Some tests failed.")
        exit(1)

    return passed


if __name__ == "__main__":
    run_check()

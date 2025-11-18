# Reference solutions for Chapter 11
# URL Shortener System with Base62 Encoding

BASE62_CHARS = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"


def encode_base62(num: int) -> str:
    """Convert number to base62 string (0-9a-zA-Z)."""
    if num == 0:
        return "0"

    result = ""
    while num > 0:
        result = BASE62_CHARS[num % 62] + result
        num //= 62

    return result


def decode_base62(s: str) -> int:
    """Convert base62 string back to number."""
    result = 0
    for char in s:
        value = BASE62_CHARS.find(char)
        if value == -1:
            raise ValueError(f"Invalid base62 character: {char}")
        result = result * 62 + value

    return result


def generate_short_code(url: str, length: int) -> str:
    """Generate a short code for a URL."""
    # Simple hash-based approach: convert URL to a number then encode
    hash_val = 0
    for char in url:
        hash_val = ((hash_val << 5) - hash_val + ord(char)) & 0xFFFFFFFF

    # Convert to positive number
    hash_val = abs(hash_val)

    # Encode to base62
    encoded = encode_base62(hash_val)

    # Pad or trim to desired length
    if len(encoded) >= length:
        return encoded[:length]
    return encoded.rjust(length, "0")


class URLShortener:
    """URL Shortening system with base62 encoding."""

    def __init__(self):
        """Initialize the shortener with empty mappings and counter."""
        self.url_map = {}  # shortCode -> longUrl
        self.reverse_map = {}  # longUrl -> shortCode
        self.counter = 0

    def shorten(self, long_url: str) -> str:
        """Shorten a URL and return the short code."""
        # Check if we already have a mapping for this URL
        if long_url in self.reverse_map:
            return self.reverse_map[long_url]

        # Generate new short code using counter
        self.counter += 1
        short_code = encode_base62(self.counter)

        # Store mappings
        self.url_map[short_code] = long_url
        self.reverse_map[long_url] = short_code

        return short_code

    def expand(self, short_code: str) -> str | None:
        """Expand a short code back to the original URL."""
        return self.url_map.get(short_code)

    def get_all_mappings(self) -> list[dict]:
        """Return all URL mappings for inspection."""
        mappings = []
        for short_code, long_url in self.url_map.items():
            mappings.append({"short": short_code, "long": long_url})
        return mappings

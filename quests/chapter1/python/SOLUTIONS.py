# Reference solutions for Chapter 1
# These are working implementations for testing purposes

def fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number."""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)


def factorial(n: int) -> int:
    """Calculate the factorial of n."""
    if n <= 1:
        return 1
    return n * factorial(n - 1)


def string_permutations(s: str) -> list[str]:
    """Generate all unique permutations of a string."""
    results = []

    if len(s) == 0:
        return ['']

    if len(s) == 1:
        return [s]

    seen = set()

    for i in range(len(s)):
        char = s[i]
        remaining = s[:i] + s[i+1:]
        perms = string_permutations(remaining)

        for perm in perms:
            full_perm = char + perm
            if full_perm not in seen:
                seen.add(full_perm)
                results.append(full_perm)

    return results

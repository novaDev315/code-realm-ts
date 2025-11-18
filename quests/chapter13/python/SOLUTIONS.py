# Reference solutions for Chapter 13: LRU Cache


class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache = {}

    def get(self, key: str) -> any:
        # Check if key exists
        if key not in self.cache:
            return None

        # Get the value
        value = self.cache[key]

        # Move to most recently used (delete and re-add to end)
        del self.cache[key]
        self.cache[key] = value

        return value

    def put(self, key: str, value: any) -> None:
        # If key already exists, remove it first
        if key in self.cache:
            del self.cache[key]
        elif len(self.cache) >= self.capacity:
            # If at capacity, evict least recently used (first item)
            first_key = next(iter(self.cache))
            del self.cache[first_key]

        # Add/update the key-value pair (added to end = most recently used)
        self.cache[key] = value

    def size(self) -> int:
        return len(self.cache)

    def has(self, key: str) -> bool:
        return key in self.cache


def cache_hit_rate(operations: list[dict]) -> float:
    cache = LRUCache(10)  # Reasonable default capacity
    hits = 0
    total_gets = 0

    for op in operations:
        if op["op"] == "put":
            cache.put(op["key"], op["value"])
        elif op["op"] == "get":
            total_gets += 1
            result = cache.get(op["key"])
            if result is not None:
                hits += 1

    if total_gets == 0:
        return 0.0

    return (hits / total_gets) * 100

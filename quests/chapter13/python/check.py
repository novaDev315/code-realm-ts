from cache import LRUCache, cache_hit_rate


def run_check() -> bool:
    passed = True

    # Test LRUCache basic operations
    print("Testing LRUCache basic operations...")
    cache = LRUCache(2)

    # Test put and get
    cache.put("a", 1)
    cache.put("b", 2)

    get_a = cache.get("a")
    if get_a != 1:
        print(f'❌ Expected cache.get("a") = 1, got {get_a}')
        passed = False
    else:
        print("✓ Put and get work correctly")

    # Test capacity and eviction
    print("\nTesting eviction policy...")
    cache.put("c", 3)  # Should evict "b" since "a" was just accessed

    get_b = cache.get("b")
    if get_b is not None:
        print(f'❌ Expected cache.get("b") = None (evicted), got {get_b}')
        passed = False
    else:
        print("✓ LRU eviction policy works correctly")

    # Test cache size
    print("\nTesting cache size...")
    if cache.size() != 2:
        print(f"❌ Expected cache size = 2, got {cache.size()}")
        passed = False
    else:
        print("✓ Cache size correct")

    # Test has method
    print("\nTesting has method...")
    if not cache.has("a"):
        print('❌ Expected cache.has("a") = True')
        passed = False
    elif cache.has("b"):
        print('❌ Expected cache.has("b") = False')
        passed = False
    else:
        print("✓ Has method works correctly")

    # Test updating existing key doesn't evict
    print("\nTesting update of existing key...")
    cache.put("a", 10)
    if cache.get("a") != 10:
        print('❌ Expected cache.get("a") = 10 after update')
        passed = False
    else:
        print("✓ Update of existing key works")

    # Test cache hit rate calculation
    print("\nTesting cache hit rate calculation...")
    operations = [
        {"op": "put", "key": "x", "value": 1},
        {"op": "put", "key": "y", "value": 2},
        {"op": "get", "key": "x"},  # hit
        {"op": "get", "key": "y"},  # hit
        {"op": "get", "key": "z"},  # miss
        {"op": "get", "key": "x"},  # hit
    ]

    hit_rate = cache_hit_rate(operations)
    expected_hit_rate = 75.0  # 3 hits out of 4 gets = 75%

    if abs(hit_rate - expected_hit_rate) > 0.01:
        print(f"❌ Expected hit rate ≈ {expected_hit_rate}%, got {hit_rate}%")
        passed = False
    else:
        print(f"✓ Cache hit rate calculation correct ({hit_rate}%)")

    # Test with different capacity
    print("\nTesting with capacity = 3...")
    cache3 = LRUCache(3)
    cache3.put("1", "a")
    cache3.put("2", "b")
    cache3.put("3", "c")
    cache3.get("1")  # Access "1" to make it recently used
    cache3.put("4", "d")  # Should evict "2"

    if cache3.has("2"):
        print('❌ Expected "2" to be evicted')
        passed = False
    elif not cache3.has("1"):
        print('❌ Expected "1" to still be in cache')
        passed = False
    else:
        print("✓ Larger cache capacity works correctly")

    if passed:
        print("\n✅ All tests passed!")
    else:
        print("\n❌ Some tests failed.")
        exit(1)

    return passed


if __name__ == "__main__":
    run_check()

// Test file for Chapter 13: Crystal Cache - LRU Cache

public class Check {
    public static void main(String[] args) {
        boolean passed = true;

        // Test LRUCache basic operations
        System.out.println("Testing LRUCache basic operations...");
        LRUCache cache = new LRUCache(2);

        // Test put and get
        cache.put(1, 10);
        cache.put(2, 20);

        int getA = cache.get(1);
        if (getA != 10) {
            System.err.println("❌ Expected cache.get(1) = 10, got " + getA);
            passed = false;
        } else {
            System.out.println("✓ Put and get work correctly");
        }

        // Test capacity and eviction
        System.out.println("\nTesting eviction policy...");
        cache.put(3, 30); // Should evict key 2 since key 1 was just accessed

        int getB = cache.get(2);
        if (getB != -1) {
            System.err.println("❌ Expected cache.get(2) = -1 (evicted), got " + getB);
            passed = false;
        } else {
            System.out.println("✓ LRU eviction policy works correctly");
        }

        // Test cache size
        System.out.println("\nTesting cache size...");
        if (cache.size() != 2) {
            System.err.println("❌ Expected cache size = 2, got " + cache.size());
            passed = false;
        } else {
            System.out.println("✓ Cache size correct");
        }

        // Test has method
        System.out.println("\nTesting has method...");
        if (!cache.has(1)) {
            System.err.println("❌ Expected cache.has(1) = true");
            passed = false;
        } else if (cache.has(2)) {
            System.err.println("❌ Expected cache.has(2) = false");
            passed = false;
        } else {
            System.out.println("✓ Has method works correctly");
        }

        // Test updating existing key doesn't evict
        System.out.println("\nTesting update of existing key...");
        cache.put(1, 100);
        if (cache.get(1) != 100) {
            System.err.println("❌ Expected cache.get(1) = 100 after update");
            passed = false;
        } else {
            System.out.println("✓ Update of existing key works");
        }

        // Test with different capacity
        System.out.println("\nTesting with capacity = 3...");
        LRUCache cache3 = new LRUCache(3);
        cache3.put(1, 10);
        cache3.put(2, 20);
        cache3.put(3, 30);
        cache3.get(1); // Access 1 to make it recently used
        cache3.put(4, 40); // Should evict 2

        if (cache3.has(2)) {
            System.err.println("❌ Expected 2 to be evicted");
            passed = false;
        } else if (!cache3.has(1)) {
            System.err.println("❌ Expected 1 to still be in cache");
            passed = false;
        } else {
            System.out.println("✓ Larger cache capacity works correctly");
        }

        // Test edge case: capacity = 1
        System.out.println("\nTesting edge case: capacity = 1...");
        LRUCache cache1 = new LRUCache(1);
        cache1.put(1, 10);
        cache1.put(2, 20); // Should evict 1
        if (cache1.has(1)) {
            System.err.println("❌ Expected 1 to be evicted");
            passed = false;
        } else if (!cache1.has(2)) {
            System.err.println("❌ Expected 2 to be in cache");
            passed = false;
        } else {
            System.out.println("✓ Capacity = 1 works correctly");
        }

        if (passed) {
            System.out.println("\n✅ All tests passed!");
        } else {
            System.out.println("\n❌ Some tests failed.");
            System.exit(1);
        }
    }
}

// Test file for Chapter 13: Crystal Cache - LRU Cache

#include <iostream>
#include <cassert>
#include "lru.cpp"

using namespace std;

int main() {
    bool passed = true;

    // Test LRUCache basic operations
    cout << "Testing LRUCache basic operations..." << endl;
    LRUCache cache(2);

    // Test put and get
    cache.put(1, 10);
    cache.put(2, 20);

    int getA = cache.get(1);
    if (getA != 10) {
        cerr << "❌ Expected cache.get(1) = 10, got " << getA << endl;
        passed = false;
    } else {
        cout << "✓ Put and get work correctly" << endl;
    }

    // Test capacity and eviction
    cout << "\nTesting eviction policy..." << endl;
    cache.put(3, 30); // Should evict key 2 since key 1 was just accessed

    int getB = cache.get(2);
    if (getB != -1) {
        cerr << "❌ Expected cache.get(2) = -1 (evicted), got " << getB << endl;
        passed = false;
    } else {
        cout << "✓ LRU eviction policy works correctly" << endl;
    }

    // Test cache size
    cout << "\nTesting cache size..." << endl;
    if (cache.size() != 2) {
        cerr << "❌ Expected cache size = 2, got " << cache.size() << endl;
        passed = false;
    } else {
        cout << "✓ Cache size correct" << endl;
    }

    // Test has method
    cout << "\nTesting has method..." << endl;
    if (!cache.has(1)) {
        cerr << "❌ Expected cache.has(1) = true" << endl;
        passed = false;
    } else if (cache.has(2)) {
        cerr << "❌ Expected cache.has(2) = false" << endl;
        passed = false;
    } else {
        cout << "✓ Has method works correctly" << endl;
    }

    // Test updating existing key doesn't evict
    cout << "\nTesting update of existing key..." << endl;
    cache.put(1, 100);
    if (cache.get(1) != 100) {
        cerr << "❌ Expected cache.get(1) = 100 after update" << endl;
        passed = false;
    } else {
        cout << "✓ Update of existing key works" << endl;
    }

    // Test with different capacity
    cout << "\nTesting with capacity = 3..." << endl;
    LRUCache cache3(3);
    cache3.put(1, 10);
    cache3.put(2, 20);
    cache3.put(3, 30);
    cache3.get(1); // Access 1 to make it recently used
    cache3.put(4, 40); // Should evict 2

    if (cache3.has(2)) {
        cerr << "❌ Expected 2 to be evicted" << endl;
        passed = false;
    } else if (!cache3.has(1)) {
        cerr << "❌ Expected 1 to still be in cache" << endl;
        passed = false;
    } else {
        cout << "✓ Larger cache capacity works correctly" << endl;
    }

    // Test edge case: capacity = 1
    cout << "\nTesting edge case: capacity = 1..." << endl;
    LRUCache cache1(1);
    cache1.put(1, 10);
    cache1.put(2, 20); // Should evict 1
    if (cache1.has(1)) {
        cerr << "❌ Expected 1 to be evicted" << endl;
        passed = false;
    } else if (!cache1.has(2)) {
        cerr << "❌ Expected 2 to be in cache" << endl;
        passed = false;
    } else {
        cout << "✓ Capacity = 1 works correctly" << endl;
    }

    if (passed) {
        cout << "\n✅ All tests passed!" << endl;
        return 0;
    } else {
        cout << "\n❌ Some tests failed." << endl;
        return 1;
    }
}

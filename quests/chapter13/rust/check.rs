// Use SOLUTIONS module for testing the working implementation
// To test the stub, change this to: mod lru;
mod SOLUTIONS;

use SOLUTIONS::{LRUCache, Operation, cache_hit_rate};

fn run_check() -> bool {
    let mut passed = true;

    // Test LRUCache basic operations
    println!("Testing LRUCache basic operations...");
    let mut cache = LRUCache::new(2);

    // Test put and get
    cache.put(1, 10);
    cache.put(2, 20);

    let get_a = cache.get(1);
    if get_a != 10 {
        eprintln!("❌ Expected cache.get(1) = 10, got {}", get_a);
        passed = false;
    } else {
        println!("✓ Put and get work correctly");
    }

    // Test capacity and eviction
    println!("\nTesting eviction policy...");
    cache.put(3, 30); // Should evict key 2 since key 1 was just accessed

    let get_b = cache.get(2);
    if get_b != -1 {
        eprintln!("❌ Expected cache.get(2) = -1 (evicted), got {}", get_b);
        passed = false;
    } else {
        println!("✓ LRU eviction policy works correctly");
    }

    // Test cache size
    println!("\nTesting cache size...");
    if cache.size() != 2 {
        eprintln!("❌ Expected cache size = 2, got {}", cache.size());
        passed = false;
    } else {
        println!("✓ Cache size correct");
    }

    // Test has method
    println!("\nTesting has method...");
    if !cache.has(1) {
        eprintln!("❌ Expected cache.has(1) = true");
        passed = false;
    } else if cache.has(2) {
        eprintln!("❌ Expected cache.has(2) = false");
        passed = false;
    } else {
        println!("✓ Has method works correctly");
    }

    // Test updating existing key doesn't evict
    println!("\nTesting update of existing key...");
    cache.put(1, 100);
    if cache.get(1) != 100 {
        eprintln!("❌ Expected cache.get(1) = 100 after update");
        passed = false;
    } else {
        println!("✓ Update of existing key works");
    }

    // Test cache hit rate calculation
    println!("\nTesting cache hit rate calculation...");
    let operations = vec![
        Operation::Put { key: 10, value: 1 },
        Operation::Put { key: 20, value: 2 },
        Operation::Get { key: 10 }, // hit
        Operation::Get { key: 20 }, // hit
        Operation::Get { key: 30 }, // miss
        Operation::Get { key: 10 }, // hit
    ];

    let hit_rate = cache_hit_rate(&operations);
    let expected_hit_rate = 75.0; // 3 hits out of 4 gets = 75%

    if (hit_rate - expected_hit_rate).abs() > 0.01 {
        eprintln!("❌ Expected hit rate ≈ {}%, got {}%", expected_hit_rate, hit_rate);
        passed = false;
    } else {
        println!("✓ Cache hit rate calculation correct ({}%)", hit_rate);
    }

    // Test with different capacity
    println!("\nTesting with capacity = 3...");
    let mut cache3 = LRUCache::new(3);
    cache3.put(1, 100);
    cache3.put(2, 200);
    cache3.put(3, 300);
    cache3.get(1); // Access 1 to make it recently used
    cache3.put(4, 400); // Should evict 2

    if cache3.has(2) {
        eprintln!("❌ Expected 2 to be evicted");
        passed = false;
    } else if !cache3.has(1) {
        eprintln!("❌ Expected 1 to still be in cache");
        passed = false;
    } else {
        println!("✓ Larger cache capacity works correctly");
    }

    if passed {
        println!("\n✅ All tests passed!");
    } else {
        eprintln!("\n❌ Some tests failed.");
        std::process::exit(1);
    }

    passed
}

fn main() {
    run_check();
}

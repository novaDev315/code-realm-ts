// Use SOLUTIONS module for testing the working implementation
// To test the stub, change this to: mod queue;
#[path = "SOLUTIONS.rs"]
mod solutions;

use solutions::{MessageQueue, process_batch};

fn run_check() -> bool {
    let mut passed = true;

    // Test enqueue and size
    println!("Testing enqueue and size...");
    let mut queue1 = MessageQueue::new();

    if queue1.size() != 0 {
        eprintln!("size() on empty queue should be 0, got {}", queue1.size());
        passed = false;
    }

    queue1.enqueue("message1".to_string());
    if queue1.size() != 1 {
        eprintln!("size() after enqueue should be 1, got {}", queue1.size());
        passed = false;
    }

    queue1.enqueue("message2".to_string());
    queue1.enqueue("message3".to_string());
    if queue1.size() != 3 {
        eprintln!("size() after 3 enqueues should be 3, got {}", queue1.size());
        passed = false;
    } else {
        println!("Enqueue and size work correctly");
    }

    // Test is_empty
    println!("\nTesting is_empty...");
    let mut queue2 = MessageQueue::new();
    if !queue2.is_empty() {
        eprintln!("is_empty() on empty queue should be true");
        passed = false;
    }

    queue2.enqueue("test".to_string());
    if queue2.is_empty() {
        eprintln!("is_empty() on non-empty queue should be false");
        passed = false;
    } else {
        println!("is_empty works correctly");
    }

    // Test peek
    println!("\nTesting peek...");
    let queue3_empty = MessageQueue::new();
    if queue3_empty.peek().is_some() {
        eprintln!("peek() on empty queue should return None");
        passed = false;
    }

    let mut queue3 = MessageQueue::new();
    queue3.enqueue("first".to_string());
    queue3.enqueue("second".to_string());
    if queue3.peek() != Some(&"first".to_string()) {
        eprintln!("peek() should return 'first', got {:?}", queue3.peek());
        passed = false;
    }
    if queue3.size() != 2 {
        eprintln!("peek() should not remove, size should still be 2");
        passed = false;
    } else {
        println!("Peek works correctly");
    }

    // Test dequeue FIFO order
    println!("\nTesting dequeue (FIFO)...");
    let mut queue4 = MessageQueue::new();
    if queue4.dequeue().is_some() {
        eprintln!("dequeue() on empty queue should return None");
        passed = false;
    }

    queue4.enqueue("alpha".to_string());
    queue4.enqueue("beta".to_string());
    queue4.enqueue("gamma".to_string());

    let first = queue4.dequeue();
    if first != Some("alpha".to_string()) {
        eprintln!("First dequeue should return 'alpha', got {:?}", first);
        passed = false;
    }

    let second = queue4.dequeue();
    if second != Some("beta".to_string()) {
        eprintln!("Second dequeue should return 'beta', got {:?}", second);
        passed = false;
    }

    let third = queue4.dequeue();
    if third != Some("gamma".to_string()) {
        eprintln!("Third dequeue should return 'gamma', got {:?}", third);
        passed = false;
    }

    if queue4.dequeue().is_some() {
        eprintln!("Dequeue from empty queue should return None");
        passed = false;
    } else {
        println!("Dequeue FIFO order works correctly");
    }

    // Test process_batch
    println!("\nTesting process_batch...");
    let mut queue5 = MessageQueue::new();
    queue5.enqueue("1".to_string());
    queue5.enqueue("2".to_string());
    queue5.enqueue("3".to_string());
    queue5.enqueue("4".to_string());
    queue5.enqueue("5".to_string());

    let batch1 = process_batch(&mut queue5, 2);
    if batch1.len() != 2 {
        eprintln!("process_batch(queue, 2) should return 2 items, got {}", batch1.len());
        passed = false;
    }
    if batch1 != vec!["1".to_string(), "2".to_string()] {
        eprintln!("process_batch should return ['1', '2'], got {:?}", batch1);
        passed = false;
    }
    if queue5.size() != 3 {
        eprintln!("Queue should have 3 items left, got {}", queue5.size());
        passed = false;
    }

    let batch2 = process_batch(&mut queue5, 5);
    if batch2.len() != 3 {
        eprintln!("process_batch(queue, 5) should return 3 items, got {}", batch2.len());
        passed = false;
    }
    if batch2 != vec!["3".to_string(), "4".to_string(), "5".to_string()] {
        eprintln!("process_batch should return ['3', '4', '5'], got {:?}", batch2);
        passed = false;
    } else {
        println!("process_batch works correctly");
    }

    // Test edge cases
    println!("\nTesting edge cases...");
    let mut queue6 = MessageQueue::new();
    let empty_batch = process_batch(&mut queue6, 10);
    if !empty_batch.is_empty() {
        eprintln!("process_batch on empty queue should return empty vec, got {} items", empty_batch.len());
        passed = false;
    } else {
        println!("Edge cases handled correctly");
    }

    // Test interleaved operations
    println!("\nTesting interleaved operations...");
    let mut queue7 = MessageQueue::new();
    queue7.enqueue("a".to_string());
    queue7.enqueue("b".to_string());
    let _ = queue7.dequeue(); // "a"
    queue7.enqueue("c".to_string());
    let msg = queue7.dequeue(); // "b"
    if msg != Some("b".to_string()) {
        eprintln!("Expected 'b' after interleaved ops, got {:?}", msg);
        passed = false;
    }
    let msg = queue7.dequeue(); // "c"
    if msg != Some("c".to_string()) {
        eprintln!("Expected 'c' after interleaved ops, got {:?}", msg);
        passed = false;
    } else {
        println!("Interleaved operations work correctly");
    }

    if passed {
        println!("\nAll tests passed!");
    } else {
        eprintln!("\nSome tests failed.");
        std::process::exit(1);
    }

    passed
}

fn main() {
    run_check();
}

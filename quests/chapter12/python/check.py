#!/usr/bin/env python3
from messagequeue import MessageQueue, process_batch
import sys


def run_check() -> bool:
    """Run all test cases for Chapter 12."""
    passed = True

    # Test enqueue and size
    print("Testing enqueue and size...")
    queue1 = MessageQueue()
    if queue1.size() != 0:
        print(f"size() on empty queue should be 0, got {queue1.size()}")
        passed = False

    queue1.enqueue("message1")
    if queue1.size() != 1:
        print(f"size() after enqueue should be 1, got {queue1.size()}")
        passed = False

    queue1.enqueue("message2")
    queue1.enqueue("message3")
    if queue1.size() != 3:
        print(f"size() after 3 enqueues should be 3, got {queue1.size()}")
        passed = False

    # Test is_empty
    print("Testing is_empty...")
    queue2 = MessageQueue()
    if not queue2.is_empty():
        print("is_empty() on empty queue should be True")
        passed = False

    queue2.enqueue("test")
    if queue2.is_empty():
        print("is_empty() on non-empty queue should be False")
        passed = False

    # Test peek
    print("Testing peek...")
    queue3 = MessageQueue()
    if queue3.peek() is not None:
        print("peek() on empty queue should return None")
        passed = False

    queue3.enqueue("first")
    queue3.enqueue("second")
    if queue3.peek() != "first":
        print(f"peek() should return first message without removing, got {queue3.peek()}")
        passed = False
    if queue3.size() != 2:
        print("peek() should not remove, size should still be 2")
        passed = False

    # Test dequeue FIFO order
    print("Testing dequeue (FIFO)...")
    queue4 = MessageQueue()
    if queue4.dequeue() is not None:
        print("dequeue() on empty queue should return None")
        passed = False

    queue4.enqueue("alpha")
    queue4.enqueue("beta")
    queue4.enqueue("gamma")

    first = queue4.dequeue()
    if first != "alpha":
        print(f'First dequeue should return "alpha", got {first}')
        passed = False

    second = queue4.dequeue()
    if second != "beta":
        print(f'Second dequeue should return "beta", got {second}')
        passed = False

    third = queue4.dequeue()
    if third != "gamma":
        print(f'Third dequeue should return "gamma", got {third}')
        passed = False

    empty = queue4.dequeue()
    if empty is not None:
        print(f"Dequeue from empty queue should return None, got {empty}")
        passed = False

    # Test process_batch
    print("Testing process_batch...")
    queue5 = MessageQueue()
    queue5.enqueue(1)
    queue5.enqueue(2)
    queue5.enqueue(3)
    queue5.enqueue(4)
    queue5.enqueue(5)

    batch1 = process_batch(queue5, 2)
    if len(batch1) != 2:
        print(f"process_batch(queue, 2) should return 2 items, got {len(batch1)}")
        passed = False
    if batch1[0] != 1 or batch1[1] != 2:
        print(f"process_batch should return [1, 2], got {batch1}")
        passed = False
    if queue5.size() != 3:
        print(f"Queue should have 3 items left, got {queue5.size()}")
        passed = False

    batch2 = process_batch(queue5, 5)
    if len(batch2) != 3:
        print(f"process_batch(queue, 5) should return 3 items, got {len(batch2)}")
        passed = False
    if batch2[0] != 3 or batch2[1] != 4 or batch2[2] != 5:
        print(f"process_batch should return [3, 4, 5], got {batch2}")
        passed = False

    # Test edge cases
    print("Testing edge cases...")
    queue6 = MessageQueue()
    empty_batch = process_batch(queue6, 10)
    if len(empty_batch) != 0:
        print(f"process_batch on empty queue should return empty list, got {len(empty_batch)} items")
        passed = False

    # Test with different data types
    print("Testing with different data types...")
    queue7 = MessageQueue()
    obj = {"id": 1, "name": "test"}
    arr = [1, 2, 3]
    queue7.enqueue(obj)
    queue7.enqueue(arr)
    queue7.enqueue(42)
    queue7.enqueue("string")
    queue7.enqueue(None)

    popped = queue7.dequeue()
    if popped != obj:
        print(f"Dequeued object should match, got {type(popped)}")
        passed = False

    if passed:
        print("✅ All tests passed!")
    else:
        print("❌ Some tests failed.")
        sys.exit(1)

    return passed


if __name__ == "__main__":
    run_check()

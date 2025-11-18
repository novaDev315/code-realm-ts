const { MessageQueue, processBatch } = require("./messagequeue");

function runCheck() {
  let passed = true;

  // Test enqueue and size
  console.log("Testing enqueue and size...");
  const queue1 = new MessageQueue();
  if (queue1.size() !== 0) {
    console.error(`size() on empty queue should be 0, got ${queue1.size()}`);
    passed = false;
  }

  queue1.enqueue("message1");
  if (queue1.size() !== 1) {
    console.error(`size() after enqueue should be 1, got ${queue1.size()}`);
    passed = false;
  }

  queue1.enqueue("message2");
  queue1.enqueue("message3");
  if (queue1.size() !== 3) {
    console.error(`size() after 3 enqueues should be 3, got ${queue1.size()}`);
    passed = false;
  }

  // Test isEmpty
  console.log("Testing isEmpty...");
  const queue2 = new MessageQueue();
  if (!queue2.isEmpty()) {
    console.error(`isEmpty() on empty queue should be true`);
    passed = false;
  }

  queue2.enqueue("test");
  if (queue2.isEmpty()) {
    console.error(`isEmpty() on non-empty queue should be false`);
    passed = false;
  }

  // Test peek
  console.log("Testing peek...");
  const queue3 = new MessageQueue();
  if (queue3.peek() !== null) {
    console.error(`peek() on empty queue should return null`);
    passed = false;
  }

  queue3.enqueue("first");
  queue3.enqueue("second");
  if (queue3.peek() !== "first") {
    console.error(`peek() should return first message without removing, got ${queue3.peek()}`);
    passed = false;
  }
  if (queue3.size() !== 2) {
    console.error(`peek() should not remove, size should still be 2`);
    passed = false;
  }

  // Test dequeue FIFO order
  console.log("Testing dequeue (FIFO)...");
  const queue4 = new MessageQueue();
  if (queue4.dequeue() !== null) {
    console.error(`dequeue() on empty queue should return null`);
    passed = false;
  }

  queue4.enqueue("alpha");
  queue4.enqueue("beta");
  queue4.enqueue("gamma");

  const first = queue4.dequeue();
  if (first !== "alpha") {
    console.error(`First dequeue should return "alpha", got ${first}`);
    passed = false;
  }

  const second = queue4.dequeue();
  if (second !== "beta") {
    console.error(`Second dequeue should return "beta", got ${second}`);
    passed = false;
  }

  const third = queue4.dequeue();
  if (third !== "gamma") {
    console.error(`Third dequeue should return "gamma", got ${third}`);
    passed = false;
  }

  const empty = queue4.dequeue();
  if (empty !== null) {
    console.error(`Dequeue from empty queue should return null, got ${empty}`);
    passed = false;
  }

  // Test processBatch
  console.log("Testing processBatch...");
  const queue5 = new MessageQueue();
  queue5.enqueue(1);
  queue5.enqueue(2);
  queue5.enqueue(3);
  queue5.enqueue(4);
  queue5.enqueue(5);

  const batch1 = processBatch(queue5, 2);
  if (batch1.length !== 2) {
    console.error(`processBatch(queue, 2) should return 2 items, got ${batch1.length}`);
    passed = false;
  }
  if (batch1[0] !== 1 || batch1[1] !== 2) {
    console.error(`processBatch should return [1, 2], got [${batch1.join(", ")}]`);
    passed = false;
  }
  if (queue5.size() !== 3) {
    console.error(`Queue should have 3 items left, got ${queue5.size()}`);
    passed = false;
  }

  const batch2 = processBatch(queue5, 5);
  if (batch2.length !== 3) {
    console.error(`processBatch(queue, 5) should return 3 items, got ${batch2.length}`);
    passed = false;
  }
  if (batch2[0] !== 3 || batch2[1] !== 4 || batch2[2] !== 5) {
    console.error(`processBatch should return [3, 4, 5], got [${batch2.join(", ")}]`);
    passed = false;
  }

  // Test edge cases
  console.log("Testing edge cases...");
  const queue6 = new MessageQueue();
  const emptyBatch = processBatch(queue6, 10);
  if (emptyBatch.length !== 0) {
    console.error(`processBatch on empty queue should return empty array, got ${emptyBatch.length} items`);
    passed = false;
  }

  // Test with different data types
  console.log("Testing with different data types...");
  const queue7 = new MessageQueue();
  const obj = { id: 1, name: "test" };
  const arr = [1, 2, 3];
  queue7.enqueue(obj);
  queue7.enqueue(arr);
  queue7.enqueue(42);
  queue7.enqueue("string");
  queue7.enqueue(null);

  const popped = queue7.dequeue();
  if (popped !== obj) {
    console.error(`Dequeued object should match, got ${typeof popped}`);
    passed = false;
  }

  if (passed) {
    console.log("✅ All tests passed!");
  } else {
    console.log("❌ Some tests failed.");
    process.exit(1);
  }

  return passed;
}

runCheck();

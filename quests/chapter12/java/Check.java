// Test file for Chapter 12: Hall of Echoes - Message Queues

public class Check {
    public static void main(String[] args) {
        boolean passed = true;

        // Test enqueue and size
        System.out.println("Testing enqueue and size...");
        MessageQueue queue1 = new MessageQueue();

        if (queue1.size() != 0) {
            System.err.println("size() on empty queue: expected 0, got " + queue1.size());
            passed = false;
        } else {
            System.out.println("  size() on empty queue = 0");
        }

        queue1.enqueue("message1");
        if (queue1.size() != 1) {
            System.err.println("size() after 1 enqueue: expected 1, got " + queue1.size());
            passed = false;
        } else {
            System.out.println("  size() after 1 enqueue = 1");
        }

        queue1.enqueue("message2");
        queue1.enqueue("message3");
        if (queue1.size() != 3) {
            System.err.println("size() after 3 enqueues: expected 3, got " + queue1.size());
            passed = false;
        } else {
            System.out.println("  size() after 3 enqueues = 3");
        }

        // Test isEmpty
        System.out.println("\nTesting isEmpty...");
        MessageQueue queue2 = new MessageQueue();

        if (!queue2.isEmpty()) {
            System.err.println("isEmpty() on empty queue: expected true, got false");
            passed = false;
        } else {
            System.out.println("  isEmpty() on empty queue = true");
        }

        queue2.enqueue("test");
        if (queue2.isEmpty()) {
            System.err.println("isEmpty() on non-empty queue: expected false, got true");
            passed = false;
        } else {
            System.out.println("  isEmpty() on non-empty queue = false");
        }

        // Test peek
        System.out.println("\nTesting peek...");
        MessageQueue queue3 = new MessageQueue();

        if (queue3.peek() != null) {
            System.err.println("peek() on empty queue: expected null, got " + queue3.peek());
            passed = false;
        } else {
            System.out.println("  peek() on empty queue = null");
        }

        queue3.enqueue("first");
        queue3.enqueue("second");
        String peeked = queue3.peek();
        if (!"first".equals(peeked)) {
            System.err.println("peek() should return first message: expected \"first\", got \"" + peeked + "\"");
            passed = false;
        } else {
            System.out.println("  peek() returns first message = \"first\"");
        }

        if (queue3.size() != 2) {
            System.err.println("peek() should not remove: expected size 2, got " + queue3.size());
            passed = false;
        } else {
            System.out.println("  peek() does not remove (size still 2)");
        }

        // Test dequeue FIFO order
        System.out.println("\nTesting dequeue (FIFO order)...");
        MessageQueue queue4 = new MessageQueue();

        if (queue4.dequeue() != null) {
            System.err.println("dequeue() on empty queue: expected null");
            passed = false;
        } else {
            System.out.println("  dequeue() on empty queue = null");
        }

        queue4.enqueue("alpha");
        queue4.enqueue("beta");
        queue4.enqueue("gamma");

        String first = queue4.dequeue();
        if (!"alpha".equals(first)) {
            System.err.println("First dequeue: expected \"alpha\", got \"" + first + "\"");
            passed = false;
        } else {
            System.out.println("  First dequeue = \"alpha\"");
        }

        String second = queue4.dequeue();
        if (!"beta".equals(second)) {
            System.err.println("Second dequeue: expected \"beta\", got \"" + second + "\"");
            passed = false;
        } else {
            System.out.println("  Second dequeue = \"beta\"");
        }

        String third = queue4.dequeue();
        if (!"gamma".equals(third)) {
            System.err.println("Third dequeue: expected \"gamma\", got \"" + third + "\"");
            passed = false;
        } else {
            System.out.println("  Third dequeue = \"gamma\"");
        }

        String empty = queue4.dequeue();
        if (empty != null) {
            System.err.println("Dequeue from empty: expected null, got \"" + empty + "\"");
            passed = false;
        } else {
            System.out.println("  Dequeue from empty queue = null");
        }

        // Test processBatch
        System.out.println("\nTesting processBatch...");
        MessageQueue queue5 = new MessageQueue();
        queue5.enqueue("1");
        queue5.enqueue("2");
        queue5.enqueue("3");
        queue5.enqueue("4");
        queue5.enqueue("5");

        String[] batch1 = MessageProcessor.processBatch(queue5, 2);
        if (batch1.length != 2) {
            System.err.println("processBatch(queue, 2): expected 2 items, got " + batch1.length);
            passed = false;
        } else if (!"1".equals(batch1[0]) || !"2".equals(batch1[1])) {
            System.err.println("processBatch should return [\"1\", \"2\"], got [\"" + batch1[0] + "\", \"" + batch1[1] + "\"]");
            passed = false;
        } else {
            System.out.println("  processBatch(queue, 2) = [\"1\", \"2\"]");
        }

        if (queue5.size() != 3) {
            System.err.println("Queue should have 3 items left, got " + queue5.size());
            passed = false;
        } else {
            System.out.println("  Queue has 3 items remaining");
        }

        String[] batch2 = MessageProcessor.processBatch(queue5, 5);
        if (batch2.length != 3) {
            System.err.println("processBatch(queue, 5): expected 3 items, got " + batch2.length);
            passed = false;
        } else if (!"3".equals(batch2[0]) || !"4".equals(batch2[1]) || !"5".equals(batch2[2])) {
            System.err.println("processBatch should return [\"3\", \"4\", \"5\"]");
            passed = false;
        } else {
            System.out.println("  processBatch(queue, 5) = [\"3\", \"4\", \"5\"] (only 3 available)");
        }

        // Test edge cases
        System.out.println("\nTesting edge cases...");
        MessageQueue queue6 = new MessageQueue();
        String[] emptyBatch = MessageProcessor.processBatch(queue6, 10);
        if (emptyBatch.length != 0) {
            System.err.println("processBatch on empty queue: expected 0 items, got " + emptyBatch.length);
            passed = false;
        } else {
            System.out.println("  processBatch on empty queue = [] (empty array)");
        }

        // Test size tracking after mixed operations
        System.out.println("\nTesting size tracking after mixed operations...");
        MessageQueue queue7 = new MessageQueue();
        queue7.enqueue("a");
        queue7.enqueue("b");
        queue7.dequeue();
        queue7.enqueue("c");
        queue7.dequeue();

        if (queue7.size() != 1) {
            System.err.println("Size after mixed operations: expected 1, got " + queue7.size());
            passed = false;
        } else {
            System.out.println("  Size after enqueue(a), enqueue(b), dequeue, enqueue(c), dequeue = 1");
        }

        String remaining = queue7.peek();
        if (!"c".equals(remaining)) {
            System.err.println("Remaining element should be \"c\", got \"" + remaining + "\"");
            passed = false;
        } else {
            System.out.println("  Remaining element is \"c\"");
        }

        if (passed) {
            System.out.println("\nAll tests passed!");
        } else {
            System.out.println("\nSome tests failed.");
            System.exit(1);
        }
    }
}

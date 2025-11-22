// Reference solutions for Chapter 12: Hall of Echoes - Message Queues
// FIFO Message Queue implementation

import java.util.LinkedList;
import java.util.ArrayList;
import java.util.List;

/**
 * MessageQueue - SOLUTION
 *
 * A FIFO (First In, First Out) message queue implementation.
 * Uses LinkedList for efficient add/remove operations at both ends.
 */
public class SOLUTIONS {
    private LinkedList<String> queue = new LinkedList<>();

    /**
     * Add a message to the end of the queue - SOLUTION
     *
     * LinkedList.addLast() or add() appends to the end in O(1) time.
     *
     * @param message The message to add to the queue
     */
    public void enqueue(String message) {
        queue.addLast(message);
    }

    /**
     * Remove and return the first message from the queue - SOLUTION
     *
     * LinkedList.pollFirst() removes and returns the first element,
     * or returns null if the list is empty. O(1) time complexity.
     *
     * @return The first message, or null if empty
     */
    public String dequeue() {
        if (queue.isEmpty()) {
            return null;
        }
        return queue.removeFirst();
    }

    /**
     * Return the first message without removing it - SOLUTION
     *
     * LinkedList.peekFirst() returns the first element without removing,
     * or returns null if the list is empty. O(1) time complexity.
     *
     * @return The first message, or null if empty
     */
    public String peek() {
        if (queue.isEmpty()) {
            return null;
        }
        return queue.getFirst();
    }

    /**
     * Return the current number of messages in the queue - SOLUTION
     *
     * @return The number of messages in the queue
     */
    public int size() {
        return queue.size();
    }

    /**
     * Check if the queue is empty - SOLUTION
     *
     * @return true if the queue has no messages, false otherwise
     */
    public boolean isEmpty() {
        return queue.isEmpty();
    }

    /**
     * Process up to batchSize messages from the queue - SOLUTION
     *
     * Dequeues messages one by one until either:
     * - batchSize messages have been processed, or
     * - The queue becomes empty
     *
     * @param queue The message queue to process
     * @param batchSize Maximum number of messages to process
     * @return Array of processed messages
     */
    public static String[] processBatch(SOLUTIONS queue, int batchSize) {
        List<String> batch = new ArrayList<>();

        for (int i = 0; i < batchSize && !queue.isEmpty(); i++) {
            String message = queue.dequeue();
            if (message != null) {
                batch.add(message);
            }
        }

        return batch.toArray(new String[0]);
    }
}

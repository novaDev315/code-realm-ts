// Chapter 12: Hall of Echoes - Message Queues
// A message queue implements FIFO (First In, First Out) principle
// for handling asynchronous communication and batch processing

#include <string>
#include <queue>
#include <deque>
#include <vector>

/**
 * MessageQueue class - FIFO queue for message processing
 *
 * You can use std::queue<std::string> or std::deque<std::string> for storage.
 * std::queue is a container adapter that provides FIFO operations.
 * std::deque provides direct access to both ends with efficient operations.
 */
class MessageQueue {
private:
    // Storage for queue messages
    // You can use either std::queue or std::deque
    std::deque<std::string> queue;

public:
    /**
     * Add a message to the end of the queue
     *
     * TODO: Implement enqueue
     * - Add the message to the end of the queue
     *
     * Example:
     *   queue.enqueue("msg1")
     *   queue.enqueue("msg2")
     *   // Queue is now: ["msg1", "msg2"]
     *
     * @param message The message to add to the queue
     */
    void enqueue(const std::string& message) {
        // TODO: Add message to the end of the queue
        throw std::runtime_error("Not implemented yet");
    }

    /**
     * Remove and return the first message from the queue
     *
     * TODO: Implement dequeue
     * - Return empty string if the queue is empty
     * - Remove and return the first message (FIFO order)
     *
     * Example:
     *   // Queue: ["msg1", "msg2"]
     *   queue.dequeue() // returns "msg1"
     *   // Queue is now: ["msg2"]
     *
     * @return The first message, or empty string if empty
     */
    std::string dequeue() {
        // TODO: Remove and return first message, return empty string if empty
        return "";
    }

    /**
     * Return the first message without removing it
     *
     * TODO: Implement peek
     * - Return empty string if the queue is empty
     * - Return the first message without removing it
     *
     * Example:
     *   // Queue: ["msg1", "msg2"]
     *   queue.peek() // returns "msg1"
     *   // Queue is still: ["msg1", "msg2"]
     *
     * @return The first message, or empty string if empty
     */
    std::string peek() const {
        // TODO: Return first message without removing, return empty string if empty
        return "";
    }

    /**
     * Return the current number of messages in the queue
     *
     * TODO: Implement size
     *
     * @return The number of messages in the queue
     */
    size_t size() const {
        // TODO: Return current queue size
        return 0;
    }

    /**
     * Check if the queue is empty
     *
     * TODO: Implement isEmpty
     *
     * @return true if the queue has no messages, false otherwise
     */
    bool isEmpty() const {
        // TODO: Return true if queue is empty
        return true;
    }
};

/**
 * Process up to batchSize messages from the queue
 *
 * TODO: Implement processBatch
 * - Dequeue up to batchSize messages
 * - Stop early if queue becomes empty
 * - Return the dequeued messages as a vector
 *
 * @param queue The message queue to process
 * @param batchSize Maximum number of messages to process
 * @return Vector of processed messages
 */
std::vector<std::string> processBatch(MessageQueue& queue, size_t batchSize) {
    // TODO: Dequeue up to batchSize messages and return them
    return std::vector<std::string>();
}

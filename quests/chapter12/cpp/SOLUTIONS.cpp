// Reference solutions for Chapter 12: Hall of Echoes - Message Queues
// FIFO Message Queue implementation

#include <string>
#include <queue>
#include <deque>
#include <vector>

/**
 * MessageQueue class - SOLUTION
 *
 * A FIFO (First In, First Out) message queue implementation.
 * Uses std::deque for efficient add/remove operations at both ends.
 *
 * Time Complexity:
 * - enqueue: O(1) amortized
 * - dequeue: O(1)
 * - peek: O(1)
 * - size: O(1)
 * - isEmpty: O(1)
 */
class MessageQueue {
private:
    std::deque<std::string> queue;

public:
    /**
     * Add a message to the end of the queue - SOLUTION
     *
     * std::deque::push_back() adds element to the end in O(1) amortized time.
     *
     * @param message The message to add to the queue
     */
    void enqueue(const std::string& message) {
        queue.push_back(message);
    }

    /**
     * Remove and return the first message from the queue - SOLUTION
     *
     * std::deque provides front() to access and pop_front() to remove
     * the first element, both in O(1) time.
     *
     * @return The first message, or empty string if empty
     */
    std::string dequeue() {
        if (queue.empty()) {
            return "";
        }
        std::string front = queue.front();
        queue.pop_front();
        return front;
    }

    /**
     * Return the first message without removing it - SOLUTION
     *
     * std::deque::front() returns reference to first element in O(1) time.
     *
     * @return The first message, or empty string if empty
     */
    std::string peek() const {
        if (queue.empty()) {
            return "";
        }
        return queue.front();
    }

    /**
     * Return the current number of messages in the queue - SOLUTION
     *
     * @return The number of messages in the queue
     */
    size_t size() const {
        return queue.size();
    }

    /**
     * Check if the queue is empty - SOLUTION
     *
     * @return true if the queue has no messages, false otherwise
     */
    bool isEmpty() const {
        return queue.empty();
    }
};

/**
 * Process up to batchSize messages from the queue - SOLUTION
 *
 * Dequeues messages one by one until either:
 * - batchSize messages have been processed, or
 * - The queue becomes empty
 *
 * @param queue The message queue to process
 * @param batchSize Maximum number of messages to process
 * @return Vector of processed messages
 */
std::vector<std::string> processBatch(MessageQueue& queue, size_t batchSize) {
    std::vector<std::string> batch;

    for (size_t i = 0; i < batchSize && !queue.isEmpty(); i++) {
        std::string message = queue.dequeue();
        if (!message.empty()) {
            batch.push_back(message);
        }
    }

    return batch;
}

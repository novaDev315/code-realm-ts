// Reference solutions for Chapter 12: Message Queues
// This file contains working implementations for all queue operations

package main

// MessageQueue represents a FIFO queue for message processing
type MessageQueue struct {
	queue []string
}

// NewMessageQueue creates a new empty message queue
func NewMessageQueue() *MessageQueue {
	return &MessageQueue{
		queue: make([]string, 0),
	}
}

// Enqueue adds a message to the back of the queue
func (q *MessageQueue) Enqueue(message string) {
	q.queue = append(q.queue, message)
}

// Dequeue removes and returns the first message from the queue
// Returns the message and true if successful, empty string and false if queue is empty
func (q *MessageQueue) Dequeue() (string, bool) {
	if len(q.queue) == 0 {
		return "", false
	}
	message := q.queue[0]
	q.queue = q.queue[1:]
	return message, true
}

// Peek returns the first message without removing it
// Returns the message and true if successful, empty string and false if queue is empty
func (q *MessageQueue) Peek() (string, bool) {
	if len(q.queue) == 0 {
		return "", false
	}
	return q.queue[0], true
}

// Size returns the current number of messages in the queue
func (q *MessageQueue) Size() int {
	return len(q.queue)
}

// IsEmpty returns true if the queue has no messages
func (q *MessageQueue) IsEmpty() bool {
	return len(q.queue) == 0
}

// ProcessBatch dequeues up to batchSize messages and returns them
func ProcessBatch(q *MessageQueue, batchSize int) []string {
	batch := make([]string, 0)
	for i := 0; i < batchSize && !q.IsEmpty(); i++ {
		if message, ok := q.Dequeue(); ok {
			batch = append(batch, message)
		}
	}
	return batch
}

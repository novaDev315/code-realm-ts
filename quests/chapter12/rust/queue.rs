// Chapter 12: Hall of Echoes - Message Queues
// A message queue implements FIFO (First In, First Out) principle
// for handling asynchronous communication and batch processing

use std::collections::VecDeque;

/// MessageQueue represents a FIFO queue for message processing
pub struct MessageQueue {
    // Using VecDeque for efficient front removal
    queue: VecDeque<String>,
}

impl MessageQueue {
    /// Creates a new empty message queue
    pub fn new() -> Self {
        MessageQueue {
            // TODO: Initialize the VecDeque
            queue: VecDeque::new(),
        }
    }

    /// Adds a message to the back of the queue
    pub fn enqueue(&mut self, message: String) {
        // TODO: Add message to the back of the queue
    }

    /// Removes and returns the first message from the queue
    /// Returns Some(message) if successful, None if queue is empty
    pub fn dequeue(&mut self) -> Option<String> {
        // TODO: Remove and return the first message
        // Return None if the queue is empty
        None
    }

    /// Returns a reference to the first message without removing it
    /// Returns Some(&message) if successful, None if queue is empty
    pub fn peek(&self) -> Option<&String> {
        // TODO: Return the first message without removing it
        // Return None if the queue is empty
        None
    }

    /// Returns the current number of messages in the queue
    pub fn size(&self) -> usize {
        // TODO: Return the queue size
        0
    }

    /// Returns true if the queue has no messages
    pub fn is_empty(&self) -> bool {
        // TODO: Return true if queue is empty
        true
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Dequeues up to batch_size messages and returns them
pub fn process_batch(queue: &mut MessageQueue, batch_size: usize) -> Vec<String> {
    // TODO: Dequeue up to batch_size messages and return them as a Vec
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = MessageQueue::new();
        queue.enqueue("first".to_string());
        queue.enqueue("second".to_string());
        assert_eq!(queue.dequeue(), Some("first".to_string()));
        assert_eq!(queue.dequeue(), Some("second".to_string()));
    }

    #[test]
    fn test_empty_queue() {
        let mut queue = MessageQueue::new();
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.peek(), None);
        assert!(queue.is_empty());
    }
}

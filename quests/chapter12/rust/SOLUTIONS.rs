// Reference solutions for Chapter 12: Message Queues
// This file contains working implementations for all queue operations

use std::collections::VecDeque;

/// MessageQueue represents a FIFO queue for message processing
pub struct MessageQueue {
    queue: VecDeque<String>,
}

impl MessageQueue {
    /// Creates a new empty message queue
    pub fn new() -> Self {
        MessageQueue {
            queue: VecDeque::new(),
        }
    }

    /// Adds a message to the back of the queue
    pub fn enqueue(&mut self, message: String) {
        self.queue.push_back(message);
    }

    /// Removes and returns the first message from the queue
    /// Returns Some(message) if successful, None if queue is empty
    pub fn dequeue(&mut self) -> Option<String> {
        self.queue.pop_front()
    }

    /// Returns a reference to the first message without removing it
    /// Returns Some(&message) if successful, None if queue is empty
    pub fn peek(&self) -> Option<&String> {
        self.queue.front()
    }

    /// Returns the current number of messages in the queue
    pub fn size(&self) -> usize {
        self.queue.len()
    }

    /// Returns true if the queue has no messages
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Dequeues up to batch_size messages and returns them
pub fn process_batch(queue: &mut MessageQueue, batch_size: usize) -> Vec<String> {
    let mut batch = Vec::new();
    for _ in 0..batch_size {
        if queue.is_empty() {
            break;
        }
        if let Some(message) = queue.dequeue() {
            batch.push(message);
        }
    }
    batch
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

    #[test]
    fn test_peek() {
        let mut queue = MessageQueue::new();
        queue.enqueue("first".to_string());
        queue.enqueue("second".to_string());
        assert_eq!(queue.peek(), Some(&"first".to_string()));
        assert_eq!(queue.size(), 2); // peek doesn't remove
    }

    #[test]
    fn test_size_and_is_empty() {
        let mut queue = MessageQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);

        queue.enqueue("a".to_string());
        assert!(!queue.is_empty());
        assert_eq!(queue.size(), 1);

        queue.enqueue("b".to_string());
        assert_eq!(queue.size(), 2);

        queue.dequeue();
        assert_eq!(queue.size(), 1);
    }

    #[test]
    fn test_process_batch() {
        let mut queue = MessageQueue::new();
        queue.enqueue("1".to_string());
        queue.enqueue("2".to_string());
        queue.enqueue("3".to_string());
        queue.enqueue("4".to_string());
        queue.enqueue("5".to_string());

        let batch1 = process_batch(&mut queue, 2);
        assert_eq!(batch1, vec!["1".to_string(), "2".to_string()]);
        assert_eq!(queue.size(), 3);

        let batch2 = process_batch(&mut queue, 5);
        assert_eq!(batch2, vec!["3".to_string(), "4".to_string(), "5".to_string()]);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_fifo_order() {
        let mut queue = MessageQueue::new();
        queue.enqueue("alpha".to_string());
        queue.enqueue("beta".to_string());
        queue.enqueue("gamma".to_string());

        assert_eq!(queue.dequeue(), Some("alpha".to_string()));
        assert_eq!(queue.dequeue(), Some("beta".to_string()));
        assert_eq!(queue.dequeue(), Some("gamma".to_string()));
        assert_eq!(queue.dequeue(), None);
    }
}

# Chapter 12: Hall of Echoes - Message Queues
# Reference solutions for message queue implementation


class MessageQueue:
    """A simple FIFO message queue implementation."""

    def __init__(self):
        self.queue = []

    def enqueue(self, message):
        """Add a message to the end of the queue."""
        self.queue.append(message)

    def dequeue(self):
        """Remove and return the first message, or None if empty."""
        if len(self.queue) == 0:
            return None
        return self.queue.pop(0)

    def peek(self):
        """Return the first message without removing it, or None if empty."""
        if len(self.queue) == 0:
            return None
        return self.queue[0]

    def size(self) -> int:
        """Return the current number of messages in the queue."""
        return len(self.queue)

    def is_empty(self) -> bool:
        """Return True if the queue is empty, False otherwise."""
        return len(self.queue) == 0


def process_batch(queue: MessageQueue, batch_size: int) -> list:
    """Dequeue up to batch_size messages and return them as a list."""
    batch = []
    for i in range(batch_size):
        if queue.is_empty():
            break
        message = queue.dequeue()
        if message is not None:
            batch.append(message)
    return batch

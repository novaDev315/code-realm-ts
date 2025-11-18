// Chapter 12: Hall of Echoes - Message Queues
// A message queue implements FIFO (First In, First Out) principle
// for handling asynchronous communication and batch processing

class MessageQueue {
  constructor() {
    this.queue = [];
  }

  enqueue(message) {
    // TODO: Add message to queue
  }

  dequeue() {
    // TODO: Remove and return first message, null if empty
    return null;
  }

  peek() {
    // TODO: Return first message without removing, null if empty
    return null;
  }

  size() {
    // TODO: Return current queue size
    return 0;
  }

  isEmpty() {
    // TODO: Return true if queue is empty
    return true;
  }
}

function processBatch(queue, batchSize) {
  // TODO: Dequeue up to batchSize messages and return them
  return [];
}

module.exports = {
  MessageQueue,
  processBatch
};

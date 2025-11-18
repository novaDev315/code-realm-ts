// Chapter 12: Hall of Echoes - Message Queues
// Reference solutions for message queue implementation

class MessageQueue {
  constructor() {
    this.queue = [];
  }

  enqueue(message) {
    this.queue.push(message);
  }

  dequeue() {
    if (this.queue.length === 0) {
      return null;
    }
    return this.queue.shift();
  }

  peek() {
    if (this.queue.length === 0) {
      return null;
    }
    return this.queue[0];
  }

  size() {
    return this.queue.length;
  }

  isEmpty() {
    return this.queue.length === 0;
  }
}

function processBatch(queue, batchSize) {
  const batch = [];
  for (let i = 0; i < batchSize && !queue.isEmpty(); i++) {
    const message = queue.dequeue();
    if (message !== null) {
      batch.push(message);
    }
  }
  return batch;
}

module.exports = {
  MessageQueue,
  processBatch
};

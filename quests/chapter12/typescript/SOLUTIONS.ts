// Chapter 12: Hall of Echoes - Message Queues
// Reference solutions for message queue implementation

export class MessageQueue {
  private queue: any[] = [];

  enqueue(message: any): void {
    this.queue.push(message);
  }

  dequeue(): any | null {
    if (this.queue.length === 0) {
      return null;
    }
    return this.queue.shift();
  }

  peek(): any | null {
    if (this.queue.length === 0) {
      return null;
    }
    return this.queue[0];
  }

  size(): number {
    return this.queue.length;
  }

  isEmpty(): boolean {
    return this.queue.length === 0;
  }
}

export function processBatch(queue: MessageQueue, batchSize: number): any[] {
  const batch: any[] = [];
  for (let i = 0; i < batchSize && !queue.isEmpty(); i++) {
    const message = queue.dequeue();
    if (message !== null) {
      batch.push(message);
    }
  }
  return batch;
}

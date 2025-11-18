// Reference solutions for Chapter 14: Gate of Trials - Load Balancing
// These are working implementations for testing purposes

export class RoundRobinBalancer {
  private servers: string[] = [];
  private currentIndex: number = 0;

  constructor(servers: string[]) {
    this.servers = servers;
  }

  getNextServer(): string | null {
    // Return next server in round-robin fashion, wrapping around when reaching end
    if (this.servers.length === 0) {
      return null;
    }

    const server = this.servers[this.currentIndex];
    this.currentIndex = (this.currentIndex + 1) % this.servers.length;
    return server;
  }
}

export class CircuitBreaker {
  private failureCount: number = 0;
  private state: 'CLOSED' | 'OPEN' | 'HALF_OPEN' = 'CLOSED';
  private threshold: number;

  constructor(threshold: number) {
    this.threshold = threshold;
  }

  call(fn: () => boolean): boolean {
    // Circuit breaker state machine:
    // CLOSED: Normal operation, execute function
    // OPEN: Too many failures, reject immediately
    // HALF_OPEN: Testing if service recovered

    if (this.state === 'OPEN') {
      // Circuit is open, reject call
      return false;
    }

    try {
      const result = fn();

      if (!result) {
        // Failure occurred
        this.failureCount++;

        if (this.failureCount >= this.threshold) {
          // Threshold exceeded, open the circuit
          this.state = 'OPEN';
        }
      } else {
        // Success, reset failure count
        this.failureCount = 0;
      }

      return result;
    } catch (error) {
      // Exception counts as failure
      this.failureCount++;

      if (this.failureCount >= this.threshold) {
        this.state = 'OPEN';
      }

      return false;
    }
  }

  getState(): string {
    return this.state;
  }
}

export function rateLimitSlidingWindow(requests: number[], windowSize: number, limit: number): boolean {
  // Sliding window rate limiting:
  // Count requests within the window and return true if limit exceeded

  if (requests.length === 0) {
    return false;
  }

  // Get the current time (use the latest request as reference)
  const now = requests[requests.length - 1];
  const windowStart = now - windowSize;

  // Count requests within the sliding window
  const requestsInWindow = requests.filter((timestamp) => timestamp > windowStart).length;

  // Return true if limit is exceeded
  return requestsInWindow > limit;
}

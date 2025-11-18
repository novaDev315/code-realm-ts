// Reference solutions for Chapter 15: Core of the Architect - FINAL BOSS
// Comprehensive distributed system combining LRU Cache, Message Queue, and Load Balancing

// ===== LRU Cache Implementation =====
class LRUCache {
  private capacity: number;
  private cache: Map<string, any> = new Map();

  constructor(capacity: number) {
    this.capacity = capacity;
  }

  get(key: string): any | null {
    if (!this.cache.has(key)) {
      return null;
    }

    const value = this.cache.get(key);
    this.cache.delete(key);
    this.cache.set(key, value);
    return value;
  }

  put(key: string, value: any): void {
    if (this.cache.has(key)) {
      this.cache.delete(key);
    } else if (this.cache.size >= this.capacity) {
      const firstKey = this.cache.keys().next().value!;
      this.cache.delete(firstKey);
    }
    this.cache.set(key, value);
  }

  size(): number {
    return this.cache.size;
  }

  has(key: string): boolean {
    return this.cache.has(key);
  }
}

// ===== Message Queue Implementation =====
class MessageQueue {
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

// ===== Load Balancer Implementation =====
class RoundRobinBalancer {
  private servers: string[] = [];
  private currentIndex: number = 0;

  constructor(servers: string[]) {
    this.servers = servers;
  }

  getNextServer(): string | null {
    if (this.servers.length === 0) {
      return null;
    }
    const server = this.servers[this.currentIndex];
    this.currentIndex = (this.currentIndex + 1) % this.servers.length;
    return server;
  }
}

// ===== Distributed System =====
export class DistributedSystem {
  private cache: LRUCache;
  private queue: MessageQueue;
  private balancer: RoundRobinBalancer;
  private cacheHits: number = 0;
  private totalRequests: number = 0;
  private servers: string[];

  constructor() {
    this.cache = new LRUCache(100);
    this.queue = new MessageQueue();
    this.servers = ["server-1", "server-2", "server-3", "server-4"];
    this.balancer = new RoundRobinBalancer(this.servers);
  }

  processRequest(request: { id: string; data: any; priority: number }): any {
    this.totalRequests++;

    // Check cache first
    if (this.cache.has(request.id)) {
      const cachedResult = this.cache.get(request.id);
      this.cacheHits++;
      return {
        requestId: request.id,
        result: cachedResult,
        server: "cache",
        cachedFromCache: true,
        processingTime: 1
      };
    }

    // Add to queue
    this.queue.enqueue({
      ...request,
      timestamp: Date.now()
    });

    // Get next server for load balancing
    const server = this.balancer.getNextServer();

    // Simulate processing and cache result
    const result = {
      data: request.data,
      processedAt: new Date().toISOString(),
      server: server
    };

    this.cache.put(request.id, result);

    return {
      requestId: request.id,
      result: result,
      server: server,
      cachedFromCache: false,
      processingTime: Math.random() * 100
    };
  }

  getMetrics(): { cacheHits: number; cacheSize: number; queueSize: number; totalRequests: number } {
    return {
      cacheHits: this.cacheHits,
      cacheSize: this.cache.size(),
      queueSize: this.queue.size(),
      totalRequests: this.totalRequests
    };
  }

  processQueue(): void {
    let processed = 0;
    while (processed < 10 && !this.queue.isEmpty()) {
      const message = this.queue.dequeue();
      if (message) {
        // Simulate processing
        const result = {
          data: message.data,
          processedAt: new Date().toISOString()
        };
        this.cache.put(message.id, result);
        processed++;
      }
    }
  }
}

// ===== System Design Function =====
export function designScalableAPI(requirements: {
  expectedRPS: number;
  dataSize: string;
  caching: boolean;
  auth: boolean;
}): { components: string[]; estimated_servers: number; architecture: string } {
  const components: string[] = [];
  let estimated_servers = 1;

  // Add components based on requirements
  components.push("web-server");
  components.push("database");

  // High RPS requires load balancing
  if (requirements.expectedRPS > 1000) {
    components.push("load-balancer");
    estimated_servers = Math.ceil(requirements.expectedRPS / 500);
  } else if (requirements.expectedRPS > 500) {
    components.push("load-balancer");
    estimated_servers = 2;
  } else if (requirements.expectedRPS > 100) {
    estimated_servers = 1;
  }

  // Large data requires caching
  if (requirements.dataSize === "large" || requirements.caching) {
    components.push("cache");
    components.push("message-queue");
  }

  // Authentication requires auth middleware
  if (requirements.auth) {
    components.push("auth-service");
    components.push("token-validator");
  }

  // Build architecture description
  let architecture = "";

  if (components.includes("load-balancer")) {
    architecture += "Distributed n-tier architecture with ";
    if (components.includes("cache")) {
      architecture += "multi-layer caching, ";
    }
    if (components.includes("message-queue")) {
      architecture += "async message queue processing, ";
    }
    architecture += `and ${estimated_servers} servers running in parallel with round-robin load balancing.`;

    if (components.includes("auth-service")) {
      architecture += " Includes authentication middleware for security.";
    }
  } else {
    architecture = "Single-server architecture with ";
    if (components.includes("cache")) {
      architecture += "local cache for performance optimization. ";
    }
    if (components.includes("auth-service")) {
      architecture += "Auth service for security.";
    } else {
      architecture += "basic request handling.";
    }
  }

  return {
    components,
    estimated_servers,
    architecture
  };
}

// ===== Database Optimization Function =====
export function optimizeDatabase(queries: string[]): { slowQueries: string[]; suggestions: string[] } {
  const slowQueries: string[] = [];
  const suggestions: string[] = [];

  // Identify and track slow patterns
  const patterns = [
    { regex: /SELECT \*/, issue: "SELECT *", suggestion: "Specify only needed columns for better performance" },
    { regex: /SELECT .* FROM .* WHERE [^=]*<>/, issue: "Inequality comparison", suggestion: "Add indexes on comparison columns" },
    { regex: /SELECT .* FROM .* JOIN .* ON/, issue: "JOIN without proper index", suggestion: "Ensure JOIN columns are indexed" },
    { regex: /WHERE NOT IN/, issue: "NOT IN clause", suggestion: "Consider using NOT EXISTS with proper indexes" },
    { regex: /YEAR\(/, issue: "Function on column", suggestion: "Avoid functions on indexed columns for better query optimization" },
    { regex: /WHERE \d+\s*=\s*1/, issue: "Always-true condition", suggestion: "Remove unnecessary WHERE conditions" }
  ];

  for (const query of queries) {
    let isSlowQuery = false;
    const querySuggestions: string[] = [];

    for (const pattern of patterns) {
      if (pattern.regex.test(query)) {
        isSlowQuery = true;
        querySuggestions.push(pattern.suggestion);
      }
    }

    if (isSlowQuery) {
      slowQueries.push(query);
      querySuggestions.forEach(suggestion => {
        if (!suggestions.includes(suggestion)) {
          suggestions.push(suggestion);
        }
      });
    }
  }

  // Ensure we have some suggestions even if all queries are optimized
  if (suggestions.length === 0 && slowQueries.length === 0) {
    suggestions.push("All queries appear to be optimized");
    suggestions.push("Consider adding composite indexes for frequently used WHERE conditions");
  }

  return {
    slowQueries,
    suggestions
  };
}

// ===== Interfaces =====
export interface SystemRequest {
  id: string;
  data: any;
  priority: number;
  timestamp: number;
}

export interface SystemResponse {
  requestId: string;
  result: any;
  server: string;
  cachedFromCache: boolean;
  processingTime: number;
}

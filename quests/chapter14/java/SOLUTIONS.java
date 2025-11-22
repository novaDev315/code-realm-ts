// Reference solutions for Chapter 14: Gate of Trials - Load Balancing

import java.util.*;
import java.util.function.Supplier;

/**
 * Solution: LoadBalancer with Round-Robin algorithm
 * Distributes requests evenly across all available servers.
 */
class LoadBalancerSolution {
    private List<String> servers;
    private int currentIndex;

    public LoadBalancerSolution(List<String> servers) {
        this.servers = new ArrayList<>(servers);
        this.currentIndex = 0;
    }

    /**
     * Get the next server in round-robin fashion.
     * Time complexity: O(1)
     */
    public String getNextServer() {
        // Handle empty server list
        if (servers.isEmpty()) {
            return null;
        }

        // Get current server and advance index
        String server = servers.get(currentIndex);
        currentIndex = (currentIndex + 1) % servers.size();
        return server;
    }

    /**
     * Add a new server to the pool.
     */
    public void addServer(String server) {
        servers.add(server);
    }

    /**
     * Remove a server from the pool.
     * Adjusts currentIndex if necessary to prevent out-of-bounds.
     */
    public void removeServer(String server) {
        int index = servers.indexOf(server);
        if (index != -1) {
            servers.remove(index);
            // Adjust currentIndex if we removed a server before or at current position
            if (!servers.isEmpty() && currentIndex >= servers.size()) {
                currentIndex = 0;
            }
        }
    }

    public int getServerCount() {
        return servers.size();
    }

    public List<String> getServers() {
        return new ArrayList<>(servers);
    }
}

/**
 * Solution: CircuitBreaker pattern implementation
 * Prevents cascading failures by failing fast when a service is unhealthy.
 *
 * State transitions:
 * - CLOSED -> OPEN: When failure count reaches threshold
 * - OPEN -> HALF_OPEN: After timeout period expires
 * - HALF_OPEN -> CLOSED: On successful call
 * - HALF_OPEN -> OPEN: On failed call
 */
class CircuitBreakerSolution {
    public enum State {
        CLOSED,    // Normal operation, requests allowed
        OPEN,      // Circuit tripped, requests blocked
        HALF_OPEN  // Testing if service recovered
    }

    private State state;
    private int failureCount;
    private int threshold;
    private long timeout;
    private long lastFailureTime;

    public CircuitBreakerSolution(int threshold, long timeout) {
        this.threshold = threshold;
        this.timeout = timeout;
        this.state = State.CLOSED;
        this.failureCount = 0;
        this.lastFailureTime = 0;
    }

    public CircuitBreakerSolution(int threshold) {
        this(threshold, 30000); // Default 30 second timeout
    }

    /**
     * Execute an operation with circuit breaker protection.
     * Supports any operation that returns a value.
     */
    public <T> T call(Supplier<T> operation) throws Exception {
        // Check if we should transition from OPEN to HALF_OPEN
        if (state == State.OPEN) {
            if (System.currentTimeMillis() - lastFailureTime >= timeout) {
                state = State.HALF_OPEN;
            } else {
                throw new CircuitBreakerOpenException("Circuit breaker is OPEN");
            }
        }

        try {
            T result = operation.get();
            onSuccess();
            return result;
        } catch (Exception e) {
            onFailure();
            throw e;
        }
    }

    /**
     * Simple call method for boolean operations.
     * Returns false indicates failure.
     */
    public boolean call(BooleanSupplierSolution operation) {
        // Check if we should transition from OPEN to HALF_OPEN
        if (state == State.OPEN) {
            if (System.currentTimeMillis() - lastFailureTime >= timeout) {
                state = State.HALF_OPEN;
            } else {
                return false; // Reject call when open
            }
        }

        try {
            boolean result = operation.getAsBoolean();

            if (result) {
                onSuccess();
            } else {
                onFailure();
            }

            return result;
        } catch (Exception e) {
            onFailure();
            return false;
        }
    }

    private void onSuccess() {
        // Success resets the circuit
        failureCount = 0;
        state = State.CLOSED;
    }

    private void onFailure() {
        failureCount++;
        lastFailureTime = System.currentTimeMillis();

        if (state == State.HALF_OPEN) {
            // Any failure in HALF_OPEN trips back to OPEN
            state = State.OPEN;
        } else if (failureCount >= threshold) {
            // Threshold exceeded, open the circuit
            state = State.OPEN;
        }
    }

    public State getState() {
        return state;
    }

    public int getFailureCount() {
        return failureCount;
    }

    public void reset() {
        this.state = State.CLOSED;
        this.failureCount = 0;
    }
}

/**
 * Custom exception for circuit breaker open state.
 */
class CircuitBreakerOpenException extends RuntimeException {
    public CircuitBreakerOpenException(String message) {
        super(message);
    }
}

/**
 * Functional interface for boolean suppliers.
 */
@FunctionalInterface
interface BooleanSupplierSolution {
    boolean getAsBoolean();
}

/**
 * Solution: Rate Limiter with Sliding Window algorithm
 */
class RateLimiterSolution {
    /**
     * Check if rate limit is exceeded using sliding window algorithm.
     *
     * @param requests Array of request timestamps
     * @param windowSize Size of the sliding window in milliseconds
     * @param limit Maximum number of requests allowed in the window
     * @return true if limit is exceeded, false otherwise
     */
    public static boolean isRateLimitExceeded(long[] requests, long windowSize, int limit) {
        if (requests == null || requests.length == 0) {
            return false;
        }

        // Use the latest request as the reference point
        long now = requests[requests.length - 1];
        long windowStart = now - windowSize;

        // Count requests within the sliding window
        int requestsInWindow = 0;
        for (long timestamp : requests) {
            if (timestamp > windowStart) {
                requestsInWindow++;
            }
        }

        // Return true if limit is exceeded
        return requestsInWindow > limit;
    }

    /**
     * Token Bucket rate limiter implementation.
     * Alternative approach for rate limiting.
     */
    public static class TokenBucket {
        private int capacity;
        private int tokens;
        private long lastRefillTime;
        private int refillRate; // tokens per second

        public TokenBucket(int capacity, int refillRate) {
            this.capacity = capacity;
            this.tokens = capacity;
            this.refillRate = refillRate;
            this.lastRefillTime = System.currentTimeMillis();
        }

        public synchronized boolean tryConsume() {
            refill();
            if (tokens > 0) {
                tokens--;
                return true;
            }
            return false;
        }

        private void refill() {
            long now = System.currentTimeMillis();
            long elapsed = now - lastRefillTime;
            int tokensToAdd = (int) (elapsed / 1000 * refillRate);

            if (tokensToAdd > 0) {
                tokens = Math.min(capacity, tokens + tokensToAdd);
                lastRefillTime = now;
            }
        }
    }
}

/**
 * Weighted Round-Robin Load Balancer
 * Distributes requests based on server weights.
 */
class WeightedLoadBalancerSolution {
    private static class ServerEntry {
        String server;
        int weight;
        int currentWeight;

        ServerEntry(String server, int weight) {
            this.server = server;
            this.weight = weight;
            this.currentWeight = 0;
        }
    }

    private List<ServerEntry> servers;

    public WeightedLoadBalancerSolution() {
        this.servers = new ArrayList<>();
    }

    public void addServer(String server, int weight) {
        servers.add(new ServerEntry(server, weight));
    }

    /**
     * Smooth Weighted Round-Robin algorithm.
     * Distributes requests proportionally to weights while maintaining smoothness.
     */
    public String getNextServer() {
        if (servers.isEmpty()) {
            return null;
        }

        int totalWeight = 0;
        ServerEntry best = null;

        for (ServerEntry entry : servers) {
            entry.currentWeight += entry.weight;
            totalWeight += entry.weight;

            if (best == null || entry.currentWeight > best.currentWeight) {
                best = entry;
            }
        }

        if (best != null) {
            best.currentWeight -= totalWeight;
            return best.server;
        }

        return null;
    }
}

// Main demonstration class
public class SOLUTIONS {
    public static void main(String[] args) {
        System.out.println("=== Chapter 14: Load Balancing Solutions ===\n");

        // Test Round-Robin Load Balancer
        System.out.println("1. Round-Robin Load Balancer");
        System.out.println("-".repeat(40));

        LoadBalancerSolution balancer = new LoadBalancerSolution(
            Arrays.asList("server1", "server2", "server3")
        );

        System.out.println("Servers: " + balancer.getServers());
        for (int i = 0; i < 6; i++) {
            System.out.println("Request " + (i + 1) + " -> " + balancer.getNextServer());
        }

        // Test dynamic add/remove
        System.out.println("\nAdding server4...");
        balancer.addServer("server4");
        System.out.println("Next: " + balancer.getNextServer());
        System.out.println("Next: " + balancer.getNextServer());

        System.out.println("\nRemoving server2...");
        balancer.removeServer("server2");
        System.out.println("Servers: " + balancer.getServers());

        // Test Circuit Breaker
        System.out.println("\n2. Circuit Breaker");
        System.out.println("-".repeat(40));

        CircuitBreakerSolution breaker = new CircuitBreakerSolution(3, 5000);
        System.out.println("Initial state: " + breaker.getState());

        // Simulate some failures
        System.out.println("\nSimulating failures...");
        for (int i = 0; i < 4; i++) {
            boolean result = breaker.call(() -> false);
            System.out.println("Call " + (i + 1) + ": result=" + result +
                ", state=" + breaker.getState() +
                ", failures=" + breaker.getFailureCount());
        }

        // Reset and test success
        System.out.println("\nResetting circuit...");
        breaker.reset();
        System.out.println("State after reset: " + breaker.getState());

        boolean successResult = breaker.call(() -> true);
        System.out.println("Successful call: result=" + successResult +
            ", state=" + breaker.getState());

        // Test Rate Limiter
        System.out.println("\n3. Sliding Window Rate Limiter");
        System.out.println("-".repeat(40));

        long now = System.currentTimeMillis();
        long[] requests1 = {now, now - 100, now - 200};
        System.out.println("3 requests in 1000ms window, limit 2: " +
            (RateLimiterSolution.isRateLimitExceeded(requests1, 1000, 2) ? "EXCEEDED" : "OK"));

        long[] requests2 = {now, now - 100};
        System.out.println("2 requests in 1000ms window, limit 3: " +
            (RateLimiterSolution.isRateLimitExceeded(requests2, 1000, 3) ? "EXCEEDED" : "OK"));

        // Test Weighted Load Balancer
        System.out.println("\n4. Weighted Round-Robin Load Balancer");
        System.out.println("-".repeat(40));

        WeightedLoadBalancerSolution weightedBalancer = new WeightedLoadBalancerSolution();
        weightedBalancer.addServer("heavy-server", 5);
        weightedBalancer.addServer("medium-server", 3);
        weightedBalancer.addServer("light-server", 1);

        Map<String, Integer> distribution = new HashMap<>();
        for (int i = 0; i < 18; i++) {
            String server = weightedBalancer.getNextServer();
            distribution.merge(server, 1, Integer::sum);
        }

        System.out.println("Distribution over 18 requests (weights 5:3:1):");
        distribution.forEach((server, count) ->
            System.out.println("  " + server + ": " + count + " requests"));

        System.out.println("\nAll solutions demonstrated successfully!");
    }
}

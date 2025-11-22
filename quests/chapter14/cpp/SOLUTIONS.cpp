// Reference solutions for Chapter 14: Gate of Trials - Load Balancing

#include <iostream>
#include <string>
#include <vector>
#include <functional>
#include <stdexcept>
#include <chrono>
#include <algorithm>
#include <map>

using namespace std;

/**
 * Solution: LoadBalancer with Round-Robin algorithm
 * Distributes requests evenly across all available servers.
 */
class LoadBalancerSolution {
private:
    vector<string> servers;
    int currentIndex;

public:
    LoadBalancerSolution(const vector<string>& serverList)
        : servers(serverList), currentIndex(0) {}

    /**
     * Get the next server in round-robin fashion.
     * Time complexity: O(1)
     */
    string getNextServer() {
        // Handle empty server list
        if (servers.empty()) {
            return "";
        }

        // Get current server and advance index
        string server = servers[currentIndex];
        currentIndex = (currentIndex + 1) % servers.size();
        return server;
    }

    /**
     * Add a new server to the pool.
     */
    void addServer(const string& server) {
        servers.push_back(server);
    }

    /**
     * Remove a server from the pool.
     * Adjusts currentIndex if necessary to prevent out-of-bounds.
     */
    void removeServer(const string& server) {
        auto it = find(servers.begin(), servers.end(), server);
        if (it != servers.end()) {
            int index = distance(servers.begin(), it);
            servers.erase(it);

            // Adjust currentIndex if we removed a server before or at current position
            if (!servers.empty() && currentIndex >= static_cast<int>(servers.size())) {
                currentIndex = 0;
            }
        }
    }

    int getServerCount() const {
        return servers.size();
    }

    vector<string> getServers() const {
        return servers;
    }
};

/**
 * Circuit Breaker State enum
 */
enum class CircuitStateSolution {
    CLOSED,    // Normal operation, requests allowed
    OPEN,      // Circuit tripped, requests blocked
    HALF_OPEN  // Testing if service recovered
};

/**
 * Custom exception for circuit breaker open state.
 */
class CircuitBreakerOpenExceptionSolution : public runtime_error {
public:
    CircuitBreakerOpenExceptionSolution(const string& message) : runtime_error(message) {}
};

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
private:
    CircuitStateSolution state;
    int failureCount;
    int threshold;
    long timeout;
    long lastFailureTime;

    long getCurrentTimeMs() const {
        return chrono::duration_cast<chrono::milliseconds>(
            chrono::system_clock::now().time_since_epoch()
        ).count();
    }

    void onSuccess() {
        // Success resets the circuit
        failureCount = 0;
        state = CircuitStateSolution::CLOSED;
    }

    void onFailure() {
        failureCount++;
        lastFailureTime = getCurrentTimeMs();

        if (state == CircuitStateSolution::HALF_OPEN) {
            // Any failure in HALF_OPEN trips back to OPEN
            state = CircuitStateSolution::OPEN;
        } else if (failureCount >= threshold) {
            // Threshold exceeded, open the circuit
            state = CircuitStateSolution::OPEN;
        }
    }

public:
    CircuitBreakerSolution(int threshold, long timeout = 30000)
        : threshold(threshold), timeout(timeout), state(CircuitStateSolution::CLOSED),
          failureCount(0), lastFailureTime(0) {}

    /**
     * Execute an operation with circuit breaker protection.
     * Supports any operation that returns a value.
     */
    template<typename T>
    T call(function<T()> operation) {
        // Check if we should transition from OPEN to HALF_OPEN
        if (state == CircuitStateSolution::OPEN) {
            if (getCurrentTimeMs() - lastFailureTime >= timeout) {
                state = CircuitStateSolution::HALF_OPEN;
            } else {
                throw CircuitBreakerOpenExceptionSolution("Circuit breaker is OPEN");
            }
        }

        try {
            T result = operation();
            onSuccess();
            return result;
        } catch (...) {
            onFailure();
            throw;
        }
    }

    /**
     * Simple call method for boolean operations.
     * Returns false indicates failure.
     */
    bool call(function<bool()> operation) {
        // Check if we should transition from OPEN to HALF_OPEN
        if (state == CircuitStateSolution::OPEN) {
            if (getCurrentTimeMs() - lastFailureTime >= timeout) {
                state = CircuitStateSolution::HALF_OPEN;
            } else {
                return false; // Reject call when open
            }
        }

        try {
            bool result = operation();

            if (result) {
                onSuccess();
            } else {
                onFailure();
            }

            return result;
        } catch (...) {
            onFailure();
            return false;
        }
    }

    CircuitStateSolution getState() const {
        return state;
    }

    string getStateString() const {
        switch (state) {
            case CircuitStateSolution::CLOSED: return "CLOSED";
            case CircuitStateSolution::OPEN: return "OPEN";
            case CircuitStateSolution::HALF_OPEN: return "HALF_OPEN";
            default: return "UNKNOWN";
        }
    }

    int getFailureCount() const {
        return failureCount;
    }

    void reset() {
        state = CircuitStateSolution::CLOSED;
        failureCount = 0;
    }
};

/**
 * Solution: Rate Limiter with Sliding Window algorithm
 */
class RateLimiterSolution {
public:
    /**
     * Check if rate limit is exceeded using sliding window algorithm.
     *
     * @param requests Vector of request timestamps
     * @param windowSize Size of the sliding window in milliseconds
     * @param limit Maximum number of requests allowed in the window
     * @return true if limit is exceeded, false otherwise
     */
    static bool isRateLimitExceeded(const vector<long>& requests, long windowSize, int limit) {
        if (requests.empty()) {
            return false;
        }

        // Use the latest request as the reference point
        long now = requests.back();
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
    class TokenBucket {
    private:
        int capacity;
        int tokens;
        long lastRefillTime;
        int refillRate; // tokens per second

        long getCurrentTimeMs() const {
            return chrono::duration_cast<chrono::milliseconds>(
                chrono::system_clock::now().time_since_epoch()
            ).count();
        }

        void refill() {
            long now = getCurrentTimeMs();
            long elapsed = now - lastRefillTime;
            int tokensToAdd = static_cast<int>(elapsed / 1000 * refillRate);

            if (tokensToAdd > 0) {
                tokens = min(capacity, tokens + tokensToAdd);
                lastRefillTime = now;
            }
        }

    public:
        TokenBucket(int capacity, int refillRate)
            : capacity(capacity), tokens(capacity), refillRate(refillRate) {
            lastRefillTime = getCurrentTimeMs();
        }

        bool tryConsume() {
            refill();
            if (tokens > 0) {
                tokens--;
                return true;
            }
            return false;
        }
    };
};

/**
 * Weighted Round-Robin Load Balancer
 * Distributes requests based on server weights.
 */
class WeightedLoadBalancerSolution {
private:
    struct ServerEntry {
        string server;
        int weight;
        int currentWeight;

        ServerEntry(const string& s, int w) : server(s), weight(w), currentWeight(0) {}
    };

    vector<ServerEntry> servers;

public:
    WeightedLoadBalancerSolution() {}

    void addServer(const string& server, int weight) {
        servers.emplace_back(server, weight);
    }

    /**
     * Smooth Weighted Round-Robin algorithm.
     * Distributes requests proportionally to weights while maintaining smoothness.
     */
    string getNextServer() {
        if (servers.empty()) {
            return "";
        }

        int totalWeight = 0;
        ServerEntry* best = nullptr;

        for (auto& entry : servers) {
            entry.currentWeight += entry.weight;
            totalWeight += entry.weight;

            if (best == nullptr || entry.currentWeight > best->currentWeight) {
                best = &entry;
            }
        }

        if (best != nullptr) {
            best->currentWeight -= totalWeight;
            return best->server;
        }

        return "";
    }
};

// Demonstration
int main() {
    cout << "=== Chapter 14: Load Balancing Solutions ===" << endl << endl;

    // Test Round-Robin Load Balancer
    cout << "1. Round-Robin Load Balancer" << endl;
    cout << string(40, '-') << endl;

    LoadBalancerSolution balancer({"server1", "server2", "server3"});

    cout << "Servers: ";
    for (const auto& s : balancer.getServers()) {
        cout << s << " ";
    }
    cout << endl;

    for (int i = 0; i < 6; i++) {
        cout << "Request " << (i + 1) << " -> " << balancer.getNextServer() << endl;
    }

    // Test dynamic add/remove
    cout << "\nAdding server4..." << endl;
    balancer.addServer("server4");
    cout << "Next: " << balancer.getNextServer() << endl;
    cout << "Next: " << balancer.getNextServer() << endl;

    cout << "\nRemoving server2..." << endl;
    balancer.removeServer("server2");
    cout << "Servers: ";
    for (const auto& s : balancer.getServers()) {
        cout << s << " ";
    }
    cout << endl;

    // Test Circuit Breaker
    cout << "\n2. Circuit Breaker" << endl;
    cout << string(40, '-') << endl;

    CircuitBreakerSolution breaker(3, 5000);
    cout << "Initial state: " << breaker.getStateString() << endl;

    // Simulate some failures
    cout << "\nSimulating failures..." << endl;
    for (int i = 0; i < 4; i++) {
        bool result = breaker.call([]() { return false; });
        cout << "Call " << (i + 1) << ": result=" << (result ? "true" : "false")
             << ", state=" << breaker.getStateString()
             << ", failures=" << breaker.getFailureCount() << endl;
    }

    // Reset and test success
    cout << "\nResetting circuit..." << endl;
    breaker.reset();
    cout << "State after reset: " << breaker.getStateString() << endl;

    bool successResult = breaker.call([]() { return true; });
    cout << "Successful call: result=" << (successResult ? "true" : "false")
         << ", state=" << breaker.getStateString() << endl;

    // Test Rate Limiter
    cout << "\n3. Sliding Window Rate Limiter" << endl;
    cout << string(40, '-') << endl;

    auto now = chrono::duration_cast<chrono::milliseconds>(
        chrono::system_clock::now().time_since_epoch()
    ).count();

    vector<long> requests1 = {now, now - 100, now - 200};
    cout << "3 requests in 1000ms window, limit 2: "
         << (RateLimiterSolution::isRateLimitExceeded(requests1, 1000, 2) ? "EXCEEDED" : "OK") << endl;

    vector<long> requests2 = {now, now - 100};
    cout << "2 requests in 1000ms window, limit 3: "
         << (RateLimiterSolution::isRateLimitExceeded(requests2, 1000, 3) ? "EXCEEDED" : "OK") << endl;

    // Test Weighted Load Balancer
    cout << "\n4. Weighted Round-Robin Load Balancer" << endl;
    cout << string(40, '-') << endl;

    WeightedLoadBalancerSolution weightedBalancer;
    weightedBalancer.addServer("heavy-server", 5);
    weightedBalancer.addServer("medium-server", 3);
    weightedBalancer.addServer("light-server", 1);

    map<string, int> distribution;
    for (int i = 0; i < 18; i++) {
        string server = weightedBalancer.getNextServer();
        distribution[server]++;
    }

    cout << "Distribution over 18 requests (weights 5:3:1):" << endl;
    for (const auto& [server, count] : distribution) {
        cout << "  " << server << ": " << count << " requests" << endl;
    }

    cout << "\nAll solutions demonstrated successfully!" << endl;

    return 0;
}

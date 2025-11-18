# Reference solutions for Chapter 15: Core of the Architect - FINAL BOSS
# Comprehensive distributed system combining LRU Cache, Message Queue, and Load Balancing

import re
from collections import OrderedDict
from datetime import datetime


# ===== LRU Cache Implementation =====
class LRUCache:
    def __init__(self, capacity):
        self.capacity = capacity
        self.cache = OrderedDict()

    def get(self, key):
        if key not in self.cache:
            return None

        # Move to end (most recently used)
        self.cache.move_to_end(key)
        return self.cache[key]

    def put(self, key, value):
        if key in self.cache:
            self.cache.move_to_end(key)
        else:
            if len(self.cache) >= self.capacity:
                # Remove least recently used (first item)
                self.cache.popitem(last=False)

        self.cache[key] = value

    def size(self):
        return len(self.cache)

    def has(self, key):
        return key in self.cache


# ===== Message Queue Implementation =====
class MessageQueue:
    def __init__(self):
        self.queue = []

    def enqueue(self, message):
        self.queue.append(message)

    def dequeue(self):
        if len(self.queue) == 0:
            return None
        return self.queue.pop(0)

    def peek(self):
        if len(self.queue) == 0:
            return None
        return self.queue[0]

    def size(self):
        return len(self.queue)

    def is_empty(self):
        return len(self.queue) == 0


# ===== Load Balancer Implementation =====
class RoundRobinBalancer:
    def __init__(self, servers):
        self.servers = servers
        self.current_index = 0

    def get_next_server(self):
        if len(self.servers) == 0:
            return None

        server = self.servers[self.current_index]
        self.current_index = (self.current_index + 1) % len(self.servers)
        return server


# ===== Distributed System =====
class DistributedSystem:
    def __init__(self):
        self.cache = LRUCache(100)
        self.queue = MessageQueue()
        self.servers = ["server-1", "server-2", "server-3", "server-4"]
        self.balancer = RoundRobinBalancer(self.servers)
        self.cache_hits = 0
        self.total_requests = 0

    def process_request(self, request):
        self.total_requests += 1

        # Check cache first
        if self.cache.has(request['id']):
            cached_result = self.cache.get(request['id'])
            self.cache_hits += 1
            return {
                'requestId': request['id'],
                'result': cached_result,
                'server': 'cache',
                'cachedFromCache': True,
                'processingTime': 1
            }

        # Add to queue
        self.queue.enqueue({
            **request,
            'timestamp': datetime.now().isoformat()
        })

        # Get next server for load balancing
        server = self.balancer.get_next_server()

        # Simulate processing and cache result
        result = {
            'data': request['data'],
            'processedAt': datetime.now().isoformat(),
            'server': server
        }

        self.cache.put(request['id'], result)

        return {
            'requestId': request['id'],
            'result': result,
            'server': server,
            'cachedFromCache': False,
            'processingTime': round(__import__('random').random() * 100, 2)
        }

    def get_metrics(self):
        return {
            'cacheHits': self.cache_hits,
            'cacheSize': self.cache.size(),
            'queueSize': self.queue.size(),
            'totalRequests': self.total_requests
        }

    def process_queue(self):
        processed = 0
        while processed < 10 and not self.queue.is_empty():
            message = self.queue.dequeue()
            if message:
                # Simulate processing
                result = {
                    'data': message['data'],
                    'processedAt': datetime.now().isoformat()
                }
                self.cache.put(message['id'], result)
                processed += 1


# ===== System Design Function =====
def design_scalable_api(requirements):
    components = []
    estimated_servers = 1

    # Add components based on requirements
    components.append("web-server")
    components.append("database")

    # High RPS requires load balancing
    if requirements.get('expectedRPS', 0) > 1000:
        components.append("load-balancer")
        estimated_servers = (requirements['expectedRPS'] + 499) // 500  # Ceiling division
    elif requirements.get('expectedRPS', 0) > 500:
        components.append("load-balancer")
        estimated_servers = 2
    elif requirements.get('expectedRPS', 0) > 100:
        estimated_servers = 1

    # Large data requires caching
    if requirements.get('dataSize') == "large" or requirements.get('caching'):
        components.append("cache")
        components.append("message-queue")

    # Authentication requires auth middleware
    if requirements.get('auth'):
        components.append("auth-service")
        components.append("token-validator")

    # Build architecture description
    architecture = ""

    if "load-balancer" in components:
        architecture += "Distributed n-tier architecture with "
        if "cache" in components:
            architecture += "multi-layer caching, "
        if "message-queue" in components:
            architecture += "async message queue processing, "
        architecture += f"and {estimated_servers} servers running in parallel with round-robin load balancing."

        if "auth-service" in components:
            architecture += " Includes authentication middleware for security."
    else:
        architecture = "Single-server architecture with "
        if "cache" in components:
            architecture += "local cache for performance optimization. "
        if "auth-service" in components:
            architecture += "Auth service for security."
        else:
            architecture += "basic request handling."

    return {
        "components": components,
        "estimated_servers": estimated_servers,
        "architecture": architecture
    }


# ===== Database Optimization Function =====
def optimize_database(queries):
    slow_queries = []
    suggestions = []

    # Define patterns for slow queries
    patterns = [
        (re.compile(r'SELECT \*'), "SELECT *", "Specify only needed columns for better performance"),
        (re.compile(r'SELECT .* FROM .* WHERE [^=]*<>'), "Inequality comparison", "Add indexes on comparison columns"),
        (re.compile(r'SELECT .* FROM .* JOIN .* ON'), "JOIN without proper index", "Ensure JOIN columns are indexed"),
        (re.compile(r'WHERE NOT IN'), "NOT IN clause", "Consider using NOT EXISTS with proper indexes"),
        (re.compile(r'YEAR\('), "Function on column", "Avoid functions on indexed columns for better query optimization"),
        (re.compile(r'WHERE \d+\s*=\s*1'), "Always-true condition", "Remove unnecessary WHERE conditions"),
    ]

    for query in queries:
        is_slow_query = False
        query_suggestions = []

        for pattern_regex, issue, suggestion in patterns:
            if pattern_regex.search(query):
                is_slow_query = True
                query_suggestions.append(suggestion)

        if is_slow_query:
            slow_queries.append(query)
            for suggestion in query_suggestions:
                if suggestion not in suggestions:
                    suggestions.append(suggestion)

    # Ensure we have some suggestions even if all queries are optimized
    if len(suggestions) == 0 and len(slow_queries) == 0:
        suggestions.append("All queries appear to be optimized")
        suggestions.append("Consider adding composite indexes for frequently used WHERE conditions")

    return {
        "slowQueries": slow_queries,
        "suggestions": suggestions
    }

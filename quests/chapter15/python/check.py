import sys
from architect import DistributedSystem, design_scalable_api, optimize_database


def run_check():
    passed = True

    # Test 1: DistributedSystem initialization
    print("Test 1: DistributedSystem initialization...")
    try:
        system = DistributedSystem()

        initial_metrics = system.get_metrics()
        if not all(isinstance(initial_metrics.get(k), (int, float)) for k in ['cacheHits', 'cacheSize', 'queueSize', 'totalRequests']):
            print("ERROR: getMetrics() should return dict with numeric properties")
            passed = False

        if initial_metrics.get('cacheHits') != 0 or initial_metrics.get('totalRequests') != 0:
            print("ERROR: Initial metrics should have 0 hits and 0 requests")
            passed = False
    except Exception as error:
        print(f"ERROR: DistributedSystem initialization failed: {error}")
        passed = False

    # Test 2: Process single request
    print("Test 2: Process single request...")
    try:
        system = DistributedSystem()

        request = {
            "id": "req-1",
            "data": {"value": "test"},
            "priority": 1,
            "timestamp": 0
        }

        response = system.process_request(request)

        if response is None:
            print("ERROR: processRequest() should return a response, got None")
            passed = False
        elif response.get('requestId') != request['id']:
            print(f"ERROR: Response should have requestId matching request.id: {request['id']}")
            passed = False
    except Exception as error:
        print(f"ERROR: Process single request failed: {error}")
        passed = False

    # Test 3: Cache functionality
    print("Test 3: Cache functionality...")
    try:
        system = DistributedSystem()

        request1 = {
            "id": "req-cached",
            "data": {"value": "cached-data"},
            "priority": 1,
            "timestamp": 0
        }

        response1 = system.process_request(request1)
        metrics1 = system.get_metrics()

        # Process same request again - should hit cache
        response2 = system.process_request(request1)
        metrics2 = system.get_metrics()

        if response1.get('cachedFromCache') == True:
            print("ERROR: First request shouldn't be from cache")
            passed = False

        if response2.get('cachedFromCache') != True:
            print("ERROR: Second request with same ID should be from cache")
            passed = False

        if metrics2.get('cacheHits', 0) < 1:
            print("ERROR: Cache hits should increase on cache hit")
            passed = False
    except Exception as error:
        print(f"ERROR: Cache functionality test failed: {error}")
        passed = False

    # Test 4: Multiple requests with load balancing
    print("Test 4: Load balancing across multiple requests...")
    try:
        system = DistributedSystem()

        requests = []
        for i in range(5):
            requests.append({
                "id": f"req-lb-{i}",
                "data": {"index": i},
                "priority": i % 3,
                "timestamp": 0
            })

        servers = set()
        for req in requests:
            response = system.process_request(req)
            if response and response.get('server'):
                servers.add(response['server'])

        if len(servers) == 0:
            print("ERROR: Load balancer should assign servers to requests")
            passed = False
    except Exception as error:
        print(f"ERROR: Load balancing test failed: {error}")
        passed = False

    # Test 5: Queue processing
    print("Test 5: Queue processing...")
    try:
        system = DistributedSystem()

        # Add multiple requests to queue
        for i in range(15):
            system.process_request({
                "id": f"queue-req-{i}",
                "data": {"index": i},
                "priority": 1,
                "timestamp": 0
            })

        metrics_before = system.get_metrics()
        system.process_queue()
        metrics_after = system.get_metrics()

        if metrics_after.get('totalRequests', 0) <= metrics_before.get('totalRequests', 0):
            print("ERROR: processQueue() should increase totalRequests")
            passed = False
    except Exception as error:
        print(f"ERROR: Queue processing test failed: {error}")
        passed = False

    # Test 6: Design scalable API
    print("Test 6: Design scalable API...")
    try:
        requirements1 = {
            "expectedRPS": 5000,
            "dataSize": "large",
            "caching": True,
            "auth": True
        }

        design1 = design_scalable_api(requirements1)

        if not isinstance(design1.get('components'), list):
            print("ERROR: designScalableAPI should return components array")
            passed = False

        if design1.get('estimated_servers', 0) < 5:
            print(f"ERROR: High RPS (5000) should require many servers (got {design1.get('estimated_servers', 0)})")
            passed = False

        if not design1.get('architecture'):
            print("ERROR: designScalableAPI should return architecture description")
            passed = False

        # Should include load balancer for high RPS
        if "load-balancer" not in design1.get('components', []):
            print("ERROR: Design with 5000 RPS should include load-balancer")
            passed = False

        # Should include cache for large data
        if "cache" not in design1.get('components', []):
            print("ERROR: Design with large data should include cache")
            passed = False
    except Exception as error:
        print(f"ERROR: Design scalable API test failed: {error}")
        passed = False

    # Test 7: Low-scale API design
    print("Test 7: Low-scale API design...")
    try:
        requirements2 = {
            "expectedRPS": 100,
            "dataSize": "small",
            "caching": False,
            "auth": False
        }

        design2 = design_scalable_api(requirements2)

        if design2.get('estimated_servers', 0) > 2:
            print("ERROR: Low RPS (100) should require fewer servers")
            passed = False

        if len(design2.get('components', [])) == 0:
            print("ERROR: Design should still include some components")
            passed = False
    except Exception as error:
        print(f"ERROR: Low-scale API design test failed: {error}")
        passed = False

    # Test 8: Optimize database queries
    print("Test 8: Optimize database queries...")
    try:
        queries = [
            "SELECT * FROM users",
            "SELECT * FROM orders WHERE status = 'pending'",
            "SELECT id, name FROM users WHERE id = 123",
            "SELECT * FROM products JOIN inventory ON products.id = inventory.product_id",
            "SELECT COUNT(*) FROM logs WHERE YEAR(date) = 2024"
        ]

        optimization = optimize_database(queries)

        if not isinstance(optimization.get('slowQueries'), list):
            print("ERROR: optimizeDatabase should return slowQueries array")
            passed = False

        if not isinstance(optimization.get('suggestions'), list):
            print("ERROR: optimizeDatabase should return suggestions array")
            passed = False

        if len(optimization.get('slowQueries', [])) == 0:
            print("ERROR: Should identify slow queries from the list")
            passed = False

        if len(optimization.get('suggestions', [])) == 0:
            print("ERROR: Should provide optimization suggestions")
            passed = False
    except Exception as error:
        print(f"ERROR: Database optimization test failed: {error}")
        passed = False

    # Test 9: Priority queue handling
    print("Test 9: Priority queue handling...")
    try:
        system = DistributedSystem()

        high_priority = {
            "id": "high-priority",
            "data": {"urgent": True},
            "priority": 10,
            "timestamp": 0
        }

        low_priority = {
            "id": "low-priority",
            "data": {"urgent": False},
            "priority": 1,
            "timestamp": 0
        }

        resp1 = system.process_request(low_priority)
        resp2 = system.process_request(high_priority)

        if resp1 is None or resp2 is None:
            print("ERROR: Both requests should be processed")
            passed = False

        metrics = system.get_metrics()
        if metrics.get('totalRequests', 0) < 2:
            print("ERROR: Should track both requests")
            passed = False
    except Exception as error:
        print(f"ERROR: Priority queue handling test failed: {error}")
        passed = False

    # Test 10: Metrics tracking
    print("Test 10: Metrics tracking...")
    try:
        system = DistributedSystem()

        for i in range(10):
            system.process_request({
                "id": f"metrics-req-{i}",
                "data": {"index": i},
                "priority": 1,
                "timestamp": 0
            })

        metrics = system.get_metrics()

        if metrics.get('totalRequests', 0) < 10:
            print("ERROR: Should track all 10 requests")
            passed = False

        if metrics.get('cacheSize', 0) < 0:
            print("ERROR: Cache size cannot be negative")
            passed = False

        if metrics.get('cacheHits', 0) < 0:
            print("ERROR: Cache hits cannot be negative")
            passed = False
    except Exception as error:
        print(f"ERROR: Metrics tracking test failed: {error}")
        passed = False

    if passed:
        print("\n✅ All Chapter 15 tests passed! You have mastered the Core of the Architect!")
    else:
        print("\n❌ Some tests failed. Review the SOLUTIONS.py for reference implementation.")
        sys.exit(1)

    return passed


if __name__ == "__main__":
    run_check()

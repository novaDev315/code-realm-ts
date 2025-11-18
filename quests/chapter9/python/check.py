"""Test suite for Chapter 9: Dungeon of DevOps"""

import sys
from devops import create_dockerfile, create_docker_compose, create_health_check


def run_check() -> bool:
    """Run all test cases and return True if all pass"""
    passed = True

    # Test create_dockerfile
    dockerfile_cases = [
        {
            "input": {"base_image": "node:18-alpine", "workdir": "/app", "port": 3000},
            "expect": {"keywords": ["FROM", "WORKDIR", "EXPOSE", "CMD"]},
            "description": "Create Node.js Dockerfile"
        },
        {
            "input": {"base_image": "python:3.11-slim", "workdir": "/workspace", "port": 5000},
            "expect": {"keywords": ["FROM", "WORKDIR", "EXPOSE", "CMD"]},
            "description": "Create Python Dockerfile"
        },
        {
            "input": {"base_image": "nginx:latest", "workdir": "/usr/share/nginx", "port": 80},
            "expect": {"keywords": ["FROM", "WORKDIR", "EXPOSE", "CMD"]},
            "description": "Create Nginx Dockerfile"
        }
    ]

    for c in dockerfile_cases:
        result = create_dockerfile(c["input"]["base_image"], c["input"]["workdir"], c["input"]["port"])
        has_all_keywords = all(keyword in result.upper() for keyword in c["expect"]["keywords"])

        if not has_all_keywords or c["input"]["base_image"] not in result:
            print(
                f"❌ create_dockerfile failed: {c['description']}\n"
                f"   Expected to contain: {', '.join(c['expect']['keywords'])} and \"{c['input']['base_image']}\"\n"
                f"   Got:\n{result or '(empty string)'}"
            )
            passed = False

    # Test create_docker_compose
    compose_cases = [
        {
            "input": {"service_name": "web", "image": "myapp:latest", "port": "3000:3000"},
            "expect": {"keywords": ["version", "services", "ports"]},
            "description": "Create web service compose file"
        },
        {
            "input": {"service_name": "api", "image": "api:1.0", "port": "8080:8080"},
            "expect": {"keywords": ["version", "services", "ports"]},
            "description": "Create API service compose file"
        },
        {
            "input": {"service_name": "worker", "image": "worker:latest", "port": "9000:9000"},
            "expect": {"keywords": ["version", "services", "ports"]},
            "description": "Create worker service compose file"
        }
    ]

    for c in compose_cases:
        result = create_docker_compose(c["input"]["service_name"], c["input"]["image"], c["input"]["port"])
        has_all_keywords = all(keyword.lower() in result.lower() for keyword in c["expect"]["keywords"])

        if not has_all_keywords or c["input"]["service_name"] not in result:
            print(
                f"❌ create_docker_compose failed: {c['description']}\n"
                f"   Expected to contain: {', '.join(c['expect']['keywords'])} and \"{c['input']['service_name']}\"\n"
                f"   Got:\n{result or '(empty string)'}"
            )
            passed = False

    # Test create_health_check
    health_check_cases = [
        {
            "input": {"endpoint": "/health", "interval": 30},
            "expect": {"properties": ["test", "interval", "timeout", "retries"]},
            "description": "Create HTTP health check"
        },
        {
            "input": {"endpoint": "/api/status", "interval": 60},
            "expect": {"properties": ["test", "interval", "timeout", "retries"]},
            "description": "Create API status health check"
        },
        {
            "input": {"endpoint": "/ping", "interval": 15},
            "expect": {"properties": ["test", "interval", "timeout", "retries"]},
            "description": "Create ping health check"
        }
    ]

    for c in health_check_cases:
        result = create_health_check(c["input"]["endpoint"], c["input"]["interval"])
        has_all_properties = all(prop in result for prop in c["expect"]["properties"])

        if not has_all_properties or not result.get("test") or result.get("interval") != c["input"]["interval"]:
            print(
                f"❌ create_health_check failed: {c['description']}\n"
                f"   Expected properties: {', '.join(c['expect']['properties'])}\n"
                f"   Expected interval: {c['input']['interval']}\n"
                f"   Got: {result}"
            )
            passed = False

    if passed:
        print("✅ All tests passed!")
    else:
        print("❌ Some tests failed.")
        sys.exit(1)

    return passed


if __name__ == "__main__":
    run_check()

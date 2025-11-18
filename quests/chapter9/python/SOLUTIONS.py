"""Reference solutions for Chapter 9: Dungeon of DevOps"""

from typing import Dict, List, Any


def create_dockerfile(base_image: str, workdir: str, port: int) -> str:
    """
    Create a Dockerfile content string

    Args:
        base_image: Base Docker image (e.g., "node:18-alpine")
        workdir: Working directory in container
        port: Port to expose

    Returns:
        Dockerfile content as multi-line string
    """
    return f"""FROM {base_image}

WORKDIR {workdir}

COPY . .

RUN if [ -f package.json ]; then npm install; fi || \\
    if [ -f requirements.txt ]; then pip install -r requirements.txt; fi || \\
    true

EXPOSE {port}

CMD ["sh", "-c", "echo 'Container running on port {port}'"]
"""


def create_docker_compose(service_name: str, image: str, port: str) -> str:
    """
    Create docker-compose.yml content string

    Args:
        service_name: Name of the service
        image: Docker image to use
        port: Port mapping (e.g., "3000:3000")

    Returns:
        docker-compose.yml content as multi-line string
    """
    return f"""version: '3.8'

services:
  {service_name}:
    image: {image}
    ports:
      - "{port}"
    environment:
      - NODE_ENV=production
    restart: unless-stopped
"""


def create_health_check(endpoint: str, interval: int) -> Dict[str, Any]:
    """
    Create a health check configuration

    Args:
        endpoint: Health check endpoint
        interval: Check interval in seconds

    Returns:
        Health check configuration dictionary
    """
    return {
        "test": ["CMD", "curl", "-f", f"http://localhost{endpoint}"],
        "interval": interval,
        "timeout": 10,
        "retries": 3
    }

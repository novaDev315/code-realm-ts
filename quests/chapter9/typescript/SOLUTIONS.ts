// Reference solutions for Chapter 9: Dungeon of DevOps
// These are working implementations for testing purposes

export function createDockerfile(baseImage: string, workdir: string, port: number): string {
  return `FROM ${baseImage}

WORKDIR ${workdir}

COPY . .

RUN if [ -f package.json ]; then npm install; fi || \
    if [ -f requirements.txt ]; then pip install -r requirements.txt; fi || \
    true

EXPOSE ${port}

CMD ["sh", "-c", "echo 'Container running on port ${port}'"]
`;
}

export function createDockerCompose(serviceName: string, image: string, port: string): string {
  return `version: '3.8'

services:
  ${serviceName}:
    image: ${image}
    ports:
      - "${port}"
    environment:
      - NODE_ENV=production
    restart: unless-stopped
`;
}

export function createHealthCheck(endpoint: string, interval: number): object {
  return {
    test: ["CMD", "curl", "-f", `http://localhost${endpoint}`],
    interval: interval,
    timeout: 10,
    retries: 3
  };
}

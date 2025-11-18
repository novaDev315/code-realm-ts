// Reference solutions for Chapter 9: Dungeon of DevOps
// These are working implementations for testing purposes

function createDockerfile(baseImage, workdir, port) {
  return `FROM ${baseImage}

WORKDIR ${workdir}

COPY . .

RUN if [ -f package.json ]; then npm install; fi || \\
    if [ -f requirements.txt ]; then pip install -r requirements.txt; fi || \\
    true

EXPOSE ${port}

CMD ["sh", "-c", "echo 'Container running on port ${port}'"]
`;
}

function createDockerCompose(serviceName, image, port) {
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

function createHealthCheck(endpoint, interval) {
  return {
    test: ["CMD", "curl", "-f", `http://localhost${endpoint}`],
    interval: interval,
    timeout: 10,
    retries: 3
  };
}

module.exports = {
  createDockerfile,
  createDockerCompose,
  createHealthCheck
};

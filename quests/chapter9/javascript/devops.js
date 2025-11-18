// Chapter 9: Dungeon of DevOps
// Player solutions will live here.
// TODO: Implement Docker and container orchestration configurations.

/**
 * Create a Dockerfile content string
 * @param {string} baseImage - Base Docker image (e.g., "node:18-alpine")
 * @param {string} workdir - Working directory in container
 * @param {number} port - Port to expose
 * @returns {string} Dockerfile content
 */
function createDockerfile(baseImage, workdir, port) {
  // TODO: Return Dockerfile content as multi-line string
  // Should include FROM, WORKDIR, EXPOSE, and CMD directives
  return "";
}

/**
 * Create docker-compose.yml content string
 * @param {string} serviceName - Name of the service
 * @param {string} image - Docker image to use
 * @param {string} port - Port mapping (e.g., "3000:3000")
 * @returns {string} docker-compose.yml content
 */
function createDockerCompose(serviceName, image, port) {
  // TODO: Return docker-compose.yml content as multi-line string
  // Should include version, services, and port mapping
  return "";
}

/**
 * Create a health check configuration
 * @param {string} endpoint - Health check endpoint
 * @param {number} interval - Check interval in seconds
 * @returns {Object} Health check configuration
 */
function createHealthCheck(endpoint, interval) {
  // TODO: Return health check configuration object
  // Should have properties: test, interval, timeout, retries
  return {};
}

module.exports = {
  createDockerfile,
  createDockerCompose,
  createHealthCheck
};

const { createDockerfile, createDockerCompose, createHealthCheck } = require("./devops");

function runCheck() {
  let passed = true;

  // Test createDockerfile
  const dockerfileCases = [
    {
      input: { baseImage: "node:18-alpine", workdir: "/app", port: 3000 },
      expect: { keywords: ["FROM", "WORKDIR", "EXPOSE", "CMD"] },
      description: "Create Node.js Dockerfile"
    },
    {
      input: { baseImage: "python:3.11-slim", workdir: "/workspace", port: 5000 },
      expect: { keywords: ["FROM", "WORKDIR", "EXPOSE", "CMD"] },
      description: "Create Python Dockerfile"
    },
    {
      input: { baseImage: "nginx:latest", workdir: "/usr/share/nginx", port: 80 },
      expect: { keywords: ["FROM", "WORKDIR", "EXPOSE", "CMD"] },
      description: "Create Nginx Dockerfile"
    }
  ];

  for (const c of dockerfileCases) {
    const result = createDockerfile(c.input.baseImage, c.input.workdir, c.input.port);
    const hasAllKeywords = c.expect.keywords.every((keyword) =>
      result.toUpperCase().includes(keyword)
    );

    if (!hasAllKeywords || !result.includes(c.input.baseImage)) {
      console.error(
        `❌ createDockerfile failed: ${c.description}\n` +
        `   Expected to contain: ${c.expect.keywords.join(", ")} and "${c.input.baseImage}"\n` +
        `   Got:\n${result || "(empty string)"}`
      );
      passed = false;
    }
  }

  // Test createDockerCompose
  const composeCases = [
    {
      input: { serviceName: "web", image: "myapp:latest", port: "3000:3000" },
      expect: { keywords: ["version", "services", "ports"] },
      description: "Create web service compose file"
    },
    {
      input: { serviceName: "api", image: "api:1.0", port: "8080:8080" },
      expect: { keywords: ["version", "services", "ports"] },
      description: "Create API service compose file"
    },
    {
      input: { serviceName: "worker", image: "worker:latest", port: "9000:9000" },
      expect: { keywords: ["version", "services", "ports"] },
      description: "Create worker service compose file"
    }
  ];

  for (const c of composeCases) {
    const result = createDockerCompose(c.input.serviceName, c.input.image, c.input.port);
    const hasAllKeywords = c.expect.keywords.every((keyword) =>
      result.toLowerCase().includes(keyword.toLowerCase())
    );

    if (!hasAllKeywords || !result.includes(c.input.serviceName)) {
      console.error(
        `❌ createDockerCompose failed: ${c.description}\n` +
        `   Expected to contain: ${c.expect.keywords.join(", ")} and "${c.input.serviceName}"\n` +
        `   Got:\n${result || "(empty string)"}`
      );
      passed = false;
    }
  }

  // Test createHealthCheck
  const healthCheckCases = [
    {
      input: { endpoint: "/health", interval: 30 },
      expect: { properties: ["test", "interval", "timeout", "retries"] },
      description: "Create HTTP health check"
    },
    {
      input: { endpoint: "/api/status", interval: 60 },
      expect: { properties: ["test", "interval", "timeout", "retries"] },
      description: "Create API status health check"
    },
    {
      input: { endpoint: "/ping", interval: 15 },
      expect: { properties: ["test", "interval", "timeout", "retries"] },
      description: "Create ping health check"
    }
  ];

  for (const c of healthCheckCases) {
    const result = createHealthCheck(c.input.endpoint, c.input.interval);
    const hasAllProperties = c.expect.properties.every((prop) => prop in result);

    if (!hasAllProperties || !result.test || result.interval !== c.input.interval) {
      console.error(
        `❌ createHealthCheck failed: ${c.description}\n` +
        `   Expected properties: ${c.expect.properties.join(", ")}\n` +
        `   Expected interval: ${c.input.interval}\n` +
        `   Got: ${JSON.stringify(result)}`
      );
      passed = false;
    }
  }

  if (passed) {
    console.log("✅ All tests passed!");
  } else {
    console.log("❌ Some tests failed.");
    process.exit(1);
  }

  return passed;
}

// Allow running as standalone script
if (require.main === module) {
  runCheck();
}

module.exports = { runCheck };

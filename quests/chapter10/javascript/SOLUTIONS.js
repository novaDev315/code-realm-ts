// Reference solutions for Chapter 10: Citadel of Firewalls
// These are working implementations for testing purposes

const crypto = require("crypto");

function hashPassword(password, salt) {
  // Simple password hashing: SHA256(password + salt)
  const combined = password + salt;
  const hash = crypto.createHash("sha256").update(combined).digest("hex");
  return hash;
}

function validateJWT(token) {
  // JWT format: header.payload.signature
  const parts = token.split(".");

  if (parts.length !== 3) {
    return { valid: false };
  }

  try {
    // Decode and validate JWT structure
    const [headerB64, payloadB64, signature] = parts;

    // Validate that parts are non-empty and valid base64
    if (!headerB64 || !payloadB64 || !signature) {
      return { valid: false };
    }

    // Decode header (should be valid JSON)
    const headerJson = Buffer.from(headerB64, "base64").toString("utf8");
    const header = JSON.parse(headerJson);

    // Decode payload (should be valid JSON)
    const payloadJson = Buffer.from(payloadB64, "base64").toString("utf8");
    const payload = JSON.parse(payloadJson);

    // Basic validation passed
    return { valid: true, payload };
  } catch (error) {
    return { valid: false };
  }
}

function rateLimit(requests, limit, windowMs) {
  // Calculate the earliest timestamp in the window
  const now = Date.now();
  const windowStart = now - windowMs;

  // Count requests from this IP within the window
  const requestCount = requests.filter((req) => req.timestamp >= windowStart).length;

  // Return true if limit is exceeded
  return requestCount > limit;
}

function sanitizeInput(input) {
  // Remove script tags and their content
  let sanitized = input.replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, "");

  // Remove other HTML tags
  sanitized = sanitized.replace(/<[^>]*>/g, "");

  // Escape dangerous characters
  sanitized = sanitized
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;")
    .replace(/&(?!(?:[a-z]+|#[0-9]+);)/gi, "&amp;");

  return sanitized;
}

module.exports = {
  hashPassword,
  validateJWT,
  rateLimit,
  sanitizeInput
};

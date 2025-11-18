// Chapter 10: Citadel of Firewalls - Security ⭐

function hashPassword(password, salt) {
  // TODO: Implement simple password hashing (can use basic crypto)
  // Should combine password + salt and return hex hash
  return "";
}

function validateJWT(token) {
  // TODO: Validate JWT structure (header.payload.signature)
  // Return validity and decoded payload if valid
  return { valid: false };
}

function rateLimit(requests, limit, windowMs) {
  // TODO: Implement rate limiting logic
  // Return true if rate limit exceeded
  return false;
}

function sanitizeInput(input) {
  // TODO: Sanitize input to prevent XSS
  // Remove/escape < > " ' and script tags
  return input;
}

module.exports = {
  hashPassword,
  validateJWT,
  rateLimit,
  sanitizeInput
};

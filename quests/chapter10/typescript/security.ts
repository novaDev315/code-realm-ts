// Chapter 10: Citadel of Firewalls - Security ⭐

export function hashPassword(password: string, salt: string): string {
  // TODO: Implement simple password hashing (can use basic crypto)
  // Should combine password + salt and return hex hash
  return "";
}

export function validateJWT(token: string): { valid: boolean; payload?: any } {
  // TODO: Validate JWT structure (header.payload.signature)
  // Return validity and decoded payload if valid
  return { valid: false };
}

export function rateLimit(requests: { ip: string; timestamp: number }[], limit: number, windowMs: number): boolean {
  // TODO: Implement rate limiting logic
  // Return true if rate limit exceeded
  return false;
}

export function sanitizeInput(input: string): string {
  // TODO: Sanitize input to prevent XSS
  // Remove/escape < > " ' and script tags
  return input;
}

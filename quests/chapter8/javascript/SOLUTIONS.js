// Reference solutions for Chapter 8
// These are working implementations for testing purposes

/**
 * Create a REST endpoint configuration
 * @param {string} method - HTTP method (GET, POST, DELETE, etc.)
 * @param {string} path - URL path for the endpoint
 * @param {Function} handler - Request handler function
 * @returns {Object} Endpoint configuration
 */
function createRestEndpoint(method, path, handler) {
  return {
    method: method.toUpperCase(),
    path: path,
    handler: handler
  };
}

/**
 * Parse a GraphQL query string
 * @param {string} query - GraphQL query string
 * @returns {Object} Parsed query with operation type and fields
 */
function parseGraphQLQuery(query) {
  // Extract operation type (query, mutation, subscription)
  const operationMatch = query.match(/^\s*(query|mutation|subscription)/i);
  const operation = operationMatch ? operationMatch[1].toLowerCase() : "query";

  // Extract top-level field names from the query
  const fields = [];
  let braceDepth = 0;
  let currentField = "";

  for (let i = 0; i < query.length; i++) {
    const char = query[i];

    if (char === "{") {
      braceDepth++;
      if (braceDepth === 2) {
        // We're entering nested content, save the field name
        const fieldName = currentField.trim();
        if (fieldName && /^[a-zA-Z_]/.test(fieldName)) {
          fields.push(fieldName);
        }
        currentField = "";
      }
    } else if (char === "}") {
      braceDepth--;
    } else if (braceDepth === 1 && (char === " " || char === "\n" || char === "\t")) {
      // Whitespace at first level - field separator
      const fieldName = currentField.trim();
      if (fieldName && /^[a-zA-Z_]/.test(fieldName)) {
        fields.push(fieldName);
      }
      currentField = "";
    } else if (braceDepth === 1) {
      currentField += char;
    }
  }

  // Add any remaining field
  const fieldName = currentField.trim();
  if (fieldName && /^[a-zA-Z_]/.test(fieldName)) {
    fields.push(fieldName);
  }

  return { operation, fields: fields.slice(0, 10) };
}

/**
 * Validate API key format
 * @param {string} apiKey - API key to validate
 * @returns {boolean} True if valid, false otherwise
 */
function validateApiKey(apiKey) {
  // Validate API key format: 32 chars alphanumeric
  if (!apiKey || typeof apiKey !== "string") {
    return false;
  }

  // Check length and character composition
  return apiKey.length === 32 && /^[a-zA-Z0-9]+$/.test(apiKey);
}

module.exports = {
  createRestEndpoint,
  parseGraphQLQuery,
  validateApiKey
};

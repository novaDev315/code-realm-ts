// Reference solutions for Chapter 8
// These are working implementations for testing purposes

export interface ApiResponse<T> {
  data?: T;
  error?: string;
  status: number;
}

export function createRestEndpoint(method: string, path: string, handler: Function): object {
  return {
    method: method.toUpperCase(),
    path: path,
    handler: handler
  };
}

export function parseGraphQLQuery(query: string): { operation: string; fields: string[] } {
  // Extract operation type (query, mutation, subscription)
  const operationMatch = query.match(/^\s*(query|mutation|subscription)/i);
  const operation = operationMatch ? operationMatch[1].toLowerCase() : "query";

  // Extract top-level field names from the query
  const fields: string[] = [];
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

export function validateApiKey(apiKey: string): boolean {
  // Validate API key format: 32 chars alphanumeric
  if (!apiKey || typeof apiKey !== "string") {
    return false;
  }

  // Check length and character composition
  return apiKey.length === 32 && /^[a-zA-Z0-9]+$/.test(apiKey);
}

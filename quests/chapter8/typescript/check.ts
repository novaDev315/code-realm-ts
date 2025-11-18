import { createRestEndpoint, parseGraphQLQuery, validateApiKey, ApiResponse } from "./api";

interface Case<I, O> {
  input: I;
  expect: O;
  description?: string;
}

export function runCheck(): boolean {
  let passed = true;

  // Test createRestEndpoint
  const endpointCases = [
    {
      input: { method: "GET", path: "/users", handler: () => {} },
      expect: { method: "GET", path: "/users" },
      description: "Create GET endpoint"
    },
    {
      input: { method: "post", path: "/api/create", handler: () => {} },
      expect: { method: "POST", path: "/api/create" },
      description: "Create POST endpoint (lowercase method)"
    },
    {
      input: { method: "DELETE", path: "/items/123", handler: () => {} },
      expect: { method: "DELETE", path: "/items/123" },
      description: "Create DELETE endpoint"
    }
  ];

  for (const c of endpointCases) {
    const result = createRestEndpoint(c.input.method, c.input.path, c.input.handler) as any;
    if (
      result.method !== c.expect.method ||
      result.path !== c.expect.path ||
      typeof result.handler !== "function"
    ) {
      console.error(
        `❌ createRestEndpoint failed: ${c.description || "unknown"}\n` +
        `   Expected: method=${c.expect.method}, path=${c.expect.path}, handler=function\n` +
        `   Got: method=${result.method}, path=${result.path}, handler=${typeof result.handler}`
      );
      passed = false;
    }
  }

  // Test parseGraphQLQuery
  const queryParseCases: Case<string, { operation: string; fields: string[] }>[] = [
    {
      input: "query { user { id name } }",
      expect: { operation: "query", fields: ["user"] },
      description: "Parse simple query"
    },
    {
      input: "mutation { createUser { id email } }",
      expect: { operation: "mutation", fields: ["createUser"] },
      description: "Parse mutation"
    },
    {
      input: "{ user posts comments }",
      expect: { operation: "query", fields: ["user", "posts", "comments"] },
      description: "Parse implicit query with multiple fields"
    },
    {
      input: "subscription { userUpdated }",
      expect: { operation: "subscription", fields: ["userUpdated"] },
      description: "Parse subscription"
    }
  ];

  for (const c of queryParseCases) {
    const result = parseGraphQLQuery(c.input);
    const fieldsMatch =
      result.fields.length === c.expect.fields.length &&
      result.fields.every((f, i) => f === c.expect.fields[i]);

    if (result.operation !== c.expect.operation || !fieldsMatch) {
      console.error(
        `❌ parseGraphQLQuery failed: ${c.description}\n` +
        `   Input: "${c.input}"\n` +
        `   Expected: operation="${c.expect.operation}", fields=[${c.expect.fields.join(", ")}]\n` +
        `   Got: operation="${result.operation}", fields=[${result.fields.join(", ")}]`
      );
      passed = false;
    }
  }

  // Test validateApiKey
  const apiKeyCases: Case<string, boolean>[] = [
    {
      input: "abcdef1234567890abcdef1234567890",
      expect: true,
      description: "Valid 32-char alphanumeric key"
    },
    {
      input: "ABCDEF1234567890ABCDEF1234567890",
      expect: true,
      description: "Valid uppercase alphanumeric key"
    },
    {
      input: "MixedCase12345678901234567890abc",
      expect: true,
      description: "Valid mixed case key"
    },
    {
      input: "toolshort",
      expect: false,
      description: "Invalid: too short"
    },
    {
      input: "abcdef1234567890abcdef1234567890extra",
      expect: false,
      description: "Invalid: too long"
    },
    {
      input: "abcdef1234567890abcdef1234567890-!",
      expect: false,
      description: "Invalid: contains special characters"
    },
    {
      input: "",
      expect: false,
      description: "Invalid: empty string"
    }
  ];

  for (const c of apiKeyCases) {
    const result = validateApiKey(c.input);
    if (result !== c.expect) {
      console.error(
        `❌ validateApiKey failed: ${c.description}\n` +
        `   Input: "${c.input}"\n` +
        `   Expected: ${c.expect}\n` +
        `   Got: ${result}`
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

const { hashPassword, validateJWT, rateLimit, sanitizeInput } = require("./security");

function runCheck() {
  let passed = true;

  // Test 1: Password Hashing
  console.log("\n🔐 Testing Password Hashing...");
  const hashCases = [
    { input: { password: "secret123", salt: "salt1" }, expect: true },
    { input: { password: "mypassword", salt: "salt2" }, expect: true }
  ];

  for (const c of hashCases) {
    const result = hashPassword(c.input.password, c.input.salt);
    // Should return a non-empty hash string
    if (typeof result !== "string" || result.length === 0) {
      console.error(
        `hashPassword("${c.input.password}", "${c.input.salt}") should return a non-empty hash string, got "${result}"`
      );
      passed = false;
    } else {
      console.log(`✓ hashPassword("${c.input.password}", "${c.input.salt}") = "${result.substring(0, 16)}..."`);
    }
  }

  // Test 2: JWT Validation
  console.log("\n🎫 Testing JWT Validation...");
  const validJWT = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
  const invalidJWT = "invalid.token.structure";
  const malformedJWT = "eyJhbGciOiJIUzI1NiJ9.invalid";

  const jwtTests = [
    { input: validJWT, shouldBeValid: true },
    { input: invalidJWT, shouldBeValid: false },
    { input: malformedJWT, shouldBeValid: false }
  ];

  for (const test of jwtTests) {
    const result = validateJWT(test.input);
    const isValid = result.valid;
    const expectedValid = test.shouldBeValid;

    if (isValid !== expectedValid) {
      console.error(
        `validateJWT("${test.input.substring(0, 20)}...") expected valid=${expectedValid}, got ${isValid}`
      );
      passed = false;
    } else {
      console.log(`✓ validateJWT("${test.input.substring(0, 20)}...") = ${isValid}`);
      if (isValid && result.payload) {
        console.log(`  Payload: ${JSON.stringify(result.payload)}`);
      }
    }
  }

  // Test 3: Rate Limiting
  console.log("\n⏱️  Testing Rate Limiting...");
  const now = Date.now();
  const requests1 = [
    { ip: "192.168.1.1", timestamp: now },
    { ip: "192.168.1.1", timestamp: now - 100 },
    { ip: "192.168.1.1", timestamp: now - 200 }
  ];

  // 3 requests within 1 second window with limit of 2 should exceed
  const result1 = rateLimit(requests1, 2, 1000);
  if (result1 !== true) {
    console.error(`rateLimit([3 requests], limit=2, window=1000ms) expected true (exceeded), got ${result1}`);
    passed = false;
  } else {
    console.log(`✓ rateLimit([3 requests], limit=2, window=1000ms) = true (exceeded)`);
  }

  // 2 requests within 1 second window with limit of 3 should not exceed
  const requests2 = [
    { ip: "192.168.1.2", timestamp: now },
    { ip: "192.168.1.2", timestamp: now - 100 }
  ];
  const result2 = rateLimit(requests2, 3, 1000);
  if (result2 !== false) {
    console.error(`rateLimit([2 requests], limit=3, window=1000ms) expected false (not exceeded), got ${result2}`);
    passed = false;
  } else {
    console.log(`✓ rateLimit([2 requests], limit=3, window=1000ms) = false (not exceeded)`);
  }

  // Test 4: Input Sanitization
  console.log("\n🛡️  Testing Input Sanitization...");
  const sanitizeCases = [
    { input: "<script>alert('XSS')</script>", expect: "alert('XSS')" },
    { input: "Hello<img src=x onerror='alert(1)'>World", expect: "HelloWorld" },
    { input: "Safe input with no tags", expect: "Safe input with no tags" },
    { input: "<div>Content</div>", expect: "Content" },
    { input: "Test & Demo \"quoted\"", expect: "Test & Demo &quot;quoted&quot;" }
  ];

  for (const c of sanitizeCases) {
    const result = sanitizeInput(c.input);
    // Should remove dangerous tags and escape quotes
    const hasDangerousContent = result.includes("<") || result.includes(">") || result.includes("script");
    if (hasDangerousContent) {
      console.error(`sanitizeInput("${c.input}") should not contain dangerous HTML tags, got "${result}"`);
      passed = false;
    } else {
      console.log(`✓ sanitizeInput("${c.input.substring(0, 30)}...") = "${result.substring(0, 30)}..."`);
    }
  }

  // Summary
  console.log("\n" + "=".repeat(50));
  if (passed) {
    console.log("✅ All security tests passed!");
  } else {
    console.log("❌ Some security tests failed.");
    process.exit(1);
  }

  return passed;
}

runCheck();

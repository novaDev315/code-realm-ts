const { encodeBase62, decodeBase62, generateShortCode, URLShortener } = require("./urlshortener");

function runCheck() {
  let passed = true;

  // Test encodeBase62
  console.log("Testing encodeBase62...");
  const encodeTests = [
    { input: 0, expect: "0", name: "Zero" },
    { input: 9, expect: "9", name: "Single digit" },
    { input: 10, expect: "a", name: "Ten (should be 'a')" },
    { input: 35, expect: "z", name: "35 (lowercase z)" },
    { input: 36, expect: "A", name: "36 (uppercase A)" },
    { input: 61, expect: "Z", name: "61 (uppercase Z)" },
    { input: 62, expect: "10", name: "62 (should be '10')" },
    { input: 3843, expect: "ZZ", name: "3843 (Z*62+Z)" }
  ];

  for (const test of encodeTests) {
    const result = encodeBase62(test.input);
    if (result !== test.expect) {
      console.error(
        `encodeBase62(${test.input}) - ${test.name}: expected "${test.expect}" but got "${result}"`
      );
      passed = false;
    }
  }

  // Test decodeBase62
  console.log("Testing decodeBase62...");
  const decodeTests = [
    { input: "0", expect: 0, name: "Zero" },
    { input: "9", expect: 9, name: "Single digit" },
    { input: "a", expect: 10, name: "Letter 'a' (lowercase)" },
    { input: "z", expect: 35, name: "Letter 'z' (lowercase)" },
    { input: "A", expect: 36, name: "Letter 'A' (uppercase)" },
    { input: "Z", expect: 61, name: "Letter 'Z' (uppercase)" },
    { input: "10", expect: 62, name: "Two chars '10'" },
    { input: "ZZ", expect: 3843, name: "Two Z's" }
  ];

  for (const test of decodeTests) {
    const result = decodeBase62(test.input);
    if (result !== test.expect) {
      console.error(
        `decodeBase62("${test.input}") - ${test.name}: expected ${test.expect} but got ${result}`
      );
      passed = false;
    }
  }

  // Test encoding/decoding round-trip
  console.log("Testing encode/decode round-trip...");
  const roundTripTests = [0, 1, 10, 35, 36, 61, 62, 100, 1000, 10000, 3843];
  for (const num of roundTripTests) {
    const encoded = encodeBase62(num);
    const decoded = decodeBase62(encoded);
    if (decoded !== num) {
      console.error(
        `Round-trip failed for ${num}: encode -> "${encoded}" -> decode -> ${decoded}`
      );
      passed = false;
    }
  }

  // Test URLShortener
  console.log("Testing URLShortener class...");
  const shortener = new URLShortener();

  // Test shortening URLs
  const url1 = "https://www.example.com/very/long/path/that/needs/shortening";
  const url2 = "https://github.com/anthropics/code-realm-ts";
  const url3 = "https://docs.anthropic.com/en/api/getting-started";

  const short1 = shortener.shorten(url1);
  const short2 = shortener.shorten(url2);
  const short3 = shortener.shorten(url3);

  if (!short1 || !short2 || !short3) {
    console.error("URLShortener.shorten() returned empty string");
    passed = false;
  }

  // Check that different URLs get different short codes
  if (short1 && short2 && short1 === short2) {
    console.error("Different URLs received the same short code");
    passed = false;
  }

  // Test expanding URLs
  if (short1) {
    const expanded1 = shortener.expand(short1);
    if (expanded1 !== url1) {
      console.error(
        `expand("${short1}"): expected "${url1}" but got "${expanded1}"`
      );
      passed = false;
    }
  }

  if (short2) {
    const expanded2 = shortener.expand(short2);
    if (expanded2 !== url2) {
      console.error(
        `expand("${short2}"): expected "${url2}" but got "${expanded2}"`
      );
      passed = false;
    }
  }

  if (short3) {
    const expanded3 = shortener.expand(short3);
    if (expanded3 !== url3) {
      console.error(
        `expand("${short3}"): expected "${url3}" but got "${expanded3}"`
      );
      passed = false;
    }
  }

  // Test expanding non-existent code
  const nonExistent = shortener.expand("nonexistent123");
  if (nonExistent !== null) {
    console.error(
      `expand("nonexistent123") should return null, got "${nonExistent}"`
    );
    passed = false;
  }

  // Test that same URL returns same short code (idempotent)
  const shortenerIdempotent = new URLShortener();
  const urlIdempotent = "https://example.com";
  const short1a = shortenerIdempotent.shorten(urlIdempotent);
  const short1b = shortenerIdempotent.shorten(urlIdempotent);
  if (short1a !== short1b) {
    console.error(
      `Shortening same URL twice returned different codes: "${short1a}" vs "${short1b}"`
    );
    passed = false;
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

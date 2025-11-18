import { createUserTable, selectActiveUsers, joinUsersOrders, createIndex } from "./database";

interface TestCase {
  query: string;
  name: string;
  requiredKeywords: string[];
}

export function runCheck(): boolean {
  let passed = true;

  const testCases: TestCase[] = [
    {
      query: createUserTable(),
      name: "createUserTable",
      requiredKeywords: ["CREATE", "TABLE", "users", "id", "name", "email", "created_at"],
    },
    {
      query: selectActiveUsers(),
      name: "selectActiveUsers",
      requiredKeywords: ["SELECT", "FROM", "users", "ORDER BY"],
    },
    {
      query: joinUsersOrders(),
      name: "joinUsersOrders",
      requiredKeywords: ["SELECT", "FROM", "users", "JOIN", "orders", "COUNT", "GROUP BY"],
    },
    {
      query: createIndex(),
      name: "createIndex",
      requiredKeywords: ["CREATE", "INDEX", "users", "email"],
    },
  ];

  console.log("Testing SQL Query Structure...\n");

  for (const tc of testCases) {
    if (!tc.query || tc.query === "") {
      console.error(`❌ ${tc.name}: Query is empty`);
      passed = false;
      continue;
    }

    let testPassed = true;
    const queryUpper = tc.query.toUpperCase();

    for (const keyword of tc.requiredKeywords) {
      if (!queryUpper.includes(keyword.toUpperCase())) {
        console.error(
          `❌ ${tc.name}: Missing required keyword "${keyword}"`
        );
        testPassed = false;
        passed = false;
      }
    }

    if (testPassed) {
      console.log(`✓ ${tc.name}`);
      console.log(`  Query: ${tc.query.substring(0, 80)}${tc.query.length > 80 ? "..." : ""}`);
    }
  }

  if (passed) {
    console.log("\n✅ All SQL queries are properly structured!");
  } else {
    console.log("\n❌ Some SQL queries are missing required components.");
    process.exit(1);
  }

  return passed;
}

if (require.main === module) {
  runCheck();
}

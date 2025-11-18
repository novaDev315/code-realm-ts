// Chapter 4: Stream of Thoughts
// Async/Await and Promises

async function delayedSum(a, b, delayMs) {
  // TODO: Return a + b after delayMs milliseconds
  return 0;
}

async function fetchMultiple(urls) {
  // TODO: Fetch all URLs in parallel and return results
  // Simulate fetch with delay (use delayedFetch helper)
  return [];
}

async function retryOperation(operation, maxRetries) {
  // TODO: Retry operation up to maxRetries times on failure
  throw new Error("Not implemented");
}

// Helper function to simulate async operation
async function delayedFetch(url) {
  return new Promise((resolve) => {
    setTimeout(() => resolve(`Data from ${url}`), 100);
  });
}

module.exports = {
  delayedSum,
  fetchMultiple,
  retryOperation,
  delayedFetch,
};

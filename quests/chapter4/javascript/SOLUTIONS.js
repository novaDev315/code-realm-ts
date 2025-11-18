// Reference solutions for Chapter 4: Stream of Thoughts

async function delayedSum(a, b, delayMs) {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(a + b);
    }, delayMs);
  });
}

async function fetchMultiple(urls) {
  // Fetch all URLs in parallel using Promise.all
  const promises = urls.map((url) => delayedFetch(url));
  return Promise.all(promises);
}

async function retryOperation(operation, maxRetries) {
  let lastError = null;

  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await operation();
    } catch (error) {
      lastError = error;
      if (attempt < maxRetries) {
        // Continue to next attempt
        continue;
      }
    }
  }

  // All retries exhausted
  throw lastError || new Error("Operation failed after retries");
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

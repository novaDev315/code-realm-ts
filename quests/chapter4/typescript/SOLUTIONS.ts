// Reference solutions for Chapter 4: Stream of Thoughts

export async function delayedSum(a: number, b: number, delayMs: number): Promise<number> {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(a + b);
    }, delayMs);
  });
}

export async function fetchMultiple(urls: string[]): Promise<string[]> {
  // Fetch all URLs in parallel using Promise.all
  const promises = urls.map((url) => delayedFetch(url));
  return Promise.all(promises);
}

export async function retryOperation<T>(
  operation: () => Promise<T>,
  maxRetries: number
): Promise<T> {
  let lastError: Error | null = null;

  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await operation();
    } catch (error) {
      lastError = error as Error;
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
export async function delayedFetch(url: string): Promise<string> {
  return new Promise((resolve) => {
    setTimeout(() => resolve(`Data from ${url}`), 100);
  });
}

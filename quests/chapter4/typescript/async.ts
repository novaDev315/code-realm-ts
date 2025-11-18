// Chapter 4: Stream of Thoughts
// Async/Await and Promises

export async function delayedSum(a: number, b: number, delayMs: number): Promise<number> {
  // TODO: Return a + b after delayMs milliseconds
  return 0;
}

export async function fetchMultiple(urls: string[]): Promise<string[]> {
  // TODO: Fetch all URLs in parallel and return results
  // Simulate fetch with delay (use delayedFetch helper)
  return [];
}

export async function retryOperation<T>(
  operation: () => Promise<T>,
  maxRetries: number
): Promise<T> {
  // TODO: Retry operation up to maxRetries times on failure
  throw new Error("Not implemented");
}

// Helper function to simulate async operation
export async function delayedFetch(url: string): Promise<string> {
  return new Promise((resolve) => {
    setTimeout(() => resolve(`Data from ${url}`), 100);
  });
}

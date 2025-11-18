// Reference solutions for Chapter 1
// These are working implementations for testing purposes

function fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

function factorial(n) {
  if (n <= 1) return 1;
  return n * factorial(n - 1);
}

function stringPermutations(str) {
  const results = [];

  if (str.length === 0) {
    return [''];
  }

  if (str.length === 1) {
    return [str];
  }

  const seen = new Set();

  for (let i = 0; i < str.length; i++) {
    const char = str[i];
    const remaining = str.slice(0, i) + str.slice(i + 1);
    const perms = stringPermutations(remaining);

    for (const perm of perms) {
      const fullPerm = char + perm;
      if (!seen.has(fullPerm)) {
        seen.add(fullPerm);
        results.push(fullPerm);
      }
    }
  }

  return results;
}

module.exports = {
  fibonacci,
  factorial,
  stringPermutations
};

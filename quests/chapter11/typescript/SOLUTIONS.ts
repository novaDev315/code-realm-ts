// Reference solutions for Chapter 11
// URL Shortener System with Base62 Encoding

const BASE62_CHARS = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

export function encodeBase62(num: number): string {
  if (num === 0) return "0";

  let result = "";
  while (num > 0) {
    result = BASE62_CHARS[num % 62] + result;
    num = Math.floor(num / 62);
  }
  return result;
}

export function decodeBase62(str: string): number {
  let result = 0;
  for (let i = 0; i < str.length; i++) {
    const char = str[i];
    const value = BASE62_CHARS.indexOf(char);
    if (value === -1) {
      throw new Error(`Invalid base62 character: ${char}`);
    }
    result = result * 62 + value;
  }
  return result;
}

export function generateShortCode(url: string, length: number): string {
  // Simple hash-based approach: convert URL to a number then encode
  let hash = 0;
  for (let i = 0; i < url.length; i++) {
    const char = url.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash; // Convert to 32bit integer
  }

  // Convert to positive number
  hash = Math.abs(hash);

  // Encode to base62
  const encoded = encodeBase62(hash);

  // Pad or trim to desired length
  if (encoded.length >= length) {
    return encoded.substring(0, length);
  }
  return encoded.padStart(length, "0");
}

export class URLShortener {
  private urlMap: Map<string, string> = new Map(); // shortCode -> longUrl
  private reverseMap: Map<string, string> = new Map(); // longUrl -> shortCode
  private counter: number = 0;

  shorten(longUrl: string): string {
    // Check if we already have a mapping for this URL
    if (this.reverseMap.has(longUrl)) {
      return this.reverseMap.get(longUrl)!;
    }

    // Generate new short code using counter
    this.counter++;
    const shortCode = encodeBase62(this.counter);

    // Store mappings
    this.urlMap.set(shortCode, longUrl);
    this.reverseMap.set(longUrl, shortCode);

    return shortCode;
  }

  expand(shortCode: string): string | null {
    return this.urlMap.get(shortCode) || null;
  }

  getAllMappings(): { short: string; long: string }[] {
    const mappings: { short: string; long: string }[] = [];
    for (const [shortCode, longUrl] of this.urlMap.entries()) {
      mappings.push({ short: shortCode, long: longUrl });
    }
    return mappings;
  }
}

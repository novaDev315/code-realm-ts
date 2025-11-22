// Test file for Chapter 11: Tower of Constructs - URL Shortener

#include <iostream>
#include <string>
#include <vector>
#include "shortener.cpp"

using namespace std;

int main() {
    bool passed = true;

    // Test base62Encode
    cout << "Testing base62Encode..." << endl;

    // Test case: 0
    string encoded0 = base62Encode(0);
    if (encoded0 != "0") {
        cerr << "base62Encode(0): expected \"0\", got \"" << encoded0 << "\"" << endl;
        passed = false;
    } else {
        cout << "  base62Encode(0) = \"0\"" << endl;
    }

    // Test case: 1
    string encoded1 = base62Encode(1);
    if (encoded1 != "1") {
        cerr << "base62Encode(1): expected \"1\", got \"" << encoded1 << "\"" << endl;
        passed = false;
    } else {
        cout << "  base62Encode(1) = \"1\"" << endl;
    }

    // Test case: 61 (should be 'z')
    string encoded61 = base62Encode(61);
    if (encoded61 != "z") {
        cerr << "base62Encode(61): expected \"z\", got \"" << encoded61 << "\"" << endl;
        passed = false;
    } else {
        cout << "  base62Encode(61) = \"z\"" << endl;
    }

    // Test case: 62 (should be '10')
    string encoded62 = base62Encode(62);
    if (encoded62 != "10") {
        cerr << "base62Encode(62): expected \"10\", got \"" << encoded62 << "\"" << endl;
        passed = false;
    } else {
        cout << "  base62Encode(62) = \"10\"" << endl;
    }

    // Test case: 3844 (62^2 = 3844, should be '100')
    string encoded3844 = base62Encode(3844);
    if (encoded3844 != "100") {
        cerr << "base62Encode(3844): expected \"100\", got \"" << encoded3844 << "\"" << endl;
        passed = false;
    } else {
        cout << "  base62Encode(3844) = \"100\"" << endl;
    }

    // Test case: large number (1000000)
    string encodedLarge = base62Encode(1000000);
    if (encodedLarge.empty()) {
        cerr << "base62Encode(1000000): returned empty string" << endl;
        passed = false;
    } else {
        cout << "  base62Encode(1000000) = \"" << encodedLarge << "\"" << endl;
    }

    // Test base62Decode
    cout << "\nTesting base62Decode..." << endl;

    // Test case: "0"
    long long decoded0 = base62Decode("0");
    if (decoded0 != 0) {
        cerr << "base62Decode(\"0\"): expected 0, got " << decoded0 << endl;
        passed = false;
    } else {
        cout << "  base62Decode(\"0\") = 0" << endl;
    }

    // Test case: "1"
    long long decoded1 = base62Decode("1");
    if (decoded1 != 1) {
        cerr << "base62Decode(\"1\"): expected 1, got " << decoded1 << endl;
        passed = false;
    } else {
        cout << "  base62Decode(\"1\") = 1" << endl;
    }

    // Test case: "z" (should be 61)
    long long decodedZ = base62Decode("z");
    if (decodedZ != 61) {
        cerr << "base62Decode(\"z\"): expected 61, got " << decodedZ << endl;
        passed = false;
    } else {
        cout << "  base62Decode(\"z\") = 61" << endl;
    }

    // Test case: "10" (should be 62)
    long long decoded10 = base62Decode("10");
    if (decoded10 != 62) {
        cerr << "base62Decode(\"10\"): expected 62, got " << decoded10 << endl;
        passed = false;
    } else {
        cout << "  base62Decode(\"10\") = 62" << endl;
    }

    // Test case: "100" (should be 3844)
    long long decoded100 = base62Decode("100");
    if (decoded100 != 3844) {
        cerr << "base62Decode(\"100\"): expected 3844, got " << decoded100 << endl;
        passed = false;
    } else {
        cout << "  base62Decode(\"100\") = 3844" << endl;
    }

    // Round-trip tests
    cout << "\nTesting encode/decode round-trip..." << endl;
    vector<long long> roundTripTests = {0, 1, 61, 62, 3844, 1000000, 123456789LL, 9223372036854775LL};

    for (long long num : roundTripTests) {
        string encoded = base62Encode(num);
        long long decoded = base62Decode(encoded);
        if (decoded != num) {
            cerr << "Round-trip failed for " << num << ": encode -> \"" << encoded << "\" -> decode -> " << decoded << endl;
            passed = false;
        } else {
            cout << "  " << num << " -> \"" << encoded << "\" -> " << decoded << endl;
        }
    }

    // Test specific character mappings
    cout << "\nTesting character mappings..." << endl;

    // 'A' should be index 10
    long long decodedA = base62Decode("A");
    if (decodedA != 10) {
        cerr << "base62Decode(\"A\"): expected 10, got " << decodedA << endl;
        passed = false;
    } else {
        cout << "  base62Decode(\"A\") = 10" << endl;
    }

    // 'Z' should be index 35
    long long decodedCapZ = base62Decode("Z");
    if (decodedCapZ != 35) {
        cerr << "base62Decode(\"Z\"): expected 35, got " << decodedCapZ << endl;
        passed = false;
    } else {
        cout << "  base62Decode(\"Z\") = 35" << endl;
    }

    // 'a' should be index 36
    long long decodedLowA = base62Decode("a");
    if (decodedLowA != 36) {
        cerr << "base62Decode(\"a\"): expected 36, got " << decodedLowA << endl;
        passed = false;
    } else {
        cout << "  base62Decode(\"a\") = 36" << endl;
    }

    if (passed) {
        cout << "\nAll tests passed!" << endl;
        return 0;
    } else {
        cout << "\nSome tests failed." << endl;
        return 1;
    }
}

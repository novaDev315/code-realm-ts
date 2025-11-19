import java.util.*;

public class SOLUTIONS {
    /**
     * Calculate fibonacci number at position n
     * Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21...
     * Each number is the sum of the two preceding ones
     *
     * @param n the position in the fibonacci sequence
     * @return the fibonacci number at position n
     */
    public static int fibonacci(int n) {
        if (n <= 1) {
            return n;
        }
        return fibonacci(n - 1) + fibonacci(n - 2);
    }

    /**
     * Calculate factorial of n
     * Factorial of n (n!) is the product of all positive integers less than or equal to n
     * 0! = 1, 1! = 1, 5! = 120
     *
     * @param n the number to calculate factorial for
     * @return the factorial of n
     */
    public static int factorial(int n) {
        if (n <= 1) {
            return 1;
        }
        return n * factorial(n - 1);
    }

    /**
     * Generate all permutations of a string
     * A permutation is an arrangement of all characters in the string
     * For example, "abc" has 6 permutations: abc, acb, bac, bca, cab, cba
     *
     * @param str the input string
     * @return list of all unique permutations
     */
    public static List<String> stringPermutations(String str) {
        List<String> result = new ArrayList<>();

        if (str.length() == 0) {
            result.add("");
            return result;
        }

        // Get all permutations of the substring without the first character
        List<String> perms = stringPermutations(str.substring(1));

        // Insert the first character at all possible positions in each permutation
        for (String perm : perms) {
            for (int i = 0; i <= perm.length(); i++) {
                result.add(perm.substring(0, i) + str.charAt(0) + perm.substring(i));
            }
        }

        return result;
    }

    public static void main(String[] args) {
        System.out.println("Fibonacci Examples:");
        for (int i = 0; i <= 10; i++) {
            System.out.println("fibonacci(" + i + ") = " + fibonacci(i));
        }

        System.out.println("\nFactorial Examples:");
        for (int i = 0; i <= 10; i++) {
            System.out.println("factorial(" + i + ") = " + factorial(i));
        }

        System.out.println("\nString Permutations Examples:");
        System.out.println("Permutations of \"abc\": " + stringPermutations("abc"));
    }
}

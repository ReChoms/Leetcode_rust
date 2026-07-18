/*
Given a string s, return true if it is a palindrome, otherwise return false.

A palindrome is a string that reads the same forward and backward. It is also case-insensitive and ignores all non-alphanumeric characters.

Note: Alphanumeric characters consist of letters (A-Z, a-z) and numbers (0-9).

Example 1:

Input: s = "Was it a car or a cat I saw?"
Output: true
Explanation: After considering only alphanumerical characters we have "wasitacaroracatisaw", which is a palindrome.

Example 2:

Input: s = "tab a cat"
Output: false
Explanation: "tabacat" is not a palindrome.

Constraints:
1 <= s.length <= 1000
s is made up of only printable ASCII characters.
*/

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        
        if bytes.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = bytes.len() - 1;

        while left < right {
            // Skip non-alphanumeric characters from the left
            if !bytes[left].is_ascii_alphanumeric() {
                left += 1;
                continue;
            }
            
            // Skip non-alphanumeric characters from the right
            if !bytes[right].is_ascii_alphanumeric() {
                right -= 1;
                continue;
            }

            // Compare characters ignoring case
            if !bytes[left].eq_ignore_ascii_case(&bytes[right]) {
                return false;
            }

            // Move both pointers inward
            left += 1;
            right -= 1;
        }

        true
    }
}

fn main() {
    println!("--- Testing Valid Palindrome ---");

    let tests = vec![
        ("Was it a car or a cat I saw?", true, "Basic example 1"),
        ("tab a cat", false, "Basic example 2"),
        (
            "A man, a plan, a canal: Panama",
            true,
            "Standard Leetcode example",
        ),
        ("race a car", false, "Standard Leetcode example 2"),
        (" ", true, "Empty space"),
        ("0P", false, "Numbers and letters"),
        ("ab_a", true, "Underscore is not alphanumeric"),
        ("a.", true, "Punctuation"),
        (".,", true, "Only punctuation"),
    ];

    for (i, (s, expected, desc)) in tests.iter().enumerate() {
        let result = Solution::is_palindrome(s.to_string());
        if result == *expected {
            println!("✅ Test {} ({}) PASSED", i + 1, desc);
        } else {
            println!(
                "❌ Test {} ({}) FAILED: s = \"{}\", expected = {}, got = {}",
                i + 1,
                desc,
                s,
                expected,
                result
            );
        }
    }
}

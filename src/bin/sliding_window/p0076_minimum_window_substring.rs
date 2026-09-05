/*
Given two strings s and t, return the shortest substring of s such that every character in t, including duplicates, is present in the substring. If such a substring does not exist, return an empty string "".

You may assume that the correct output is always unique.

Example 1:
Input: s = "OUZODYXAZV", t = "XYZ"
Output: "YXAZ"
Explanation: "YXAZ" is the shortest substring that includes "X", "Y", and "Z" from string t.

Example 2:
Input: s = "xyz", t = "xyz"
Output: "xyz"

Example 3:
Input: s = "x", t = "xy"
Output: ""

Constraints:
1 <= s.length <= 100,000
1 <= t.length <= 100,000
s and t consist of uppercase and lowercase English letters.
*/

use std::i32::MAX;

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut check_match = [0; 128];
        let mut fill_match = [0; 128];
        // Directly index with byte - no if statements needed!
        for &byte in t.as_bytes() {
            check_match[byte as usize] += 1;
        }

        let s_bytes = s.as_bytes();
        let mut left = 0;
        let mut formed = 0;
        let mut min_length = MAX;
        let mut best_start = 0;
        let required = check_match.iter().filter(|&&c| c > 0).count();
        for right in 0..s_bytes.len() {
            let right_byte = s_bytes[right] as usize;
            fill_match[right_byte] += 1;
            if check_match[right_byte] > 0 && fill_match[right_byte] == check_match[right_byte] {
                formed += 1;
            }
            while formed == required {
                let left_byte = s_bytes[left] as usize;
                fill_match[s_bytes[left] as usize] -= 1;
                let current_len = (right - left + 1) as i32;
                if current_len < min_length {
                    min_length = current_len;
                    best_start = left;
                }
                left += 1;
                if check_match[left_byte] > 0 && fill_match[left_byte] < check_match[left_byte] {
                    formed -= 1;
                }
            }
        }
        if min_length == MAX {
            return String::new();
        }
        s[best_start..best_start + min_length as usize].to_string()
    }
}
fn main() {
    println!("=== Testing Minimum Window Substring ===");

    let tests: Vec<(&str, &str, &str, &str)> = vec![
        (
            "OUZODYXAZV",
            "XYZ",
            "YXAZ",
            "Example 1 (Mixed chars with extra letters)",
        ),
        ("xyz", "xyz", "xyz", "Example 2 (Exact match)"),
        ("x", "xy", "", "Example 3 (t longer than s)"),
        ("ADOBECODEBANC", "ABC", "BANC", "Classic LeetCode Example"),
        ("a", "a", "a", "Single character match"),
        (
            "a",
            "aa",
            "",
            "Single char in s, duplicate requirement in t",
        ),
        ("aa", "aa", "aa", "Exact duplicate characters match"),
        ("bba", "ab", "ba", "Duplicate chars in s, substring at end"),
        (
            "cabwefgewcwaefgcf",
            "cae",
            "cwae",
            "Multiple possible windows with different sizes",
        ),
        (
            "aaflslflsldkalskaaa",
            "aaa",
            "aaa",
            "Repeating target letters",
        ),
        ("ab", "b", "b", "Target single letter at end"),
        ("ab", "a", "a", "Target single letter at start"),
    ];

    for (i, (s, t, expected, desc)) in tests.iter().enumerate() {
        let actual = Solution::min_window(s.to_string(), t.to_string());
        assert_eq!(
            actual,
            *expected,
            "Failed test {}: {} (s = {:?}, t = {:?})",
            i + 1,
            desc,
            s,
            t
        );
        println!("✅ Test {} ({}) PASSED", i + 1, desc);
    }
}

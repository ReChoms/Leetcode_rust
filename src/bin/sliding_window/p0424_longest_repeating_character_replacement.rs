/*
You are given a string s consisting of only uppercase english characters and an integer k. You can choose up to k characters of the string and replace them with any other uppercase English character.

After performing at most k replacements, return the length of the longest substring which contains only one distinct character.

Example 1:
Input: s = "XYYX", k = 2
Output: 4
Explanation: Either replace the 'X's with 'Y's, or replace the 'Y's with 'X's.

Example 2:
Input: s = "AAABABB", k = 1
Output: 5

Constraints:
1 <= s.length <= 100,000
0 <= k <= s.length
s consists of only uppercase english characters.
*/

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s_bytes = s.as_bytes(); // Working with bytes is faster for ASCII
        let mut counts = [0; 26];

        let mut left = 0;
        let mut max_len = 0;
        let mut max_count = 0; // Tracks the frequency of the most common character in the current window

        for right in 0..s_bytes.len() {
            // 1. Get the 0-25 index for the new character entering the window
            let right_idx = (s_bytes[right] - b'A') as usize;

            counts[right_idx] += 1;
            max_count = max_count.max(counts[right_idx]);
            // 2. The Golden Rule check: window_length - max_count > k
            while (right - left + 1 - max_count > k as usize) {
                let left_idx = (s_bytes[left] - b'A') as usize;
                counts[left_idx] -= 1;
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}

fn main() {
    println!("=== Testing Longest Repeating Character Replacement ===");

    let tests: Vec<(&str, i32, i32, &str)> = vec![
        ("XYYX", 2, 4, "Example 1"),
        ("AAABABB", 1, 5, "Example 2"),
        ("AABABBA", 1, 4, "LeetCode classic test case"),
        ("A", 0, 1, "Single char, k = 0"),
        ("A", 1, 1, "Single char, k = 1"),
        ("AAAA", 2, 4, "All identical characters"),
        ("ABBB", 2, 4, "Replacing minority char"),
        ("ABCDE", 1, 2, "All distinct characters, k = 1"),
        ("ABCDE", 2, 3, "All distinct characters, k = 2"),
        ("BAAA", 0, 3, "k = 0, longest existing streak"),
        ("ABAA", 0, 2, "k = 0, streak at end"),
    ];

    for (i, (s, k, expected, desc)) in tests.iter().enumerate() {
        let actual = Solution::character_replacement(s.to_string(), *k);
        assert_eq!(
            actual,
            *expected,
            "Failed test {} ({}): s = \"{}\", k = {}, expected {}, got {}",
            i + 1,
            desc,
            s,
            k,
            expected,
            actual
        );
        println!("✅ Test {} ({}) PASSED", i + 1, desc);
    }
}

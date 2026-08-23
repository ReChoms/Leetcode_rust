/*
Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.

In other words, return true if one of s1's permutations is the substring of s2.

Both strings only contain lowercase letters.

Example 1:
Input: s1 = "abc", s2 = "lecabee"
Output: true
Explanation: The substring "cab" is a permutation of "abc" and is present in "lecabee".

Example 2:
Input: s1 = "abc", s2 = "lecaabee"
Output: false

Constraints:
1 <= s1.length, s2.length <= 10^4
s1 and s2 consist of lowercase English letters.
*/

use std::time::Instant;

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let window_size = s1.len();
        if window_size > s2.len() {
            return false;
        }

        let mut target = [0; 26];
        let mut check = [0; 26];

        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        // 1. Build the target counts and the initial window of size window_size
        for i in 0..window_size {
            target[(s1_bytes[i] - b'a') as usize] += 1;
            check[(s2_bytes[i] - b'a') as usize] += 1;
        }

        // 2. Check if the very first window is already a match
        if target == check {
            return true;
        }

        // 3. Start sliding from index `window_size` onwards
        for i in window_size..s2_bytes.len() {
            let incoming = (s2_bytes[i] - b'a') as usize;
            let outgoing = (s2_bytes[i - window_size] - b'a') as usize;

            check[incoming] += 1;
            check[outgoing] -= 1;

            if target == check {
                return true;
            }
        }

        false
    }

    pub fn check_inclusion_v2(s1: String, s2: String) -> bool {
        let window_size = s1.len();
        if window_size > s2.len() {
            return false;
        }

        let mut target = [0; 26];
        let mut check = [0; 26];

        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        // V2 Enhancement 1: Pre-populate counts for s1 and initial s2 window
        for i in 0..window_size {
            target[(s1_bytes[i] - b'a') as usize] += 1;
            check[(s2_bytes[i] - b'a') as usize] += 1;
        }

        // V2 Enhancement 2: Maintain a `matches` count (0..=26) tracking how many letter counts match.
        let mut matches = 0;
        for i in 0..26 {
            if target[i] == check[i] {
                matches += 1;
            }
        }

        if matches == 26 {
            return true;
        }

        // V2 Enhancement 3: Truly O(1) slide per character by updating `matches` directly,
        // avoiding comparing all 26 array elements (O(26)) on every single step.
        for i in window_size..s2_bytes.len() {
            let incoming = (s2_bytes[i] - b'a') as usize;
            let outgoing = (s2_bytes[i - window_size] - b'a') as usize;

            // Handle incoming character
            check[incoming] += 1;
            if check[incoming] == target[incoming] {
                matches += 1;
            } else if check[incoming] == target[incoming] + 1 {
                matches -= 1;
            }

            // Handle outgoing character
            check[outgoing] -= 1;
            if check[outgoing] == target[outgoing] {
                matches += 1;
            } else if check[outgoing] == target[outgoing] - 1 {
                matches -= 1;
            }

            // Single integer check O(1) instead of 26-element array comparison
            if matches == 26 {
                return true;
            }
        }

        false
    }
}

fn main() {
    println!("=== Testing Permutation in String ===");

    let tests: Vec<(&str, &str, bool, &str)> = vec![
        ("abc", "lecabee", true, "User Example 1"),
        ("abc", "lecaabee", false, "User Example 2"),
        ("ab", "eidbaooo", true, "LeetCode Example 1"),
        ("ab", "eidboaoo", false, "LeetCode Example 2"),
        ("a", "a", true, "Single character exact match"),
        ("a", "b", false, "Single character mismatch"),
        ("hello", "o", false, "s1 longer than s2"),
        ("abc", "bcaefg", true, "Permutation at start"),
        ("abc", "defcba", true, "Permutation at end"),
        ("aabb", "eidbbaaooo", true, "Duplicate characters matching"),
        (
            "aabb",
            "eidbaoo",
            false,
            "Duplicate characters insufficient",
        ),
        ("adc", "dcda", true, "Overlapping valid window"),
    ];

    for (i, (s1, s2, expected, desc)) in tests.iter().enumerate() {
        let actual_v1 = Solution::check_inclusion(s1.to_string(), s2.to_string());
        let actual_v2 = Solution::check_inclusion_v2(s1.to_string(), s2.to_string());

        assert_eq!(
            actual_v1,
            *expected,
            "V1 Failed test {}: {} (s1 = {:?}, s2 = {:?})",
            i + 1,
            desc,
            s1,
            s2
        );
        assert_eq!(
            actual_v2,
            *expected,
            "V2 Failed test {}: {} (s1 = {:?}, s2 = {:?})",
            i + 1,
            desc,
            s1,
            s2
        );
        println!("✅ Test {} ({}) PASSED", i + 1, desc);
    }

    println!("\n=== Benchmarking (100,000 iterations) ===");
    let bench_s1 = "abcedfghijklmnopqrstuvwxyz";
    let bench_s2 = "zyxwvutsrqponmlkjihgfedcba".repeat(10);

    // Benchmark V1
    let start_v1 = Instant::now();
    for _ in 0..100_000 {
        Solution::check_inclusion(bench_s1.to_string(), bench_s2.to_string());
    }
    let duration_v1 = start_v1.elapsed();
    println!("V1 Duration: {:?}", duration_v1);

    // Benchmark V2
    let start_v2 = Instant::now();
    for _ in 0..100_000 {
        Solution::check_inclusion_v2(bench_s1.to_string(), bench_s2.to_string());
    }
    let duration_v2 = start_v2.elapsed();
    println!("V2 Duration: {:?}", duration_v2);

    if duration_v2 < duration_v1 {
        let multiplier = duration_v1.as_secs_f64() / duration_v2.as_secs_f64();
        println!("🚀 V2 is {:.2}x faster!", multiplier);
    } else {
        let multiplier = duration_v2.as_secs_f64() / duration_v1.as_secs_f64();
        println!("🏎️ V1 is {:.2}x faster!", multiplier);
    }
}

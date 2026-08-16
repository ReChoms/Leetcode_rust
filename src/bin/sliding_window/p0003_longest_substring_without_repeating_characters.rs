/*
Given a string s, find the length of the longest substring without duplicate characters.

A substring is a contiguous sequence of characters within a string.

Example 1:
Input: s = "zxyzxyz"
Output: 3
Explanation: The string "xyz" is the longest without duplicate characters.

Example 2:
Input: s = "xxxx"
Output: 1

Constraints:
0 <= s.length <= 50,000
s may consist of printable ASCII characters.
*/

use std::collections::HashSet;
use std::time::Instant;

struct Solution;

impl Solution {
    // Version 1: Standard Sliding Window with HashSet
    // Expands `window_end` and contracts `window_start` step-by-step when duplicates appear
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars = s.as_bytes();
        let mut window_chars = HashSet::new();
        let mut window_start = 0;
        let mut max_len = 0;

        for window_end in 0..chars.len() {
            // Shrink window from the start until duplicate character is removed
            while window_chars.contains(&chars[window_end]) {
                window_chars.remove(&chars[window_start]);
                window_start += 1;
            }

            window_chars.insert(chars[window_end]);
            max_len = max_len.max(window_end - window_start + 1);
        }

        max_len as i32
    }

    // Version 2: Optimized Sliding Window with Direct ASCII Array Lookup & Jump
    pub fn length_of_longest_substring_v2(s: String) -> i32 {
        let chars = s.as_bytes();
        // Enhancement 1: Fixed-size 128-element stack array avoids heap allocations & hashing overhead
        // Stores 1-based last seen index (0 means not seen yet)
        let mut last_seen = [0usize; 128];
        let mut window_start = 0;
        let mut max_len = 0;

        for (window_end, &current_char) in chars.iter().enumerate() {
            let prev_pos = last_seen[current_char as usize];
            // Enhancement 2: Jump `window_start` directly past duplicate's last occurrence (O(1) skip)
            if prev_pos > window_start {
                window_start = prev_pos;
            }

            last_seen[current_char as usize] = window_end + 1;
            max_len = max_len.max(window_end - window_start + 1);
        }

        max_len as i32
    }

    // Version 3: Functional fold iterator approach
    pub fn length_of_longest_substring_v3(s: String) -> i32 {
        // Enhancement: Functional accumulator carrying (last_seen_table, window_start, max_len)
        s.as_bytes()
            .iter()
            .enumerate()
            .fold(
                ([0usize; 128], 0usize, 0usize),
                |(mut last_seen, mut window_start, max_len), (window_end, &current_char)| {
                    let prev_pos = last_seen[current_char as usize];
                    if prev_pos > window_start {
                        window_start = prev_pos;
                    }
                    last_seen[current_char as usize] = window_end + 1;
                    (last_seen, window_start, max_len.max(window_end - window_start + 1))
                },
            )
            .2 as i32
    }
}

fn main() {
    println!("=== Testing Longest Substring Without Repeating Characters ===");

    let tests: Vec<(&str, i32, &str)> = vec![
        ("zxyzxyz", 3, "Example 1"),
        ("xxxx", 1, "Example 2: All identical characters"),
        ("pwwkew", 3, "Subsequence vs Substring check ('wke')"),
        ("abcabcbb", 3, "Standard repeating pattern ('abc')"),
        ("", 0, "Edge Case: Empty string"),
        (" ", 1, "Edge Case: Single space"),
        ("au", 2, "Two distinct characters"),
        ("dvdf", 3, "Restarting window with overlap ('vdf')"),
        ("abba", 2, "Edge Case: Stale index in lookup ('ab' or 'ba')"),
        ("tmmzuxt", 5, "Longer mixed pattern ('mzuxt')"),
    ];

    println!("\n--- Testing v1 (Sliding Window with HashSet) ---");
    for (i, (s, expected, desc)) in tests.iter().enumerate() {
        let actual = Solution::length_of_longest_substring(s.to_string());
        assert_eq!(actual, *expected, "v1 Failed test {} ({})", i + 1, desc);
        println!("✅ v1 Test {} ({}) PASSED", i + 1, desc);
    }

    println!("\n--- Testing v2 (Optimized Direct Array & Jump) ---");
    for (i, (s, expected, desc)) in tests.iter().enumerate() {
        let actual = Solution::length_of_longest_substring_v2(s.to_string());
        assert_eq!(actual, *expected, "v2 Failed test {} ({})", i + 1, desc);
        println!("✅ v2 Test {} ({}) PASSED", i + 1, desc);
    }

    println!("\n--- Testing v3 (Functional fold) ---");
    for (i, (s, expected, desc)) in tests.iter().enumerate() {
        let actual = Solution::length_of_longest_substring_v3(s.to_string());
        assert_eq!(actual, *expected, "v3 Failed test {} ({})", i + 1, desc);
        println!("✅ v3 Test {} ({}) PASSED", i + 1, desc);
    }

    println!("\n=== Performance Benchmarking (100,000 runs) ===");
    let benchmark_input = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+"
        .repeat(5); // 375-char ASCII string
    let iterations = 100_000;

    // Benchmark v1
    let start_v1 = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(Solution::length_of_longest_substring(benchmark_input.clone()));
    }
    let duration_v1 = start_v1.elapsed();

    // Benchmark v2
    let start_v2 = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(Solution::length_of_longest_substring_v2(benchmark_input.clone()));
    }
    let duration_v2 = start_v2.elapsed();

    // Benchmark v3
    let start_v3 = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(Solution::length_of_longest_substring_v3(benchmark_input.clone()));
    }
    let duration_v3 = start_v3.elapsed();

    println!("v1 (HashSet Sliding Window) : {:?}", duration_v1);
    println!("v2 (Direct Array Jump)      : {:?}", duration_v2);
    println!("v3 (Functional fold)        : {:?}", duration_v3);

    let v1_micros = duration_v1.as_micros() as f64;
    let v2_micros = duration_v2.as_micros() as f64;
    let v3_micros = duration_v3.as_micros() as f64;

    if v2_micros < v1_micros {
        println!(
            "⚡ v2 is {:.2}x faster than v1!",
            v1_micros / v2_micros.max(1.0)
        );
    }
    if v3_micros < v1_micros {
        println!(
            "⚡ v3 is {:.2}x faster than v1!",
            v1_micros / v3_micros.max(1.0)
        );
    }
    println!(
        "📊 v2 vs v3: v2 is {:.2}x compared to v3",
        v3_micros / v2_micros.max(1.0)
    );
}

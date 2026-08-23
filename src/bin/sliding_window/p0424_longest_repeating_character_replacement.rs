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

use std::time::Instant;

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s_bytes = s.as_bytes();
        let mut counts = [0; 26];

        let mut left = 0;
        let mut max_len = 0;
        let mut max_count = 0;

        for right in 0..s_bytes.len() {
            let right_idx = (s_bytes[right] - b'A') as usize;

            counts[right_idx] += 1;
            max_count = max_count.max(counts[right_idx]);
            
            while right - left + 1 - max_count > k as usize {
                let left_idx = (s_bytes[left] - b'A') as usize;
                counts[left_idx] -= 1;
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }

    pub fn character_replacement_v2(s: String, k: i32) -> i32 {
        let s_bytes = s.as_bytes();
        let mut counts = [0; 26];
        
        let mut left = 0;
        let mut max_count = 0;

        // V2 Enhancement 1: Using iter().enumerate() is more idiomatic in Rust 
        // and avoids bounds checking on every `s_bytes[right]` access.
        for (right, &byte) in s_bytes.iter().enumerate() {
            let right_idx = (byte - b'A') as usize;
            
            counts[right_idx] += 1;
            max_count = max_count.max(counts[right_idx]);

            // V2 Enhancement 2: Change `while` to `if`. 
            // We don't actually need to shrink the window fully to find the max length!
            // If the window is invalid, we just slide the whole window over by 1 
            // (moving both left and right simultaneously). 
            // The window NEVER shrinks, it only stays the same size or grows when it's valid.
            if right - left + 1 - max_count > k as usize {
                let left_idx = (s_bytes[left] - b'A') as usize;
                counts[left_idx] -= 1;
                left += 1;
            }
        }
        
        // V2 Enhancement 3: Since the window never shrinks, its final size 
        // (total length - left pointer) is guaranteed to be the max length we ever found!
        (s_bytes.len() - left) as i32
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
        let actual_v1 = Solution::character_replacement(s.to_string(), *k);
        let actual_v2 = Solution::character_replacement_v2(s.to_string(), *k);
        
        assert_eq!(actual_v1, *expected, "V1 Failed test {}", i + 1);
        assert_eq!(actual_v2, *expected, "V2 Failed test {}", i + 1);
        
        println!("✅ Test {} ({}) PASSED", i + 1, desc);
    }

    println!("\n=== Benchmarking (100,000 iterations) ===");
    let bench_string = "AABABBA".repeat(100);
    let bench_k = 20;
    
    // Benchmark V1
    let start_v1 = Instant::now();
    for _ in 0..100_000 {
        Solution::character_replacement(bench_string.clone(), bench_k);
    }
    let duration_v1 = start_v1.elapsed();
    println!("V1 Duration: {:?}", duration_v1);

    // Benchmark V2
    let start_v2 = Instant::now();
    for _ in 0..100_000 {
        Solution::character_replacement_v2(bench_string.clone(), bench_k);
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

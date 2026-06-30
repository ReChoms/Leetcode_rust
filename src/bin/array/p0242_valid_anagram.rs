/*
Given two strings s and t, return true if the two strings are anagrams of each other, otherwise return false.

An anagram is a string that contains the exact same characters as another string, but the order of the characters can be different.

Example 1:

Input: s = "racecar", t = "carrace"
Output: true

Example 2:

Input: s = "jar", t = "jam"
Output: false

Constraints:

1 <= s.length, t.length <= 5 * 10^4
s and t consist of lowercase English letters.
*/

use std::array;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut vecy = vec![0; 26];
        for si in s.chars() {
            let idx = (si as u8 - b'a') as usize;
            vecy[idx] += 1;
        }
        for ti in t.chars() {
            let idx = (ti as u8 - b'a') as usize;
            vecy[idx] -= 1;
        }
        if vecy.iter().all(|&x| x == 0) {
            true
        } else {
            false
        }
    }

    pub fn is_anagram_v2(s: String, t: String) -> bool {
        // OPTIMIZATION 1: Early length check. If lengths differ, it's impossible to be an anagram.
        if s.len() != t.len() {
            return false;
        }

        let mut vecy = [0; 26];

        // OPTIMIZATION 2: Use .bytes() instead of .chars(). 
        // This avoids UTF-8 parsing overhead since we are guaranteed ASCII inputs.
        for b in s.bytes() {
            vecy[(b - b'a') as usize] += 1;
        }
        for b in t.bytes() {
            vecy[(b - b'a') as usize] -= 1;
        }

        // OPTIMIZATION 3: Implicit boolean return. No need for `if condition { true } else { false }`
        vecy.iter().all(|&x| x == 0)
    }
}

fn main() {
    let test_cases = vec![
        // Standard examples
        ("racecar", "carrace", true),
        ("jar", "jam", false),
        // Edge cases
        // Different lengths
        ("a", "ab", false),
        ("ab", "a", false),
        // Same characters but different frequencies
        ("aacc", "ccac", false),
        ("aabbbb", "aaaabb", false),
        // Identical strings
        ("hello", "hello", true),
        // Single characters
        ("a", "a", true),
        ("a", "b", false),
    ];

    let mut all_passed = true;
    for (i, (s, t, expected)) in test_cases.iter().enumerate() {
        let result = Solution::is_anagram(s.to_string(), t.to_string());
        if result == *expected {
            println!("Test Case {} PASSED", i + 1);
        } else {
            println!(
                "Test Case {} FAILED: is_anagram(\"{}\", \"{}\") returned {}, expected {}",
                i + 1,
                s,
                t,
                result,
                expected
            );
            all_passed = false;
        }
    }

    if all_passed {
        println!("All test cases passed for V1!\n");
    } else {
        println!("Some test cases failed for V1.\n");
    }

    // Run tests for V2
    let mut all_passed_v2 = true;
    for (i, (s, t, expected)) in test_cases.iter().enumerate() {
        let result = Solution::is_anagram_v2(s.to_string(), t.to_string());
        if result != *expected {
            println!("V2 Test Case {} FAILED: expected {}, got {}", i + 1, expected, result);
            all_passed_v2 = false;
        }
    }
    if all_passed_v2 {
        println!("All test cases passed for V2!\n");
    }

    // Benchmarking
    use std::time::Instant;

    let iterations = 10_000;
    let long_s = "z".repeat(1000);
    let long_t = "z".repeat(1000);

    println!("Starting Speed Benchmark ({} iterations)...", iterations);

    // Benchmark V1
    let start_v1 = Instant::now();
    for _ in 0..iterations {
        Solution::is_anagram(long_s.clone(), long_t.clone());
    }
    let duration_v1 = start_v1.elapsed();

    // Benchmark V2
    let start_v2 = Instant::now();
    for _ in 0..iterations {
        Solution::is_anagram_v2(long_s.clone(), long_t.clone());
    }
    let duration_v2 = start_v2.elapsed();

    println!("V1 Execution Time: {:?}", duration_v1);
    println!("V2 Execution Time: {:?}", duration_v2);

    if duration_v2 < duration_v1 {
        let multiplier = duration_v1.as_secs_f64() / duration_v2.as_secs_f64();
        println!("V2 is {:.2}x faster!", multiplier);
    } else {
        println!("V1 was faster (this is rare, might be CPU noise).");
    }
}

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
use std::time::Instant;

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
        //formed counter means how much of solution or check
        let mut min_length = MAX;
        let mut best_start = 0;
        let required = check_match.iter().filter(|&&c| c > 0).count();
        for right in 0..s_bytes.len() {
            let right_byte = s_bytes[right] as usize;
            fill_match[right_byte] += 1;
            // check if the byte or letter is a match, if yes increase formed counter
            if check_match[right_byte] > 0 && fill_match[right_byte] == check_match[right_byte] {
                formed += 1;
            }
            // while formed == required means as long as we have the overlapp which means in oour calced
            // string is the needed solution present from t
            while formed == required {
                let left_byte = s_bytes[left] as usize;
                // deacrease fill_match s_bytes at postion left by 1, we want to shrink window and so the current
                // left value needs to be subtracted by 1.
                fill_match[s_bytes[left] as usize] -= 1;
                let current_len = (right - left + 1) as i32;
                // update min length and start so later the solution can be formed
                if current_len < min_length {
                    min_length = current_len;
                    best_start = left;
                }
                left += 1;
                // check if counter at positon left_byte(we have sorted for ASCII letters) is smaller than the
                // comparison if that is the case formed must be decreased because
                // then the fill_match doenst have enough chars anymore
                if check_match[left_byte] > 0 && fill_match[left_byte] < check_match[left_byte] {
                    formed -= 1;
                }
            }
        }
        // early checkout
        if min_length == MAX {
            return String::new();
        }
        //forming of solution string
        s[best_start..best_start + min_length as usize].to_string()
    }

    pub fn min_window_v2(s: String, t: String) -> String {
        // V2 Enhancement 1: Fast-exit guard clause.
        // If s is shorter than t, it is mathematically impossible to contain all characters of t.
        if s.len() < t.len() || t.is_empty() {
            return String::new();
        }

        // V2 Enhancement 2: Single signed frequency delta array [i32; 128].
        // Cuts stack footprint in half (512 bytes vs 1024 bytes) compared to two separate arrays,
        // maximizing L1 cache residency and eliminating separate fill_match lookups.
        let mut target_counts = [0i32; 128];
        for &byte in t.as_bytes() {
            target_counts[byte as usize] += 1;
        }

        let s_bytes = s.as_bytes();
        let mut left = 0;
        let mut best_start = 0;
        // V2 Enhancement 3: Use usize throughout to eliminate repeated `as i32` / `as usize` casts.
        let mut min_len = usize::MAX;
        // V2 Enhancement 4: Track total missing characters instead of distinct `formed` count.
        // Avoids the initial `.filter().count()` scan over the frequency array.
        let mut missing = t.len();

        for right in 0..s_bytes.len() {
            let r_byte = s_bytes[right] as usize;
            // If target_counts[r_byte] > 0, this character is actively required by t
            if target_counts[r_byte] > 0 {
                missing -= 1;
            }
            // Decrement count; excess characters in the window simply drop below zero
            target_counts[r_byte] -= 1;

            // When missing == 0, the current window [left..=right] contains all characters of t
            while missing == 0 {
                let current_len = right - left + 1;
                if current_len < min_len {
                    min_len = current_len;
                    best_start = left;
                }

                let l_byte = s_bytes[left] as usize;
                // looking at item before throwing it out so if its 0 before i would become 1 missing
                // If target_counts[l_byte] == 0, evicting this character breaks the quota
                if target_counts[l_byte] == 0 {
                    missing += 1;
                }
                target_counts[l_byte] += 1;
                left += 1;
            }
        }

        if min_len == usize::MAX {
            String::new()
        } else {
            s[best_start..best_start + min_len].to_string()
        }
    }

    pub fn min_window_v3(s: String, t: String) -> String {
        let t_len = t.len();
        // V3 Enhancement 1: Immediate guard clause.
        // If s is shorter than t or t is empty, return immediately.
        if s.len() < t_len || t_len == 0 {
            return String::new();
        }

        // V3 Enhancement 2: [i32; 256] array to eliminate compiler bounds checks.
        // Every u8 is guaranteed to be in 0..=255, so the compiler removes bounds checks entirely.
        let mut target_counts = [0i32; 256];
        for &byte in t.as_bytes() {
            target_counts[byte as usize] += 1;
        }

        let s_bytes = s.as_bytes();
        let mut left = 0;
        let mut best_start = 0;
        let mut min_len = usize::MAX;
        let mut missing = t_len;

        for right in 0..s_bytes.len() {
            let r_byte = s_bytes[right] as usize;
            if target_counts[r_byte] > 0 {
                missing -= 1;
            }
            target_counts[r_byte] -= 1;

            while missing == 0 {
                let current_len = right - left + 1;
                if current_len < min_len {
                    min_len = current_len;
                    best_start = left;

                    // V3 Enhancement 3: Theoretical minimum early-exit short-circuit.
                    // No valid window can ever be shorter than t.len(). If we find one of exact length,
                    // we can stop scanning immediately without processing the remainder of the string!
                    if min_len == t_len {
                        return s[best_start..best_start + min_len].to_string();
                    }
                }

                let l_byte = s_bytes[left] as usize;
                if target_counts[l_byte] == 0 {
                    missing += 1;
                }
                target_counts[l_byte] += 1;
                left += 1;
            }
        }

        if min_len == usize::MAX {
            String::new()
        } else {
            s[best_start..best_start + min_len].to_string()
        }
    }

    pub fn min_window_v4(s: String, t: String) -> String {
        let t_len = t.len();
        if s.len() < t_len || t_len == 0 {
            return String::new();
        }

        // V4 Unsafe Enhancement 1: Best of both worlds for memory footprint.
        // Keep the small 128-element table (512 bytes) for ultra-fast L1 zero-init,
        // but use `get_unchecked_mut` to strip out all bounds checks.
        let mut target_counts = [0i32; 128];
        let t_bytes = t.as_bytes();
        unsafe {
            for i in 0..t_len {
                let byte = *t_bytes.get_unchecked(i) as usize;
                *target_counts.get_unchecked_mut(byte) += 1;
            }
        }

        let s_bytes = s.as_bytes();
        let s_ptr = s_bytes.as_ptr();
        let mut left = 0;
        let mut best_start = 0;
        let mut min_len = usize::MAX;
        let mut missing = t_len;

        for right in 0..s_bytes.len() {
            unsafe {
                // V4 Unsafe Enhancement 2: Raw pointer arithmetic for reading s without slice overhead
                let r_byte = *s_ptr.add(right) as usize;
                let r_count = target_counts.get_unchecked_mut(r_byte);
                if *r_count > 0 {
                    missing -= 1;
                }
                *r_count -= 1;

                while missing == 0 {
                    let current_len = right - left + 1;
                    if current_len < min_len {
                        min_len = current_len;
                        best_start = left;

                        if min_len == t_len {
                            // V4 Unsafe Enhancement 3: Skip UTF-8 re-validation when returning
                            let raw_slice = std::slice::from_raw_parts(s_ptr.add(best_start), min_len);
                            return std::str::from_utf8_unchecked(raw_slice).to_string();
                        }
                    }

                    let l_byte = *s_ptr.add(left) as usize;
                    let l_count = target_counts.get_unchecked_mut(l_byte);
                    if *l_count == 0 {
                        missing += 1;
                    }
                    *l_count += 1;
                    left += 1;
                }
            }
        }

        if min_len == usize::MAX {
            String::new()
        } else {
            unsafe {
                let raw_slice = std::slice::from_raw_parts(s_ptr.add(best_start), min_len);
                std::str::from_utf8_unchecked(raw_slice).to_string()
            }
        }
    }

    // V5: Branchless arithmetic + standard String return signature.
    // Replaces `if` statements with branchless conditional arithmetic (setg/sete)
    // while maintaining LeetCode's `String` input/output signature.
    pub fn min_window_v5(s: String, t: String) -> String {
        let t_len = t.len();
        if s.len() < t_len || t_len == 0 {
            return String::new();
        }

        let mut target_counts = [0i32; 128];
        for &byte in t.as_bytes() {
            target_counts[byte as usize] += 1;
        }

        let s_bytes = s.as_bytes();
        let mut left = 0;
        let mut best_start = 0;
        let mut min_len = usize::MAX;
        let mut missing = t_len;

        for right in 0..s_bytes.len() {
            let r_byte = s_bytes[right] as usize;

            // Branchless subtraction: evaluates to 1 if needed, 0 otherwise (setg assembly).
            missing -= (target_counts[r_byte] > 0) as usize;
            target_counts[r_byte] -= 1;

            while missing == 0 {
                let current_len = right - left + 1;
                if current_len < min_len {
                    min_len = current_len;
                    best_start = left;

                    if min_len == t_len {
                        return s[best_start..best_start + min_len].to_string();
                    }
                }

                let l_byte = s_bytes[left] as usize;
                // Branchless addition: evaluates to 1 if count == 0, 0 otherwise (sete assembly).
                missing += (target_counts[l_byte] == 0) as usize;
                target_counts[l_byte] += 1;
                left += 1;
            }
        }

        if min_len == usize::MAX {
            String::new()
        } else {
            s[best_start..best_start + min_len].to_string()
        }
    }

    /// V6 (Pure HFT-style zero-allocation):
    /// 1. Branchless math: removes `if` branches using conditional set (`setg`/`sete`).
    /// 2. Zero-allocation: borrows `&'a str` directly, completely bypassing `malloc`.
    pub fn min_window_v6<'a>(s: &'a str, t: &str) -> &'a str {
        let t_len = t.len();
        if s.len() < t_len || t_len == 0 {
            return "";
        }

        let mut target_counts = [0i32; 128];
        for &byte in t.as_bytes() {
            target_counts[byte as usize] += 1;
        }

        let s_bytes = s.as_bytes();
        let mut left = 0;
        let mut best_start = 0;
        let mut min_len = usize::MAX;
        let mut missing = t_len;

        for right in 0..s_bytes.len() {
            let r_byte = s_bytes[right] as usize;

            // HFT Enhancement 1: Branchless subtract.
            // Evaluates to 1 if needed, 0 otherwise. Translates to branchless `setg` assembly.
            missing -= (target_counts[r_byte] > 0) as usize;
            target_counts[r_byte] -= 1;

            while missing == 0 {
                let current_len = right - left + 1;
                if current_len < min_len {
                    min_len = current_len;
                    best_start = left;

                    // Theoretical minimum early-exit
                    if min_len == t_len {
                        return &s[best_start..best_start + min_len];
                    }
                }

                let l_byte = s_bytes[left] as usize;
                // HFT Enhancement 2: Branchless addition.
                // Evaluates to 1 if count == 0, 0 otherwise (`sete` instruction, zero pipeline stalls).
                missing += (target_counts[l_byte] == 0) as usize;
                target_counts[l_byte] += 1;
                left += 1;
            }
        }

        if min_len == usize::MAX {
            ""
        } else {
            // HFT Enhancement 3: Zero-allocation slice return.
            // Returns a borrow directly from input string with 0 calls to the heap allocator.
            &s[best_start..best_start + min_len]
        }
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
        let actual_v1 = Solution::min_window(s.to_string(), t.to_string());
        let actual_v2 = Solution::min_window_v2(s.to_string(), t.to_string());
        let actual_v3 = Solution::min_window_v3(s.to_string(), t.to_string());
        let actual_v4 = Solution::min_window_v4(s.to_string(), t.to_string());

        assert_eq!(
            actual_v1,
            *expected,
            "V1 Failed test {}: {} (s = {:?}, t = {:?})",
            i + 1,
            desc,
            s,
            t
        );
        assert_eq!(
            actual_v2,
            *expected,
            "V2 Failed test {}: {} (s = {:?}, t = {:?})",
            i + 1,
            desc,
            s,
            t
        );
        assert_eq!(
            actual_v3,
            *expected,
            "V3 Failed test {}: {} (s = {:?}, t = {:?})",
            i + 1,
            desc,
            s,
            t
        );
        assert_eq!(
            actual_v4,
            *expected,
            "V4 Failed test {}: {} (s = {:?}, t = {:?})",
            i + 1,
            desc,
            s,
            t
        );
        let actual_v5 = Solution::min_window_v5(s.to_string(), t.to_string());
        assert_eq!(
            actual_v5,
            *expected,
            "V5 Failed test {}: {} (s = {:?}, t = {:?})",
            i + 1,
            desc,
            s,
            t
        );
        println!("✅ Test {} ({}) PASSED", i + 1, desc);
    }

    println!("\n=== Benchmark 1: General Case (min window > t.len()) (100,000 runs) ===");
    let bench_s1 = "ADOBECODEBANC".repeat(5);
    let bench_t1 = "ABC";

    let start_v1 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window(bench_s1.clone(), bench_t1.to_string());
    }
    let dur_v1 = start_v1.elapsed();

    let start_v2 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window_v2(bench_s1.clone(), bench_t1.to_string());
    }
    let dur_v2 = start_v2.elapsed();

    let start_v3 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window_v3(bench_s1.clone(), bench_t1.to_string());
    }
    let dur_v3 = start_v3.elapsed();

    let start_v4 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window_v4(bench_s1.clone(), bench_t1.to_string());
    }
    let dur_v4 = start_v4.elapsed();

    let start_v5 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window_v5(bench_s1.clone(), bench_t1.to_string());
    }
    let dur_v5 = start_v5.elapsed();

    // Pure HFT Zero-Alloc Benchmark (borrowing slice directly without malloc)
    let start_v6 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window_v6(&bench_s1, bench_t1);
    }
    let dur_v6 = start_v6.elapsed();

    println!("V1 (Original):                   {:?}", dur_v1);
    println!("V2 (Single Array [i32; 128]):    {:?}", dur_v2);
    println!("V3 ([i32; 256] + Early Exit):    {:?}", dur_v3);
    println!("V4 (Unsafe Raw Pointers):        {:?}", dur_v4);
    println!("V5 (Branchless Math + Alloc):    {:?}", dur_v5);
    println!("V6 (Pure HFT Zero-Alloc &str):   {:?}", dur_v6);
    println!(
        "🚀 V6 (Zero-Alloc) vs V1: {:.2}x faster",
        dur_v1.as_secs_f64() / dur_v6.as_secs_f64()
    );
    println!(
        "🔥 V6 (Zero-Alloc) vs V2: {:.2}x faster!",
        dur_v2.as_secs_f64() / dur_v6.as_secs_f64()
    );

    println!(
        "\n=== Benchmark 2: Exact-Match Early Exit (min window == t.len()) (100,000 runs) ==="
    );
    let bench_s2 = "ADOBECODEBANC".repeat(5);
    let bench_t2 = "BANC";

    let start2_v1 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window(bench_s2.clone(), bench_t2.to_string());
    }
    let dur2_v1 = start2_v1.elapsed();

    let start2_v2 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window_v2(bench_s2.clone(), bench_t2.to_string());
    }
    let dur2_v2 = start2_v2.elapsed();

    let start2_v3 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window_v3(bench_s2.clone(), bench_t2.to_string());
    }
    let dur2_v3 = start2_v3.elapsed();

    let start2_v4 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window_v4(bench_s2.clone(), bench_t2.to_string());
    }
    let dur2_v4 = start2_v4.elapsed();

    let start2_v5 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window_v5(bench_s2.clone(), bench_t2.to_string());
    }
    let dur2_v5 = start2_v5.elapsed();

    let start2_v6 = Instant::now();
    for _ in 0..100_000 {
        Solution::min_window_v6(&bench_s2, bench_t2);
    }
    let dur2_v6 = start2_v6.elapsed();

    println!("V1 (Original):                   {:?}", dur2_v1);
    println!("V2 (Single Array [i32; 128]):    {:?}", dur2_v2);
    println!("V3 ([i32; 256] + Early Exit):    {:?}", dur2_v3);
    println!("V4 (Unsafe Raw Pointers):        {:?}", dur2_v4);
    println!("V5 (Branchless Math + Alloc):    {:?}", dur2_v5);
    println!("V6 (Pure HFT Zero-Alloc &str):   {:?}", dur2_v6);
    println!(
        "🚀 V6 (Zero-Alloc) vs V1: {:.2}x faster",
        dur2_v1.as_secs_f64() / dur2_v6.as_secs_f64()
    );
    println!(
        "🔥 V6 (Zero-Alloc) vs V2: {:.2}x faster!",
        dur2_v2.as_secs_f64() / dur2_v6.as_secs_f64()
    );

}

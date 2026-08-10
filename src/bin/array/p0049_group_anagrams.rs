/*
Group Anagrams
Medium

Given an array of strings strs, group all anagrams together into sublists. You may return the output in any order.

An anagram is a string that contains the exact same characters as another string, but the order of the characters can be different.

Example 1:
Input: strs = ["act","pots","tops","cat","stop","hat"]
Output: [["hat"],["act", "cat"],["stop", "pots", "tops"]]

Example 2:
Input: strs = ["x"]
Output: [["x"]]

Example 3:
Input: strs = [""]
Output: [[""]]

Constraints:
1 <= strs.length <= 1000.
0 <= strs[i].length <= 100
strs[i] is made up of lowercase English letters.
*/

pub struct Solution;
use ::std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len());
        for str in strs {
            let mut counts = [0u8; 26];
            //deconstruct string
            for byty in str.bytes() {
                let cnum = (byty - b'a') as usize;
                counts[cnum] += 1;
            }
            res.entry(counts).or_insert(Vec::new()).push(str);
        }
        res.into_values().collect()
    }
}

fn main() {
    let test_cases = vec![
        (
            vec!["act", "pots", "tops", "cat", "stop", "hat"],
            vec![
                vec!["hat"],
                vec!["act", "cat"],
                vec!["stop", "pots", "tops"],
            ],
        ),
        (vec!["x"], vec![vec!["x"]]),
        (vec![""], vec![vec![""]]),
        // Edge case: duplicate strings
        (vec!["a", "b", "a"], vec![vec!["a", "a"], vec!["b"]]),
        // Edge case: all same length but no anagrams
        (
            vec!["abc", "def", "ghi"],
            vec![vec!["abc"], vec!["def"], vec!["ghi"]],
        ),
    ];

    // Helper to sort the result for comparison, since output order doesn't matter
    fn normalize(mut res: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in &mut res {
            group.sort();
        }
        res.sort();
        res
    }

    let mut all_passed = true;
    for (i, (input, expected)) in test_cases.into_iter().enumerate() {
        let input_strs: Vec<String> = input.into_iter().map(|s| s.to_string()).collect();
        let expected_strs: Vec<Vec<String>> = expected
            .into_iter()
            .map(|g| g.into_iter().map(|s| s.to_string()).collect())
            .collect();

        let result = Solution::group_anagrams(input_strs.clone());
        let normalized_result = normalize(result.clone());
        let normalized_expected = normalize(expected_strs.clone());

        if normalized_result == normalized_expected {
            println!("Test {} PASSED", i + 1);
        } else {
            all_passed = false;
            println!("Test {} FAILED", i + 1);
            println!("  Input: {:?}", input_strs);
            println!("  Expected: {:?}", normalized_expected);
            println!("  Got: {:?}", normalized_result);
        }
    }

    if all_passed {
        println!("All standard tests PASSED.");
    }

    // --- Benchmarking ---
    // Uncomment when implementing the solution to measure performance
    /*
    println!("\n--- Benchmarking ---");
    use std::time::Instant;
    let bench_input = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()].repeat(1000);
    let start = Instant::now();
    for _ in 0..100 {
        let _ = Solution::group_anagrams(bench_input.clone());
    }
    let duration = start.elapsed();
    println!("Time elapsed for 100 iterations: {:?}", duration);
    */
}

/*
Given an array of integers nums and an integer target, return the indices i and j such that nums[i] + nums[j] == target and i != j.

You may assume that every input has exactly one pair of indices i and j that satisfy the condition.

Return the answer with the smaller index first.

Example 1:
Input: nums = [3,4,5,6], target = 7
Output: [0,1]
Explanation: nums[0] + nums[1] == 7, so we return [0, 1].

Example 2:
Input: nums = [4,5,6], target = 10
Output: [0,2]

Example 3:
Input: nums = [5,5], target = 10
Output: [0,1]

Constraints:
2 <= nums.length <= 1000
-10,000,000 <= nums[i] <= 10,000,000
-10,000,000 <= target <= 10,000,000
Only one valid answer exists.
*/

use core::num;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashymap = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(&x) = hashymap.get(&diff) {
                return vec![x, i as i32];
            } else {
                hashymap.insert(num, i as i32);
            }
        }
        vec![1]
    }

    pub fn two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // OPTIMIZATION 1: Pre-allocate memory for the HashMap to prevent re-hashing and re-allocating
        let mut hashymap = HashMap::with_capacity(nums.len());
        
        for (i, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(&x) = hashymap.get(&diff) {
                return vec![x, i as i32];
            } else {
                hashymap.insert(num, i as i32);
            }
        }
        
        // OPTIMIZATION 2: Use unreachable!() for code paths that should mathematically never occur
        unreachable!("Constraints guarantee exactly one valid answer exists.");
    }
}

fn main() {
    let test_cases = vec![
        // Standard examples
        (vec![3, 4, 5, 6], 7, vec![0, 1]),
        (vec![4, 5, 6], 10, vec![0, 2]),
        (vec![5, 5], 10, vec![0, 1]),
        // Edge cases
        // Negative numbers
        (vec![-1, -2, -3, -4, -5], -8, vec![2, 4]),
        // Mixed positive and negative
        (vec![2, -7, 11, 15], 4, vec![1, 2]),
        // Zeroes
        (vec![0, 4, 3, 0], 0, vec![0, 3]),
        // Large inputs (constraints say up to 10M)
        (vec![10000000, 2, 3, 5000000], 15000000, vec![0, 3]),
    ];

    let mut all_passed = true;
    for (i, (nums, target, expected)) in test_cases.iter().enumerate() {
        let result = Solution::two_sum(nums.clone(), *target);
        if result == *expected {
            println!("Test Case {} PASSED", i + 1);
        } else {
            println!(
                "Test Case {} FAILED: two_sum({:?}, {}) returned {:?}, expected {:?}",
                i + 1,
                nums,
                target,
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
    for (i, (nums, target, expected)) in test_cases.iter().enumerate() {
        let result = Solution::two_sum_v2(nums.clone(), *target);
        if result != *expected {
            println!("V2 Test Case {} FAILED: expected {:?}, got {:?}", i + 1, expected, result);
            all_passed_v2 = false;
        }
    }
    if all_passed_v2 {
        println!("All test cases passed for V2!\n");
    }

    // Benchmarking
    use std::time::Instant;

    let iterations = 10_000;
    // Create a large array of 10,000 items where the answer is at the very end
    let mut long_nums = vec![1; 10_000];
    long_nums[9998] = 5;
    long_nums[9999] = 5;
    let target = 10;

    println!("Starting Speed Benchmark ({} iterations on 10,000-item array)...", iterations);

    let start_v1 = Instant::now();
    for _ in 0..iterations {
        Solution::two_sum(long_nums.clone(), target);
    }
    let duration_v1 = start_v1.elapsed();

    let start_v2 = Instant::now();
    for _ in 0..iterations {
        Solution::two_sum_v2(long_nums.clone(), target);
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

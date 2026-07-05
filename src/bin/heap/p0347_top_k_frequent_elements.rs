/*
Given an integer array nums and an integer k, return the k most frequent elements within the array.

The test cases are generated such that the answer is always unique.

You may return the output in any order.

Example 1:
Input: nums = [1,2,2,3,3,3], k = 2
Output: [2,3]

Example 2:
Input: nums = [7,7], k = 1
Output: [7]
*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hashy = HashMap::with_capacity(nums.len());
        for num in nums {
            if let Some(count) = hashy.get_mut(&num) {
                // The number exists, increment the count!
                *count += 1;
            } else {
                // The number doesn't exist yet, insert it with a count of 1.
                hashy.insert(num, 1);
            }
        }

        // We set up the BinaryHeap (Min-Heap) for you:
        let mut heap = BinaryHeap::new();

        // `hashy` is an iterator of `(key, value)` pairs, so we unpack it:
        for (num, count) in hashy {
            // Push the current item into the Min-Heap
            heap.push(Reverse((count, num)));

            // If the heap grows larger than k, pop the element with the smallest frequency
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        let mut result = Vec::with_capacity(k as usize);
        while let Some(Reverse((_, num))) = heap.pop() {
            result.push(num);
        }

        result
    }

    // --- VERSION 2: Bucket Sort (O(N) Time Complexity) ---
    // Note on Performance:
    // While Bucket Sort has a superior theoretical time complexity of O(N) compared
    // to the Min-Heap's O(N log k), the Min-Heap is often faster in practice for
    // sparse data distributions or large input arrays. This is because Bucket Sort
    // requires allocating an array of vectors proportional to the size of `nums`,
    // resulting in significant memory allocation overhead. The Min-Heap maintains
    // a strict capacity of `k`, keeping allocations minimal.
    pub fn top_k_frequent_v2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Step 1: Count frequencies (same as before, but using a cleaner idiom)
        let mut hashy = HashMap::with_capacity(nums.len());
        for &num in &nums {
            // `entry().or_insert(0)` is the idiomatic way to do the if/else we wrote in v1!
            *hashy.entry(num).or_insert(0) += 1;
        }

        // Step 2: Bucket Sort
        // Since a number can appear at most `nums.len()` times, we create an array of "buckets".
        // The index of the array represents the *frequency*, and the value is a list of numbers with that frequency.
        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];

        for (num, count) in hashy {
            buckets[count as usize].push(num);
        }

        // Step 3: Gather the top `k` elements
        // We iterate backwards from the highest possible frequency (nums.len()) down to 0.
        let mut result = Vec::with_capacity(k as usize);
        for i in (0..buckets.len()).rev() {
            for &num in &buckets[i] {
                result.push(num);
                if result.len() == k as usize {
                    return result;
                }
            }
        }

        result
    }
}

fn main() {
    println!("--- Testing Top K Frequent Elements ---");

    let test_cases = vec![
        (vec![1, 2, 2, 3, 3, 3], 2, vec![2, 3]),
        (vec![7, 7], 1, vec![7]),
        (vec![1], 1, vec![1]),
        (vec![-1, -1], 1, vec![-1]),
        (vec![4, 1, -1, 2, -1, 2, 3], 2, vec![-1, 2]),
    ];

    let mut all_passed = true;
    for (i, (nums, k, expected)) in test_cases.into_iter().enumerate() {
        let mut result = Solution::top_k_frequent(nums.clone(), k);

        // The problem states we can return the answer in any order,
        // so we sort both the result and expected arrays before comparing.
        result.sort_unstable();
        let mut expected_sorted = expected.clone();
        expected_sorted.sort_unstable();

        if result == expected_sorted {
            println!("Test Case {}: PASSED", i + 1);
        } else {
            println!("Test Case {}: FAILED", i + 1);
            println!("  Input: nums = {:?}, k = {}", nums, k);
            println!("  Expected: {:?}", expected);
            println!("  Got:      {:?}", result);
            all_passed = false;
        }
    }

    if all_passed {
        println!("\nAll tests passed! Proceeding to benchmark...");

        // Let's create a large test case for performance benchmarking
        let mut large_nums = Vec::with_capacity(1_000_000);
        for i in 0..1_000_000 {
            large_nums.push(i % 1000); // 1000 unique numbers, each repeated 1000 times
        }
        let k = 10;

        println!("\nBenchmarking v1 (Min-Heap O(N log k))...");
        let start_v1 = Instant::now();
        let _ = Solution::top_k_frequent(large_nums.clone(), k);
        let duration_v1 = start_v1.elapsed();
        println!("v1 completed in {:?}", duration_v1);

        println!("\nBenchmarking v2 (Bucket Sort O(N))...");
        let start_v2 = Instant::now();
        let _ = Solution::top_k_frequent_v2(large_nums, k);
        let duration_v2 = start_v2.elapsed();
        println!("v2 completed in {:?}", duration_v2);

        if duration_v2 < duration_v1 {
            println!(
                "\nv2 is {:.2}x faster!",
                duration_v1.as_secs_f64() / duration_v2.as_secs_f64()
            );
        } else {
            println!(
                "\nv1 is {:.2}x faster!",
                duration_v2.as_secs_f64() / duration_v1.as_secs_f64()
            );

            println!("\n--- Insight: Why is v1 faster? ---");
            println!(
                "Wait, why is the theoretically O(N) Bucket Sort slower than the O(N log k) Min-Heap?!"
            );
            println!(
                "The answer is Memory Allocation. In this benchmark, we process 1,000,000 elements."
            );
            println!(
                "For Bucket Sort (v2) to work, it had to initialize a `buckets` array containing 1,000,001 inner Vecs."
            );
            println!(
                "Creating that many empty vectors takes a massive amount of memory allocation time behind the scenes."
            );
            println!(
                "The Min-Heap (v1), on the other hand, only ever allocates memory for exactly `k` (10) items,"
            );
            println!("making it drastically lighter and faster in practice for sparse datasets!");
        }
    }
}

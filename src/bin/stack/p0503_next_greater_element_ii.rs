/*
503. Next Greater Element II

Given a circular integer array nums (i.e., the next element of nums[nums.length - 1] is nums[0]), return the next greater number for every element in nums.

The next greater number of a number x is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, return -1 for this number.

Example 1:
Input: nums = [1,2,1]
Output: [2,-1,2]
Explanation: The first 1's next greater number is 2;
The number 2 can't find next greater number.
The second 1's next greater number needs to search circularly, which is also 2.

Example 2:
Input: nums = [1,2,3,4,3]
Output: [2,3,4,-1,4]

Constraints:
1 <= nums.length <= 10^4
-10^9 <= nums[i] <= 10^9
*/

use std::time::Instant;
// Uncomment this if you want to use your custom allocator!
// use leetcode::TrackingAllocator;

// #[global_allocator]
// static ALLOCATOR: TrackingAllocator = TrackingAllocator::new();

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![-1; nums.len()];
    let mut stack: Vec<usize> = Vec::with_capacity(nums.len());

    for (i, &num) in nums.iter().enumerate() {
        while let Some(&top_idx) = stack.last() {
            if num > nums[top_idx] {
                res[top_idx] = num;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    for (i, &num) in nums.iter().enumerate() {
        while let Some(&top_idx) = stack.last() {
            if num > nums[top_idx] {
                res[top_idx] = num;
                stack.pop();
            } else {
                break;
            }
        }
    }
    res
}

fn main() {
    println!("=== 503. Next Greater Element II ===");

    let tests = vec![
        // Standard examples
        (vec![1, 2, 1], vec![2, -1, 2]),
        (vec![1, 2, 3, 4, 3], vec![2, 3, 4, -1, 4]),
        // Edge cases
        (vec![5, 4, 3, 2, 1], vec![-1, 5, 5, 5, 5]), // Strictly decreasing (wrap around required)
        (vec![1, 1, 1, 1], vec![-1, -1, -1, -1]),    // All identical elements
        (vec![42], vec![-1]),                        // Single element
    ];

    let mut all_passed = true;
    for (i, (nums, expected)) in tests.iter().enumerate() {
        let result = next_greater_elements(nums.clone());
        if result == *expected {
            println!("Test {} PASSED", i + 1);
        } else {
            println!("Test {} FAILED", i + 1);
            println!("  Input:    {:?}", nums);
            println!("  Expected: {:?}", expected);
            println!("  Got:      {:?}", result);
            all_passed = false;
        }
    }

    if all_passed {
        println!("\nAll basic tests PASSED! 🎉");
        println!("\n--- Benchmarking ---");

        let test_input: Vec<i32> = (1..=10_000).rev().collect(); // Worst case: strictly decreasing
        let iterations = 1000;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(next_greater_elements(test_input.clone()));
        }
        let duration = start.elapsed();
        println!(
            "Execution time ({} iterations of 10,000 elements): {:?}",
            iterations, duration
        );
        println!("Average time per run: {:?}", duration / iterations as u32);
    } else {
        println!("\nLet's get the logic working first!");
    }
}

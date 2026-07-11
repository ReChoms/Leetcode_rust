/*
Products of Array Except Self
Medium

Given an integer array nums, return an array output where output[i] is the product of all the elements of nums except nums[i].

Each product is guaranteed to fit in a 32-bit integer.

Follow-up: Could you solve it in O(n) time without using the division operation?

Example 1:
Input: nums = [1,2,4,6]
Output: [48,24,12,8]

Example 2:
Input: nums = [-1,0,1,2,3]
Output: [0,-6,0,0,0]

Constraints:
2 <= nums.length <= 1000
-20 <= nums[i] <= 20
*/

use core::num;

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut left: Vec<i32> = Vec::with_capacity(nums.len());
        let mut right: Vec<i32> = Vec::with_capacity(nums.len());
        let mut res: Vec<i32> = Vec::with_capacity(nums.len());
        left.push(1);
        for i in 1..nums.len() {
            let next_product = left[i - 1] * nums[i - 1];
            left.push(next_product);
        }

        right.push(1);
        for i in (1..nums.len()).rev() {
            let next_product = right.last().unwrap() * nums[i];
            right.push(next_product);
        }
        right.reverse();
        println!("{:?}", right);

        for i in 0..nums.len() {
            res.push(left[i] * right[i]);
        }
        res
    }

    pub fn product_except_self_v2(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        // We only allocate the final result array (O(1) extra space if we don't count the output)
        let mut res = vec![0; n];
        
        // 1. Build the left running product directly into the result array
        res[0] = 1;
        for i in 1..n {
            res[i] = res[i - 1] * nums[i - 1];
        }
        
        // 2. Instead of a 'right' array, we just keep track of the running product in an integer
        let mut right_running_product = 1;
        for i in (0..n).rev() {
            // Multiply the left product (already in res) by the right running product
            res[i] = res[i] * right_running_product;
            // Update the right running product for the next number to the left
            right_running_product *= nums[i];
        }
        
        res
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 4, 6], vec![48, 24, 12, 8]),
        (vec![-1, 0, 1, 2, 3], vec![0, -6, 0, 0, 0]),
        (vec![2, 3], vec![3, 2]),
        (vec![0, 0, 0], vec![0, 0, 0]),
        (vec![2, 2, 2, 2], vec![8, 8, 8, 8]),
    ];

    println!("--- CORRECTNESS TESTING ---");
    for (i, (nums, expected)) in test_cases.into_iter().enumerate() {
        let result_v1 = Solution::product_except_self(nums.clone());
        let result_v2 = Solution::product_except_self_v2(nums.clone());
        if result_v1 == expected && result_v2 == expected {
            println!("Test {} PASSED for both versions", i + 1);
        } else {
            println!("Test {} FAILED", i + 1);
        }
    }

    println!("\n--- BENCHMARKING (100,000 Iterations) ---");
    use std::time::Instant;
    let bench_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Benchmark V1 (O(N) space)
    let start_v1 = Instant::now();
    for _ in 0..100_000 {
        let _ = Solution::product_except_self(bench_data.clone());
    }
    let duration_v1 = start_v1.elapsed();
    println!("Version 1 (O(N) space) Time: {:?}", duration_v1);

    // Benchmark V2 (O(1) space)
    let start_v2 = Instant::now();
    for _ in 0..100_000 {
        let _ = Solution::product_except_self_v2(bench_data.clone());
    }
    let duration_v2 = start_v2.elapsed();
    println!("Version 2 (O(1) space) Time: {:?}", duration_v2);
    
    if duration_v2 < duration_v1 {
        let speedup = duration_v1.as_secs_f64() / duration_v2.as_secs_f64();
        println!("V2 is {:.2}x faster!", speedup);
    }
}

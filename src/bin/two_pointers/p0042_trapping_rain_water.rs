/*
Trapping Rain Water
Hard
Topics
Company Tags
Hints
You are given an array of non-negative integers height which represent an elevation map. Each value height[i] represents the height of a bar, which has a width of 1.

Return the maximum area of water that can be trapped between the bars.

Example 1:
Input: height = [0,2,0,3,1,0,1,3,2,1]
*/

pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_left = height[left];
        let mut max_right = height[right];
        let mut total_water = 0;

        while left < right {
            if max_left < max_right {
                left += 1;
                max_left = max_left.max(height[left]);
                total_water += max_left - height[left];
            } else {
                right -= 1;
                max_right = max_right.max(height[right]);
                total_water += max_right - height[right];
            }
        }

        total_water
    }
}

fn main() {
    println!("--- Trapping Rain Water ---");

    // Standard Example
    let height1 = vec![0, 2, 0, 3, 1, 0, 1, 3, 2, 1];
    let expected1 = 9; // Assuming standard example [0,1,0,2,1,0,1,3,2,1,2,1] is usually 6, wait, the example given is [0,2,0,3,1,0,1,3,2,1].
    // Let's trace it quickly for the test suite:
    // [0, 2, 0, 3, 1, 0, 1, 3, 2, 1]
    // 0: 0
    // 2: 0 (left max 0)
    // 0: min(2, 3) - 0 = 2
    // 3: 0 (left max 2, right max 3) -> 0
    // 1: min(3, 3) - 1 = 2
    // 0: min(3, 3) - 0 = 3
    // 1: min(3, 3) - 1 = 2
    // 3: 0
    // 2: min(3, 1)? wait, right max for 2 is 1. so min(3,1) - 2 = 0.
    // 1: 0
    // Total: 2 + 2 + 3 + 2 = 9. Yes, it's 9.
    
    // Additional Edge Cases
    let height2 = vec![4, 2, 0, 3, 2, 5];
    let expected2 = 9;

    let empty: Vec<i32> = vec![];
    let expected_empty = 0;

    let no_trap = vec![1, 2, 3, 4, 5];
    let expected_no_trap = 0;

    // Run tests
    let tests = vec![
        ("Example 1", height1, expected1),
        ("Example 2 (Leetcode)", height2, expected2),
        ("Empty Array", empty, expected_empty),
        ("Increasing (No trap)", no_trap, expected_no_trap),
    ];

    for (name, input, expected) in tests {
        let result = Solution::trap(input.clone());
        if result == expected {
            println!("✅ {} PASSED", name);
        } else {
            println!("❌ {} FAILED: expected {}, got {}", name, expected, result);
        }
    }
}

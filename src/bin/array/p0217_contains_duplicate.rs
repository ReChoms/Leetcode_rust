/*
Given an integer array nums, return true if any value appears more than once in the array, otherwise return false.

Example 1:
Input: nums = [1, 2, 3, 3]
Output: true

Example 2:
Input: nums = [1, 2, 3, 4]
Output: false

Constraints:
0 <= nums.length <= 10^5
-10^9 <= nums[i] <= 10^9
*/

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hashy_set: HashSet<i32> = HashSet::new();

        for num in nums {
            if hashy_set.contains(&num) {
                return true;
            } else {
                hashy_set.insert(num);
            }
        }

        false
    }
}

fn main() {
    println!("--- Contains Duplicate ---");

    // Standard LeetCode examples
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 3]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);

    // Additional Edge Cases
    assert_eq!(Solution::contains_duplicate(vec![]), false); // empty array
    assert_eq!(Solution::contains_duplicate(vec![1]), false); // single element array
    assert_eq!(Solution::contains_duplicate(vec![1, 1]), true); // only duplicates
    assert_eq!(
        Solution::contains_duplicate(vec![1000000000, -1000000000, 1000000000]),
        true
    ); // extreme constraints

    println!("All standard and edge-case tests passed!");
    println!("Remember to uncomment the tests after you implement your solution.");

    // Space for benchmarking will go here later
}

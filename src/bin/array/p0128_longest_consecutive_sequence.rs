/*
128. Longest Consecutive Sequence
Medium

Given an array of integers nums, return the length of the longest consecutive sequence of elements that can be formed.

A consecutive sequence is a sequence of elements in which each element is exactly 1 greater than the previous element. The elements do not have to be consecutive in the original array.

You must write an algorithm that runs in O(n) time.

Example 1:
Input: nums = [2,20,4,10,3,4,5]
Output: 4
Explanation: The longest consecutive sequence is [2, 3, 4, 5].

Example 2:
Input: nums = [0,3,2,5,4,6,1,1]
Output: 7

Constraints:
0 <= nums.length <= 10^5
-10^9 <= nums[i] <= 10^9
*/

use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut hashy = HashSet::new();
    
    // 1. First pass: Insert all elements into the HashSet
    for &num in &nums {
        hashy.insert(num);
    }

    let mut res = 0;
    // 2. Second pass: Iterate over the array and find sequences
    for num in nums {
        if hashy.contains(&(num - 1)) {
            // num is not start of sequence
            continue;
        } else {
            // num is start of sequence
            let mut count = 0;
            while hashy.contains(&(num + count)) {
                count += 1;
            }
            if count > res {
                res = count;
            }
        }
    }
    res
}

fn main() {
    let test_cases = vec![
        (vec![100, 4, 200, 1, 3, 2], 4),   // standard case (1, 2, 3, 4)
        (vec![0, 3, 2, 5, 4, 6, 1, 1], 7), // with duplicates and zeros
        (vec![], 0),                       // empty array edge case
        (vec![5], 1),                      // single element edge case
        (vec![1, 2, 0, 1], 3),             // duplicates at the start/end
        (vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6], 7), // negatives and multiple sequences
    ];

    let mut all_passed = true;
    for (i, (nums, expected)) in test_cases.into_iter().enumerate() {
        let input_clone = nums.clone();
        let result = longest_consecutive(nums);

        if result == expected {
            println!("✅ Test {} PASSED", i + 1);
        } else {
            println!("❌ Test {} FAILED", i + 1);
            println!("   Input:    {:?}", input_clone);
            println!("   Expected: {}", expected);
            println!("   Got:      {}", result);
            all_passed = false;
        }
    }

    if all_passed {
        println!("\n🎉 All standard tests passed! Ready to benchmark if needed.");
    }
}

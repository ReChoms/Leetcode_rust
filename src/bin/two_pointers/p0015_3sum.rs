/*
15. 3Sum
Medium

Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] where nums[i] + nums[j] + nums[k] == 0, and the indices i, j and k are all distinct.

The output should not contain any duplicate triplets. You may return the output and the triplets in any order.

Example 1:

Input: nums = [-1,0,1,2,-1,-4]
Output: [[-1,-1,2],[-1,0,1]]
Explanation:
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
The distinct triplets are [-1,0,1] and [-1,-1,2].

Example 2:

Input: nums = [0,1,1]
Output: []
Explanation: The only possible triplet does not sum up to 0.

Example 3:

Input: nums = [0,0,0]
Output: [[0,0,0]]
Explanation: The only possible triplet sums up to 0.

Constraints:
3 <= nums.length <= 1000
-10^5 <= nums[i] <= 10^5
*/

use core::num;

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = Vec::new();
        let n = nums.len();
        for i in 0..n {
            let a = nums[i];
            if a > 0 {
                break;
            }
            if i > 0 && a == nums[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                let sum = a + nums[l] + nums[r];
                if sum > 0 {
                    r -= 1;
                } else if sum < 0 {
                    l += 1;
                } else {
                    res.push(vec![a, nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                }
            }
        }
        res
    }
}

fn main() {
    let test_cases = vec![
        (
            vec![-1, 0, 1, 2, -1, -4],
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        ),
        (vec![0, 1, 1], vec![]),
        (vec![0, 0, 0], vec![vec![0, 0, 0]]),
        // Additional edge case: all zeros, but more than 3
        (vec![0, 0, 0, 0], vec![vec![0, 0, 0]]),
        // Additional edge case: no triplets sum to zero
        (vec![1, 2, -2, -1], vec![]),
        // Additional edge case: multiple duplicate elements
        (vec![-2, 0, 0, 2, 2], vec![vec![-2, 0, 2]]),
    ];

    for (nums, expected) in test_cases {
        let mut result = Solution::three_sum(nums.clone());

        // Sorting individual sub-arrays and then the main array
        // to make comparison order-agnostic
        for sub in result.iter_mut() {
            sub.sort();
        }
        result.sort();

        let mut expected_sorted = expected.clone();
        for sub in expected_sorted.iter_mut() {
            sub.sort();
        }
        expected_sorted.sort();

        if result == expected_sorted {
            println!("✅ PASSED for nums: {:?}", nums);
        } else {
            println!("❌ FAILED for nums: {:?}", nums);
            println!("   Expected: {:?}", expected);
            println!("   Got: {:?}", result);
        }
    }
}

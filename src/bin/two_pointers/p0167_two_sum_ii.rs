/*
167. Two Sum II - Input Array Is Sorted

Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.

Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.

The tests are generated such that there is exactly one solution. You may not use the same element twice.

Your solution must use only constant extra space.

Example 1:
Input: numbers = [2,7,11,15], target = 9
Output: [1,2]
Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].

Example 2:
Input: numbers = [2,3,4], target = 6
Output: [1,3]
Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].

Example 3:
Input: numbers = [-1,0], target = -1
Output: [1,2]
Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].

Constraints:
2 <= numbers.length <= 3 * 10^4
-1000 <= numbers[i] <= 1000
numbers is sorted in non-decreasing order.
-1000 <= target <= 1000
The tests are generated such that there is exactly one solution.
*/

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let sum = numbers[left] + numbers[right];

            if sum == target {
                // The problem asks for 1-indexed results
                return vec![(left + 1) as i32, (right + 1) as i32];
            } else if sum < target {
                // We need a larger sum, so we move the left pointer forward
                left += 1;
            } else {
                // We need a smaller sum, so we move the right pointer backward
                right -= 1;
            }
        }

        vec![] // Should never be reached based on problem constraints
    }
}

fn main() {
    let test_cases = vec![
        (vec![2, 7, 11, 15], 9, vec![1, 2]),
        (vec![2, 3, 4], 6, vec![1, 3]),
        (vec![-1, 0], -1, vec![1, 2]),
        (vec![1, 2, 3, 4], 3, vec![1, 2]), // User's example
        // Edge cases
        (vec![-5, -2, 1, 4, 8], 2, vec![2, 4]),
    ];

    for (numbers, target, expected) in test_cases {
        let result = Solution::two_sum(numbers.clone(), target);
        if result == expected {
            println!("✅ PASSED for numbers: {:?}, target: {}", numbers, target);
        } else {
            println!("❌ FAILED for numbers: {:?}, target: {}", numbers, target);
            println!("   Expected: {:?}, Got: {:?}", expected, result);
        }
    }
}

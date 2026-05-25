// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
//
// Q3. Find All Numbers Disappeared in an Array — Easy
//
// Given an array nums of n integers where nums[i] is in the range [1, n],
// return an array of all the integers in the range [1, n] that do not appear
// in nums.
//
// Example 1:
//   Input:  nums = [4,3,2,7,8,2,3,1]
//   Output: [5,6]
//
// Example 2:
//   Input:  nums = [1,1]
//   Output: [2]
//
// Constraints:
//   n == nums.length
//   1 <= n <= 10^5
//   1 <= nums[i] <= n
//
// Follow up: Could you do it without extra space and in O(n) runtime?
// You may assume the returned list does not count as extra space.

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

fn main() {
    println!("{:?}", Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]));
    println!("{:?}", Solution::find_disappeared_numbers(vec![1, 1]));
}

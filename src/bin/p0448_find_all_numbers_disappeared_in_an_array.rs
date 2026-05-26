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
use std::{collections::HashMap, i32};
impl Solution {
    pub fn find_disappeared_numbers_1(nums: Vec<i32>) -> Vec<i32> {
        // Solution: HashMap
        // Time:  O(n)  — ~2n iterations total (one insert pass + one lookup pass)
        // Space: O(n)  — heap-allocated map grows with input
        // Cost per iteration: ~50–150 ns  (~150–300 CPU cycles)
        //   work per step: hash computation, bucket lookup, possible probe/collision walk,
        //   occasional rehash + heap allocation, scattered (cache-unfriendly) memory access
        let mut map: HashMap<i32, i32> = HashMap::new();

        // count occurrences
        for num in &nums {
            *map.entry(*num).or_insert(0) += 1;
        }
        // find missing numbers from 1..n
        let mut result = Vec::new();
        let n = nums.len() as i32;
        for i in 1..=n {
            if !map.contains_key(&i) {
                result.push(i);
            }
        }

        result
    }

    pub fn find_disappeared_numbers_2(mut nums: Vec<i32>) -> Vec<i32> {
        // Solution: in-place sign-flip marking
        // Time:  O(n)  — ~2n iterations total (mark pass + collect pass)
        // Space: O(1)  — mutates the input; output Vec doesn't count
        // Cost per iteration: ~1–3 ns  (~3–6 CPU cycles)
        //   work per step: abs, array index, compare, sign flip — all register-level ops
        //   sequential memory access: CPU prefetcher streams the Vec into cache
        // Net effect: ~20–50x faster per element than the HashMap version, despite same Big-O.
        /*
          ┌──────┬───────────┬─────────────────────┬────────────────────────────────┐
          │ Step │ See value │ Index to mark (v-1) │          Array after           │
          ├──────┼───────────┼─────────────────────┼────────────────────────────────┤
          │ 0    │ 4         │ 3                   │ [4, 3, 2, -7, 8, 2, 3, 1]      │
          ├──────┼───────────┼─────────────────────┼────────────────────────────────┤
          │ 1    │ 3         │ 2                   │ [4, 3, -2, -7, 8, 2, 3, 1]     │
          ├──────┼───────────┼─────────────────────┼────────────────────────────────┤
          │ 2    │ abs(-2)=2 │ 1                   │ [4, -3, -2, -7, 8, 2, 3, 1]    │
          ├──────┼───────────┼─────────────────────┼────────────────────────────────┤
          │ 3    │ abs(-7)=7 │ 6                   │ [4, -3, -2, -7, 8, 2, -3, 1]   │
          ├──────┼───────────┼─────────────────────┼────────────────────────────────┤
          │ 4    │ 8         │ 7                   │ [4, -3, -2, -7, 8, 2, -3, -1]  │
          ├──────┼───────────┼─────────────────────┼────────────────────────────────┤
          │ 5    │ 2         │ 1                   │ (already -3, stays -3)         │
          ├──────┼───────────┼─────────────────────┼────────────────────────────────┤
          │ 6    │ abs(-3)=3 │ 2                   │ (already -2, stays -2)         │
          ├──────┼───────────┼─────────────────────┼────────────────────────────────┤
          │ 7    │ 1         │ 0                   │ [-4, -3, -2, -7, 8, 2, -3, -1] │
          └──────┴───────────┴─────────────────────┴────────────────────────────────┘
        // When 8 and 2 are positive, doenst matter but it matters that index 4 and 5 are positive, which  results in 5 and 6 being the missing numbers. This solution is index orientated, which means you leverage on the numbering arrays have by index..
        */
        for i in 0..nums.len() {
            let num = nums[i];
            let idx = num.abs() as usize - 1;
            if nums[idx] > 0 {
                nums[idx] = -nums[idx];
            }
        }
        let mut result = Vec::new();
        for i in 0..nums.len() {
            if nums[i] > 0 {
                result.push(i as i32 + 1)
            }
        }

        result
    }

    pub fn find_disappeared_numbers_3(mut nums: Vec<i32>) -> Vec<i32> {
        // Solution: cyclic sort (in-place)
        // Time:  O(n)  — each swap places one value permanently in its home slot,
        //                so at most n swaps occur across the whole outer loop.
        // Space: O(1)  — mutates input; output Vec doesn't count.
        // Idea:  Every value v in [1, n] has a "home" slot at index v - 1.
        //        Pass 1 walks the array and swaps each value into its home (where possible).
        //        Pass 2: any slot i where nums[i] != i + 1 reveals that value i + 1
        //        is missing — its home is occupied by a duplicate of another number.
        // Index-oriented like solution 2, but uses positional placement instead of sign flags.
        let mut result = Vec::new();
        let n = nums.len();

        for i in 0..n {
            // Keep swapping until slot i holds its correct value (i + 1),
            // OR the target home already holds the right value (duplicate — stop to avoid infinite loop).
            while nums[i] != (i as i32) + 1 && nums[nums[i] as usize - 1] != nums[i] {
                let home = nums[i] as usize - 1;
                nums.swap(i, home);
            }
        }
        for i in 0..n {
            if nums[i] != (i as i32) + 1 {
                result.push(i as i32 + 1)
            }
        }
        result
    }

    pub fn find_disappeared_numbers_4(mut nums: Vec<i32>) -> Vec<i32> {
        // Solution: in-place "add n" marking (modulo trick)
        // Time:  O(n) — two passes
        // Space: O(1) — mutates input
        let n = nums.len() as i32;

        // Pass 1: for each value, mark its home slot by adding n
        for i in 0..nums.len() {
            let idx = ((nums[i] - 1) % n) as usize; // recover original even if already marked
            nums[idx] += n;
        }

        // Pass 2: any slot still ≤ n was never marked → its index+1 is missing
        let mut result = Vec::new();
        for i in 0..nums.len() {
            if nums[i] <= n {
                result.push(i as i32 + 1);
            }
        }
        result
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::find_disappeared_numbers_1(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
    println!("{:?}", Solution::find_disappeared_numbers_1(vec![1, 1]));
    println!(
        "{:?}",
        Solution::find_disappeared_numbers_2(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
    println!("{:?}", Solution::find_disappeared_numbers_2(vec![1, 1]));
    println!(
        "{:?}",
        Solution::find_disappeared_numbers_3(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
    println!("{:?}", Solution::find_disappeared_numbers_3(vec![1, 1]));
    println!(
        "{:?}",
        Solution::find_disappeared_numbers_4(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
    println!("{:?}", Solution::find_disappeared_numbers_4(vec![1, 1]));
}

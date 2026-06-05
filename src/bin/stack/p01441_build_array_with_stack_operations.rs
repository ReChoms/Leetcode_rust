/*
LeetCode 1441 - Build an Array With Stack Operations

Observation:
Every number from the stream must be read in order.
For numbers that belong to target, perform:
    Push
For numbers that do not belong to target, perform:
    Push, Pop

Since target is strictly increasing, a single pass with a pointer
into target is sufficient.

Complexity:
- Time: O(m), where m = target[target.len() - 1]
- Space: O(k), where k is the number of operations returned.

Example 1:

Input: target = [1,3], n = 3
Output: ["Push","Push","Pop","Push"]
Explanation: Initially the stack s is empty. The last element is the top of the stack.
Read 1 from the stream and push it to the stack. s = [1].
Read 2 from the stream and push it to the stack. s = [1,2].
Pop the integer on the top of the stack. s = [1].
Read 3 from the stream and push it to the stack. s = [1,3].
Example 2:

Input: target = [1,2,3], n = 3
Output: ["Push","Push","Push"]
Explanation: Initially the stack s is empty. The last element is the top of the stack.
Read 1 from the stream and push it to the stack. s = [1].
Read 2 from the stream and push it to the stack. s = [1,2].
Read 3 from the stream and push it to the stack. s = [1,2,3].
Example 3:

Input: target = [1,2], n = 4
Output: ["Push","Push"]
Explanation: Initially the stack s is empty. The last element is the top of the stack.
Read 1 from the stream and push it to the stack. s = [1].
Read 2 from the stream and push it to the stack. s = [1,2].
Since the stack (from the bottom to the top) is equal to target, we stop the stack operations.
The answers that read integer 3 from the stream are not accepted.


Constraints:

1 <= target.length <= 100
1 <= n <= 100
1 <= target[i] <= n
target is strictly increasing.
*/

struct Solution;
use std::{collections::HashMap, i32, process::exit};
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ops: Vec<String> = Vec::new();
        let mut idx: i32 = 0;
        // n cant be bigger than last number in target, without these rest of code make out of bounds error
        let mut nn = target[target.len() - 1];
        for n in 1..nn + 1 {
            // always push, variable is remove
            ops.push("Push".to_string());
            if n == target[idx as usize] {
                // count upwards only if target value was found
                idx += 1;
            } else {
                //not in target value to pop again
                ops.push("Pop".to_string());
            }
        }
        ops
    }
    // costs more memory because of pre allocation but only for small data
    pub fn build_array_extra(target: Vec<i32>, _n: i32) -> Vec<String> {
        // get the last element of target to know where to stop
        let nn = target[target.len() - 1] as usize;
        // calculate exact operations capacity to save memory and avoid resizing
        let capacity = 2 * nn - target.len();
        let mut ops = Vec::with_capacity(capacity);

        // start stream pointer at 1
        let mut curr = 1;
        for tar in target {
            // push and pop for all elements we skipped
            for _ in curr..tar {
                ops.push("Push".to_string());
                ops.push("Pop".to_string());
            }
            // push the matched target element
            ops.push("Push".to_string());
            // advance stream pointer beyond the match
            curr = tar + 1;
        }
        ops
    }
}

fn main() {
    println!("Running tests for LeetCode 1441 - Build an Array With Stack Operations...\n");

    println!("=== 1. Correctness Verification ===");
    
    // Example 1: target = [1,3], n = 3
    let target1 = vec![1, 3];
    let n1 = 3;
    let expected1 = vec!["Push", "Push", "Pop", "Push"];
    let result1 = Solution::build_array(target1.clone(), n1);
    let result1_extra = Solution::build_array_extra(target1.clone(), n1);
    
    println!("Example 1: target = {:?}, n = {}", target1, n1);
    println!("  Expected: {:?}", expected1);
    println!("  Original Output:      {:?} -> {}", result1, if result1 == expected1 { "PASSED" } else { "FAILED" });
    println!("  Optimized Output:     {:?} -> {}\n", result1_extra, if result1_extra == expected1 { "PASSED" } else { "FAILED" });

    // Example 2: target = [1,2,3], n = 3
    let target2 = vec![1, 2, 3];
    let n2 = 3;
    let expected2 = vec!["Push", "Push", "Push"];
    let result2 = Solution::build_array(target2.clone(), n2);
    let result2_extra = Solution::build_array_extra(target2.clone(), n2);
    
    println!("Example 2: target = {:?}, n = {}", target2, n2);
    println!("  Expected: {:?}", expected2);
    println!("  Original Output:      {:?} -> {}", result2, if result2 == expected2 { "PASSED" } else { "FAILED" });
    println!("  Optimized Output:     {:?} -> {}\n", result2_extra, if result2_extra == expected2 { "PASSED" } else { "FAILED" });

    // Example 3: target = [1,2], n = 4
    let target3 = vec![1, 2];
    let n3 = 4;
    let expected3 = vec!["Push", "Push"];
    let result3 = Solution::build_array(target3.clone(), n3);
    let result3_extra = Solution::build_array_extra(target3.clone(), n3);
    
    println!("Example 3: target = {:?}, n = {}", target3, n3);
    println!("  Expected: {:?}", expected3);
    println!("  Original Output:      {:?} -> {}", result3, if result3 == expected3 { "PASSED" } else { "FAILED" });
    println!("  Optimized Output:     {:?} -> {}\n", result3_extra, if result3_extra == expected3 { "PASSED" } else { "FAILED" });

    println!("=== 2. Performance & Memory Benchmark ===");
    
    // Create a large target to make differences visible (odd numbers up to 100)
    let target_bench = (1..100).filter(|x| x % 2 != 0).collect::<Vec<i32>>(); // size = 50, last = 99
    let n_bench = 100;

    // A. Memory: Compare Allocated Capacity
    let res_orig = Solution::build_array(target_bench.clone(), n_bench);
    let res_extra = Solution::build_array_extra(target_bench.clone(), n_bench);
    
    println!("Vector Memory Allocation (for target size {}):", target_bench.len());
    println!("  Original build_array Vector Capacity:       {} elements", res_orig.capacity());
    println!("  Optimized build_array_extra Vector Capacity: {} elements", res_extra.capacity());
    println!("  (The optimized version allocates exactly what it needs with zero wasted space!)\n");

    // B. Performance: Compare Speed over 100,000 Iterations
    let iterations = 100_000;
    println!("Running both methods {} times to measure execution speed...", iterations);

    // Benchmark Original
    let start_orig = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = Solution::build_array(target_bench.clone(), n_bench);
    }
    let duration_orig = start_orig.elapsed();

    // Benchmark Optimized
    let start_extra = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = Solution::build_array_extra(target_bench.clone(), n_bench);
    }
    let duration_extra = start_extra.elapsed();

    println!("Speed Results:");
    println!("  Original build_array:       {:?}", duration_orig);
    println!("  Optimized build_array_extra: {:?}", duration_extra);
    
    let speedup = (duration_orig.as_nanos() as f64) / (duration_extra.as_nanos() as f64);
    println!("  Optimized version is {:.2}x faster!\n", speedup);
}

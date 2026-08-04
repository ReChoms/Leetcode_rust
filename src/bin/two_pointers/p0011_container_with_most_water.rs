/*
11. Container With Most Water
Medium

You are given an integer array heights where heights[i] represents the height of the ith bar.

You may choose any two bars to form a container. Return the maximum amount of water a container can store.

Example 1:
Input: height = [1,7,2,5,4,7,3,6]
Output: 36

Example 2:
Input: height = [2,2,2]
Output: 4

Constraints:
2 <= height.length <= 100,000
0 <= height[i] <= 10,000
*/

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut return_area = 0;
        while l < r {
            let mut area = 0;
            if height[l] > height[r] {
                area = height[r] * (r - l) as i32;
                r -= 1;
            } else {
                area = height[l] * (r - l) as i32;
                l += 1;
            }
            if area > return_area {
                return_area = area;
            }
        }
        return_area
    }
    pub fn max_area_v2(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max_area = 0;

        while l < r {
            let h_l = height[l];
            let h_r = height[r];

            // 1. Idiomatic Rust: Use built-in .min() and .max() instead of if/else logic
            let current_area = h_l.min(h_r) * (r - l) as i32;
            max_area = max_area.max(current_area);

            // 2. Optimization: If we move our pointer, we can safely skip any inner lines 
            // that are shorter than or equal to our current line. A shorter line with a 
            // smaller width can NEVER produce a larger area, so we skip calculating it entirely.
            if h_l < h_r {
                l += 1;
                while l < r && height[l] <= h_l {
                    l += 1;
                }
            } else {
                r -= 1;
                while l < r && height[r] <= h_r {
                    r -= 1;
                }
            }
        }
        max_area
    }
}
fn main() {
    let test_cases = vec![
        (vec![1, 7, 2, 5, 4, 7, 3, 6], 36),
        (vec![2, 2, 2], 4),
        // Additional edge cases
        (vec![1, 1], 1),
        (vec![4, 3, 2, 1, 4], 16),
        (vec![1, 2, 1], 2),
    ];

    println!("--- Correctness Tests ---");
    for (i, (height, expected)) in test_cases.iter().enumerate() {
        let result_v1 = Solution::max_area(height.clone());
        let result_v2 = Solution::max_area_v2(height.clone());
        
        if result_v1 == *expected && result_v2 == *expected {
            println!("Test case {} passed for both versions", i + 1);
        } else {
            println!("Test case {} FAILED!", i + 1);
        }
    }

    println!("\n--- Benchmarking ---");
    // Create a very large synthetic array to see the optimization difference
    let mut large_input = vec![0; 100_000];
    for i in 0..50_000 {
        large_input[i] = i as i32;
        large_input[99_999 - i] = i as i32;
    }
    
    let runs = 1_000;
    
    let start_v1 = std::time::Instant::now();
    for _ in 0..runs {
        Solution::max_area(large_input.clone());
    }
    let duration_v1 = start_v1.elapsed();
    println!("v1 (Original) time: {:?}", duration_v1);

    let start_v2 = std::time::Instant::now();
    for _ in 0..runs {
        Solution::max_area_v2(large_input.clone());
    }
    let duration_v2 = start_v2.elapsed();
    println!("v2 (Optimized) time: {:?}", duration_v2);
    
    if duration_v2 < duration_v1 {
        let speedup = duration_v1.as_secs_f64() / duration_v2.as_secs_f64();
        println!("v2 is {:.2}x faster!", speedup);
    }
}

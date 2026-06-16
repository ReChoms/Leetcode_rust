/*
Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.

Example 1:
Input: heights = [2,1,5,6,2,3]
Output: 10
Explanation: The above is a histogram where width of each bar is 1.
The largest rectangle is shown in the red area, which has an area = 10 units.

Example 2:
Input: heights = [2,4]
Output: 4

Constraints:
1 <= heights.length <= 10^5
0 <= heights[i] <= 10^4
*/

pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(usize, i32)> = Vec::with_capacity(heights.len());
        let mut max_area: i32 = 0;

        for (idx, height) in heights.iter().enumerate() {
            let mut start_idx = idx;
            while let Some(&(_top_idx, top_height)) = stack.last() {
                if top_height > *height {
                    // We found a taller bar on the stack!
                    // 1. Pop it.
                    // 2. Calculate its area.
                    // 3. Update max_area.

                    let (popped_idx, popped_height) = stack.pop().unwrap();
                    start_idx = popped_idx;
                    let width = (idx - popped_idx) as i32;
                    let area = popped_height * width;
                    max_area = max_area.max(area);
                } else {
                    // The bar at the top of the stack is shorter or equal, so we can stop popping.
                    break;
                }
            }
            stack.push((start_idx, *height));
        }
        while let Some((popped_idx, popped_height)) = stack.pop() {
            // These bars extend all the way to the end of the array.
            // So the width is: (total length of array - popped_idx)
            let width = (heights.len() - popped_idx) as i32;
            let area = popped_height * width;
            max_area = max_area.max(area);
        }

        max_area
    }
}

fn main() {
    println!("--- Testing Largest Rectangle in Histogram ---");

    let test_cases = vec![
        // Standard examples
        (vec![2, 1, 5, 6, 2, 3], 10),
        (vec![2, 4], 4),
        // Edge cases
        (vec![], 0),              // Empty histogram
        (vec![5], 5),             // Single bar
        (vec![3, 3, 3, 3], 12),   // All bars same height
        (vec![1, 2, 3, 4, 5], 9), // Increasing order (rect formed by 3,4,5 is 3*3=9)
        (vec![5, 4, 3, 2, 1], 9), // Decreasing order (rect formed by 5,4,3 is 3*3=9)
        (vec![2, 1, 2], 3),       // Valley
        (vec![1, 2, 2], 4),       // Duplicate values
    ];

    let mut all_passed = true;

    for (i, (heights, expected)) in test_cases.into_iter().enumerate() {
        let result = Solution::largest_rectangle_area(heights.clone());

        if result == expected {
            println!("Test Case {} PASSED", i + 1);
        } else {
            println!(
                "Test Case {} FAILED: Expected {}, got {} for input {:?}",
                i + 1,
                expected,
                result,
                heights
            );
            all_passed = false;
        }
    }

    if all_passed {
        println!("All tests PASSED! Great job.");
    } else {
        println!("Some tests FAILED. Let's fix them!");
    }

    // --- Performance Benchmarking ---
    println!("\n--- Performance Benchmarking ---");
    
    let huge_input: Vec<i32> = (1..=100_000).collect();
    let start_time = std::time::Instant::now();
    let iterations = 100;
    
    for _ in 0..iterations {
        std::hint::black_box(Solution::largest_rectangle_area(huge_input.clone()));
    }
    
    let duration = start_time.elapsed();
    println!("Processed 100,000 elements {} times in: {:?}", iterations, duration);
    println!("Average time per execution: {:?}", duration / iterations);
}

/*
739. Daily Temperatures

Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.

Example 1:
Input: temperatures = [73,74,75,71,69,72,76,73]
Output: [1,1,4,2,1,1,0,0]

Example 2:
Input: temperatures = [30,40,50,60]
Output: [1,1,1,0]

Example 3:
Input: temperatures = [30,60,90]
Output: [1,1,0]

Constraints:
1 <= temperatures.length <= 10^5
30 <= temperatures[i] <= 100
*/

use leetcode::TrackingAllocator;

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator::new();

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; temperatures.len()];
    let mut stack: Vec<usize> = Vec::new();
    for (idx, temp) in temperatures.iter().enumerate() {
        while let Some(&top_index) = stack.last() {
            if *temp > temperatures[top_index] {
                // unwrap() is safe here because the `while let Some(...)` condition
                // guarantees that the stack is not empty before we enter this block.
                let days_diff = idx - stack.pop().unwrap();
                res[top_index] = days_diff as i32;
            } else {
                break;
            }
        }
        stack.push(idx);
    }
    res
}

pub fn daily_temperatures_v2(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; temperatures.len()];

    // ENHANCEMENT 1: Pre-allocate capacity to prevent memory reallocation overhead
    let mut stack: Vec<usize> = Vec::with_capacity(temperatures.len());

    for (idx, temp) in temperatures.iter().enumerate() {
        while let Some(&top_index) = stack.last() {
            if *temp > temperatures[top_index] {
                // ENHANCEMENT 2: Safe pop using `if let` to entirely avoid `.unwrap()`.
                // This makes the code immune to future refactoring bugs.
                if let Some(popped_idx) = stack.pop() {
                    res[popped_idx] = (idx - popped_idx) as i32;
                }
            } else {
                break;
            }
        }
        stack.push(idx);
    }
    res
}

fn main() {
    println!("=== 739. Daily Temperatures ===");

    let tests = vec![
        (
            vec![73, 74, 75, 71, 69, 72, 76, 73],
            vec![1, 1, 4, 2, 1, 1, 0, 0],
        ),
        (vec![30, 40, 50, 60], vec![1, 1, 1, 0]),
        (vec![30, 60, 90], vec![1, 1, 0]),
    ];

    let mut all_passed = true;
    for (i, (temps, expected)) in tests.iter().enumerate() {
        let result_v1 = daily_temperatures(temps.clone());
        let result_v2 = daily_temperatures_v2(temps.clone());
        if result_v1 == *expected && result_v2 == *expected {
            println!("Test {} PASSED (v1 & v2)", i + 1);
        } else {
            println!("Test {} FAILED", i + 1);
            println!("  Input:    {:?}", temps);
            println!("  Expected: {:?}", expected);
            println!("  Got (v1): {:?}", result_v1);
            println!("  Got (v2): {:?}", result_v2);
            all_passed = false;
        }
    }

    if all_passed {
        println!("\nAll basic tests PASSED! 🎉");
        println!("\n--- Benchmarking ---");

        let test_input = vec![73, 74, 75, 71, 69, 72, 76, 73];

        // True Memory Allocation tracking using our custom global allocator
        ALLOCATOR.reset();
        let _ = daily_temperatures(test_input.clone());
        println!(
            "Heap Bytes Allocated (v1): {} bytes",
            ALLOCATOR.get_allocated_bytes()
        );

        ALLOCATOR.reset();
        let _ = daily_temperatures_v2(test_input.clone());
        println!(
            "Heap Bytes Allocated (v2): {} bytes",
            ALLOCATOR.get_allocated_bytes()
        );

        let iterations = 100_000;

        // Speed benchmark v1
        let start_v1 = Instant::now();
        for _ in 0..iterations {
            let _ = daily_temperatures(test_input.clone());
        }
        let duration_v1 = start_v1.elapsed();
        println!(
            "Execution time v1 ({} iterations): {:?}",
            iterations, duration_v1
        );

        // Speed benchmark v2
        let start_v2 = Instant::now();
        for _ in 0..iterations {
            let _ = daily_temperatures_v2(test_input.clone());
        }
        let duration_v2 = start_v2.elapsed();
        println!(
            "Execution time v2 ({} iterations): {:?}",
            iterations, duration_v2
        );
    } else {
        println!("\nLet's get the logic working first!");
    }
}

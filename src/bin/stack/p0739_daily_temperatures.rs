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

use std::time::Instant;

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    // TODO: We'll implement this together!
    vec![0; temperatures.len()]
}

fn main() {
    println!("=== 739. Daily Temperatures ===");

    let tests = vec![
        (vec![73, 74, 75, 71, 69, 72, 76, 73], vec![1, 1, 4, 2, 1, 1, 0, 0]),
        (vec![30, 40, 50, 60], vec![1, 1, 1, 0]),
        (vec![30, 60, 90], vec![1, 1, 0]),
    ];

    let mut all_passed = true;
    for (i, (temps, expected)) in tests.iter().enumerate() {
        let result = daily_temperatures(temps.clone());
        if result == *expected {
            println!("Test {} PASSED", i + 1);
        } else {
            println!("Test {} FAILED", i + 1);
            println!("  Input:    {:?}", temps);
            println!("  Expected: {:?}", expected);
            println!("  Got:      {:?}", result);
            all_passed = false;
        }
    }

    if all_passed {
        println!("\nAll basic tests PASSED! 🎉");
        println!("\n--- Benchmarking ---");

        let test_input = vec![73, 74, 75, 71, 69, 72, 76, 73];

        // Memory check for capacity
        let result = daily_temperatures(test_input.clone());
        println!(
            "Memory Allocation: Vector length = {}, capacity = {}",
            result.len(),
            result.capacity()
        );

        // Speed benchmark
        let start = Instant::now();
        let iterations = 100_000;
        for _ in 0..iterations {
            let _ = daily_temperatures(test_input.clone());
        }
        let duration = start.elapsed();
        println!("Execution time ({} iterations): {:?}", iterations, duration);
    } else {
        println!("\nLet's get the logic working first!");
    }
}

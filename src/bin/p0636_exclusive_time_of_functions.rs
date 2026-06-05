/*
LeetCode 636 - Exclusive Time of Functions
Medium

On a single-threaded CPU, we execute a program containing n functions. Each function has a unique ID between 0 and n - 1.

Function calls are stored in a call stack: when a function call starts, its ID is pushed onto the stack, and when a function call ends, its ID is popped off the stack. The function whose ID is at the top of the stack is the current function being executed. Each time a function starts or ends, we write a log with the ID, whether it started or ended, and the timestamp.

You are given a list logs, where logs[i] represents the ith log message formatted as a string "{function_id}:{"start" | "end"}:{timestamp}". For example, "0:start:3" means a function call with function ID 0 started at the beginning of timestamp 3, and "1:end:2" means a function call with function ID 1 ended at the end of timestamp 2. Note that a function can be called multiple times, possibly recursively.

A function's exclusive time is the sum of execution times for all function calls in the program. For example, if a function is called twice, one call executing for 2 time units and another call executing for 1 time unit, the exclusive time is 2 + 1 = 3.

Return the exclusive time of each function in an array, where the value at the ith index represents the exclusive time for the function with ID i.

Example 1:
Input: n = 2, logs = ["0:start:0","1:start:2","1:end:5","0:end:6"]
Output: [3,4]

Example 2:
Input: n = 1, logs = ["0:start:0","0:start:2","0:end:5","0:start:6","0:end:6","0:end:7"]
Output: [8]

Example 3:
Input: n = 2, logs = ["0:start:0","0:start:2","0:end:5","1:start:6","1:end:6","0:end:7"]
Output: [7,1]

Constraints:
1 <= n <= 100
2 <= logs.length <= 500
0 <= function_id < n
0 <= timestamp <= 10^9
No two start events will happen at the same timestamp.
No two end events will happen at the same timestamp.
Each function has an "end" log for each "start" log.
*/

struct Solution;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut result = vec![0; n as usize];
        let mut stack: Vec<(usize, i32)> = Vec::new();

        for log in logs {
            let parts: Vec<&str> = log.split(':').collect();
            let function_id: usize = parts[0].parse().unwrap();
            let action = parts[1];
            let timestamp: i32 = parts[2].parse().unwrap();

            if action == "start" {
                // 1. A new function started! Just push it onto the stack.
                stack.push((function_id, timestamp));
            } else {
                // 1. A function ended! Pop the most recent one off the stack.
                let (popped_id, start_time) = stack.pop().unwrap();

                // 2. Calculate exactly how long it took
                let execution_time = timestamp - start_time + 1;

                // 3. Add that time to its score in the result array
                result[popped_id] += execution_time;

                // 4.  Was someone paused?
                // If the stack is NOT empty, it means the function currently sitting
                // on top of the stack was waiting for us to finish.
                if let Some(&(parent_id, _parent_start_time)) = stack.last() {
                    // Subtract our execution time from the paused parent!
                    result[parent_id] -= execution_time;
                }
            }
        }

        result
    }

    pub fn exclusive_time_v2(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut result = vec![0; n as usize];
        let mut stack: Vec<(usize, i32)> = Vec::new();

        for log in logs {
            // =========================================================================
            // NASA Optimization: Zero-Allocation String Parsing (Iterators vs Vectors)
            // =========================================================================
            // Beginner Rust: `.split(':').collect::<Vec<&str>>()`
            //   - Pauses the program to ask the OS for physical RAM (Heap Allocation).
            //   - Builds an entire Vector structure just to hold 3 items, then deletes it.
            //   - Very slow (costs ~100-200 nanoseconds per log).
            //
            // Advanced Rust: `log.split(':')` + `.next()`
            //   - Bypasses the OS entirely. No Heap Memory is allocated.
            //   - Returns a lightweight `Split` struct (an Iterator) on the CPU Stack.
            //   - The Iterator acts as a "bookmark", just pointing to the existing text.
            //   - Moving the bookmark with `.next()` takes ~1 nanosecond.
            // =========================================================================
            let mut parts = log.split(':');
            let function_id: usize = parts.next().unwrap().parse().unwrap();
            let action = parts.next().unwrap();
            let timestamp: i32 = parts.next().unwrap().parse().unwrap();

            if action == "start" {
                stack.push((function_id, timestamp));
            } else {
                let (popped_id, start_time) = stack.pop().unwrap();
                let execution_time = timestamp - start_time + 1;

                result[popped_id] += execution_time;

                // The Magic Step: If the stack isn't empty, someone was paused!
                // We use `if let` to safely check the top of the stack.
                // The `_` ignores the timestamp because we only need the parent's ID.
                if let Some(&(parent_id, _)) = stack.last() {
                    result[parent_id] -= execution_time;
                }
            }
        }

        result
    }
}

fn main() {
    println!("Running tests for Exclusive Time of Functions...\n");

    let to_vec =
        |tokens: &[&str]| -> Vec<String> { tokens.iter().map(|s| s.to_string()).collect() };

    // Example 1
    let n1 = 2;
    let logs1 = to_vec(&["0:start:0", "1:start:2", "1:end:5", "0:end:6"]);
    let expected1 = vec![3, 4];
    let result1 = Solution::exclusive_time(n1, logs1.clone());
    println!("Example 1:");
    println!("  Expected: {:?}", expected1);
    println!(
        "  Output:   {:?} -> {}\n",
        result1,
        if result1 == expected1 {
            "PASSED"
        } else {
            "FAILED"
        }
    );

    // Example 2
    let n2 = 1;
    let logs2 = to_vec(&[
        "0:start:0",
        "0:start:2",
        "0:end:5",
        "0:start:6",
        "0:end:6",
        "0:end:7",
    ]);
    let expected2 = vec![8];
    let result2 = Solution::exclusive_time(n2, logs2.clone());
    println!("Example 2:");
    println!("  Expected: {:?}", expected2);
    println!(
        "  Output:   {:?} -> {}\n",
        result2,
        if result2 == expected2 {
            "PASSED"
        } else {
            "FAILED"
        }
    );

    // Example 3
    let n3 = 2;
    let logs3 = to_vec(&[
        "0:start:0",
        "0:start:2",
        "0:end:5",
        "1:start:6",
        "1:end:6",
        "0:end:7",
    ]);
    let expected3 = vec![7, 1];
    let result3 = Solution::exclusive_time(n3, logs3.clone());
    println!("Example 3:");
    println!("  Expected: {:?}", expected3);
    println!(
        "  Output:   {:?} -> {}\n",
        result3,
        if result3 == expected3 {
            "PASSED"
        } else {
            "FAILED"
        }
    );

    // --- BENCHMARKING ---
    println!("--------------------------------------------------");
    println!("Running Benchmarks (100,000 iterations)...");

    let iterations = 100_000;
    
    // Benchmark Version 1
    let start_bench_v1 = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = Solution::exclusive_time(n3, logs3.clone());
    }
    let duration_v1 = start_bench_v1.elapsed();
    println!("Solution 1 (Standard) Execution Time: {:?}", duration_v1);

    // Benchmark Version 2 (NASA)
    let start_bench_v2 = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = Solution::exclusive_time_v2(n3, logs3.clone());
    }
    let duration_v2 = start_bench_v2.elapsed();
    println!("Solution 2 (NASA) Execution Time:     {:?}", duration_v2);
    
    let multiplier = duration_v1.as_secs_f64() / duration_v2.as_secs_f64();
    println!("NASA Solution is {:.2}x faster!", multiplier);
}

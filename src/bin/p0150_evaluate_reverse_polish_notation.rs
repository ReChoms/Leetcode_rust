/*
LeetCode 150 - Evaluate Reverse Polish Notation
Medium

Topics: Array, Math, Stack

You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.

Evaluate the expression. Return an integer that represents the value of the expression.

Note that:
- The valid operators are '+', '-', '*', and '/'.
- Each operand may be an integer or another expression.
- The division between two integers always truncates toward zero.
- There will not be any division by zero.
- The input represents a valid arithmetic expression in a reverse polish notation.
- The answer and all the intermediate calculations can be represented in a 32-bit integer.

Example 1:
Input: tokens = ["2","1","+","3","*"]
Output: 9
Explanation: ((2 + 1) * 3) = 9

Example 2:
Input: tokens = ["4","13","5","/","+"]
Output: 6
Explanation: (4 + (13 / 5)) = 6

Example 3:
Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
Output: 22
Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
= ((10 * (6 / (12 * -11))) + 17) + 5
= ((10 * (6 / -132)) + 17) + 5
= ((10 * 0) + 17) + 5
= (0 + 17) + 5
= 17 + 5
= 22

Constraints:
- 1 <= tokens.length <= 10^4
- tokens[i] is either an operator: "+", "-", "*", or "/", or an integer in the range [-200, 200].
*/

struct Solution;

impl Solution {
    /// The standard, readable implementation of Reverse Polish Notation.
    /// Good for learning and interviews.
    pub fn eval_rpn_standard(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(tokens.len());

        for x in tokens {
            match x.as_str() {
                "+" => {
                    let right_num = stack.pop().unwrap();
                    let left_num = stack.pop().unwrap();
                    stack.push(left_num + right_num);
                }
                "-" => {
                    let right_num = stack.pop().unwrap();
                    let left_num = stack.pop().unwrap();
                    stack.push(left_num - right_num);
                }
                "*" => {
                    let right_num = stack.pop().unwrap();
                    let left_num = stack.pop().unwrap();
                    stack.push(left_num * right_num);
                }
                "/" => {
                    let right_num = stack.pop().unwrap();
                    let left_num = stack.pop().unwrap();
                    stack.push(left_num / right_num);
                }
                _ => {
                    let number: i32 = x.parse().unwrap();
                    stack.push(number);
                }
            }
        }

        stack.pop().unwrap()
    }

    /// The highly-optimized implementation.
    /// Uses mathematically perfect memory allocation and pointer mutation.
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(tokens.len() / 2 + 1);

        for x in tokens {
            match x.as_str() {
                "+" => {
                    let right_num = stack.pop().unwrap();
                    *stack.last_mut().unwrap() += right_num;
                }
                "-" => {
                    let right_num = stack.pop().unwrap();
                    *stack.last_mut().unwrap() -= right_num;
                }
                "*" => {
                    let right_num = stack.pop().unwrap();
                    *stack.last_mut().unwrap() *= right_num;
                }
                "/" => {
                    let right_num = stack.pop().unwrap();
                    *stack.last_mut().unwrap() /= right_num;
                }
                _ => {
                    stack.push(x.parse().unwrap());
                }
            }
        }

        stack.pop().unwrap()
    }

    /// The production-safe, optimized implementation.
    /// Uses safe error handling instead of unwrap() to prevent crashes on invalid input.
    pub fn eval_rpn_safe(tokens: Vec<String>) -> Result<i32, String> {
        let mut stack: Vec<i32> = Vec::with_capacity(tokens.len() / 2 + 1);

        for x in tokens {
            match x.as_str() {
                "+" => {
                    if let (Some(right_num), Some(left_mut)) = (stack.pop(), stack.last_mut()) {
                        *left_mut += right_num;
                    } else {
                        return Err("Invalid expression: not enough numbers for '+'".to_string());
                    }
                }
                "-" => {
                    if let (Some(right_num), Some(left_mut)) = (stack.pop(), stack.last_mut()) {
                        *left_mut -= right_num;
                    } else {
                        return Err("Invalid expression: not enough numbers for '-'".to_string());
                    }
                }
                "*" => {
                    if let (Some(right_num), Some(left_mut)) = (stack.pop(), stack.last_mut()) {
                        *left_mut *= right_num;
                    } else {
                        return Err("Invalid expression: not enough numbers for '*'".to_string());
                    }
                }
                "/" => {
                    if let (Some(right_num), Some(left_mut)) = (stack.pop(), stack.last_mut()) {
                        if right_num == 0 {
                            return Err("Division by zero".to_string());
                        }
                        *left_mut /= right_num;
                    } else {
                        return Err("Invalid expression: not enough numbers for '/'".to_string());
                    }
                }
                _ => match x.parse::<i32>() {
                    Ok(num) => stack.push(num),
                    Err(_) => return Err(format!("Invalid token: '{}'", x)),
                },
            }
        }

        if stack.len() == 1 {
            Ok(stack.pop().unwrap())
        } else {
            Err("Invalid expression: leftover numbers on stack".to_string())
        }
    }
}

fn main() {
    println!("Running tests for LeetCode 150 - Evaluate Reverse Polish Notation...\n");

    let to_vec =
        |tokens: &[&str]| -> Vec<String> { tokens.iter().map(|s| s.to_string()).collect() };

    // Example 1
    let tokens1 = to_vec(&["2", "1", "+", "3", "*"]);
    let expected1 = 9;
    let result1 = Solution::eval_rpn(tokens1.clone());
    println!("Example 1: tokens = {:?}", tokens1);
    println!("  Expected: {}", expected1);
    println!(
        "  Output:   {} -> {}\n",
        result1,
        if result1 == expected1 {
            "PASSED"
        } else {
            "FAILED"
        }
    );

    // Example 2
    let tokens2 = to_vec(&["4", "13", "5", "/", "+"]);
    let expected2 = 6;
    let result2 = Solution::eval_rpn(tokens2.clone());
    println!("Example 2: tokens = {:?}", tokens2);
    println!("  Expected: {}", expected2);
    println!(
        "  Output:   {} -> {}\n",
        result2,
        if result2 == expected2 {
            "PASSED"
        } else {
            "FAILED"
        }
    );

    // Example 3
    let tokens3 = to_vec(&[
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ]);
    let expected3 = 22;
    let result3 = Solution::eval_rpn(tokens3.clone());
    println!("Example 3: tokens = {:?}", tokens3);
    println!("  Expected: {}", expected3);
    println!(
        "  Output:   {} -> {}\n",
        result3,
        if result3 == expected3 {
            "PASSED"
        } else {
            "FAILED"
        }
    );

    // --- BENCHMARKING
    // println!("--------------------------------------------------");

    // --- BENCHMARKING ---
    println!("--------------------------------------------------");

    println!("Running Benchmarks (1,000,000 iterations)...");

    let iterations = 1_000_000;

    // Benchmark Standard
    let start_std = std::time::Instant::now();
    for _ in 0..iterations {
        Solution::eval_rpn_standard(tokens3.clone());
    }
    let duration_std = start_std.elapsed();
    println!("Standard Version:  {:?}", duration_std);

    // Benchmark Optimized (Unsafe)
    let start_opt = std::time::Instant::now();
    for _ in 0..iterations {
        Solution::eval_rpn(tokens3.clone());
    }
    let duration_opt = start_opt.elapsed();
    println!("Optimized Version: {:?}", duration_opt);

    // Benchmark Optimized + Safe
    let start_safe = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = Solution::eval_rpn_safe(tokens3.clone());
    }
    let duration_safe = start_safe.elapsed();
    println!("Safe+Optimized:    {:?}", duration_safe);

    if duration_opt < duration_std {
        let multiplier = duration_std.as_secs_f64() / duration_opt.as_secs_f64();
        println!(
            "Result: Optimized version is {:.2}x faster than standard!",
            multiplier
        );
    }
}

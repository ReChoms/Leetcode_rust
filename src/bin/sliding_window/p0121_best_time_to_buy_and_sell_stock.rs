/*
You are given an integer array prices where prices[i] is the price of NeetCoin on the ith day.

You may choose a single day to buy one NeetCoin and choose a different day in the future to sell it.

Return the maximum profit you can achieve. You may choose to not make any transactions, in which case the profit would be 0.

Example 1:

Input: prices = [10,1,5,6,7,1]
Output: 6
Explanation: Buy prices[1] and sell prices[4], profit = 7 - 1 = 6.

Example 2:

Input: prices = [10,8,7,5,2]
Output: 0
Explanation: No profitable transactions can be made, thus the max profit is 0.

Constraints:
1 <= prices.length <= 100
0 <= prices[i] <= 100
*/

use std::time::Instant;

struct Solution;

impl Solution {
    // Version 1: Original explicit branching approach
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
            if price < min_price {
                min_price = price;
            } else {
                if price - min_price > max_profit {
                    max_profit = price - min_price;
                }
            }
        }

        max_profit
    }

    // Version 2: Idiomatic procedural approach using std::cmp methods (.min() and .max())
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
            // Enhancement 1: Cleanly track the cheapest buy day seen so far without nested if/else
            min_price = min_price.min(price);

            // Enhancement 2: Calculate the potential profit on this day and keep the maximum profit
            max_profit = max_profit.max(price - min_price);
        }

        max_profit
    }

    // Version 3: Functional iterator fold (zero mutable local variables)
    pub fn max_profit_v3(prices: Vec<i32>) -> i32 {
        // Enhancement: Leverages iterator state accumulation via fold((min_price, max_profit), price)
        prices
            .into_iter()
            .fold((i32::MAX, 0), |(min_price, max_profit), price| {
                (min_price.min(price), max_profit.max(price - min_price))
            })
            .1
    }
}

fn main() {
    println!("=== Testing Best Time to Buy and Sell Stock ===");

    let tests: Vec<(Vec<i32>, i32, &str)> = vec![
        (vec![10, 1, 5, 6, 7, 1], 6, "NeetCode Example 1"),
        (
            vec![10, 8, 7, 5, 2],
            0,
            "NeetCode Example 2 (strictly decreasing)",
        ),
        (vec![7, 1, 5, 3, 6, 4], 5, "Standard LeetCode Example 1"),
        (vec![5], 0, "Single element array"),
        (vec![1, 5], 4, "Two elements (profitable)"),
        (vec![5, 1], 0, "Two elements (unprofitable)"),
        (vec![3, 3, 3, 3], 0, "All identical prices"),
        (vec![2, 4, 1], 2, "Lowest price at end after maximum profit"),
        (vec![0, 100], 100, "Boundary price values"),
    ];

    println!("\n--- Testing v1 (Original) ---");
    for (i, (prices, expected, desc)) in tests.iter().enumerate() {
        let actual = Solution::max_profit(prices.clone());
        assert_eq!(actual, *expected, "v1 Failed test {} ({})", i + 1, desc);
        println!("✅ v1 Test {} ({}) PASSED", i + 1, desc);
    }

    println!("\n--- Testing v2 (Idiomatic .min / .max) ---");
    for (i, (prices, expected, desc)) in tests.iter().enumerate() {
        let actual = Solution::max_profit_v2(prices.clone());
        assert_eq!(actual, *expected, "v2 Failed test {} ({})", i + 1, desc);
        println!("✅ v2 Test {} ({}) PASSED", i + 1, desc);
    }

    println!("\n--- Testing v3 (Functional .fold) ---");
    for (i, (prices, expected, desc)) in tests.iter().enumerate() {
        let actual = Solution::max_profit_v3(prices.clone());
        assert_eq!(actual, *expected, "v3 Failed test {} ({})", i + 1, desc);
        println!("✅ v3 Test {} ({}) PASSED", i + 1, desc);
    }

    println!("\n=== Performance Benchmarking (100,000 runs) ===");
    let benchmark_input = vec![
        7, 1, 5, 3, 6, 4, 8, 2, 9, 3, 10, 5, 1, 14, 2, 8, 12, 4, 15, 6, 2, 18, 9, 3,
    ];
    let iterations = 100_000;

    // Benchmark v1
    let start_v1 = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(Solution::max_profit(benchmark_input.clone()));
    }
    let duration_v1 = start_v1.elapsed();

    // Benchmark v2
    let start_v2 = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(Solution::max_profit_v2(benchmark_input.clone()));
    }
    let duration_v2 = start_v2.elapsed();

    // Benchmark v3
    let start_v3 = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(Solution::max_profit_v3(benchmark_input.clone()));
    }
    let duration_v3 = start_v3.elapsed();

    println!("v1 (Original)          : {:?}", duration_v1);
    println!("v2 (Idiomatic min/max) : {:?}", duration_v2);
    println!("v3 (Functional fold)   : {:?}", duration_v3);

    let v1_micros = duration_v1.as_micros() as f64;
    let v2_micros = duration_v2.as_micros() as f64;
    let v3_micros = duration_v3.as_micros() as f64;

    if v2_micros < v1_micros {
        println!(
            "⚡ v2 is {:.2}x faster than v1!",
            v1_micros / v2_micros.max(1.0)
        );
    }
    if v3_micros < v1_micros {
        println!(
            "⚡ v3 is {:.2}x faster than v1!",
            v1_micros / v3_micros.max(1.0)
        );
    }
}

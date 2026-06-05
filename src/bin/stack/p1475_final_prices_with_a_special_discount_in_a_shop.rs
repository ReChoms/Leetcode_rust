/*
1475. Final Prices With a Special Discount in a Shop
Easy

You are given an integer array prices where prices[i] is the price of the ith item in a shop.

There is a special discount for items in the shop. If you buy the ith item, then you will receive a discount equivalent to prices[j] where j is the minimum index such that j > i and prices[j] <= prices[i]. Otherwise, you will not receive any discount at all.

Return an integer array answer where answer[i] is the final price you will pay for the ith item of the shop, considering the special discount.

Example 1:
Input: prices = [8,4,6,2,3]
Output: [4,2,4,2,3]
Explanation:
For item 0 with price[0]=8 you will receive a discount equivalent to prices[1]=4, therefore, the final price you will pay is 8 - 4 = 4.
For item 1 with price[1]=4 you will receive a discount equivalent to prices[3]=2, therefore, the final price you will pay is 4 - 2 = 2.
For item 2 with price[2]=6 you will receive a discount equivalent to prices[3]=2, therefore, the final price you will pay is 6 - 2 = 4.
For items 3 and 4 you will not receive any discount at all.

Example 2:
Input: prices = [1,2,3,4,5]
Output: [1,2,3,4,5]
Explanation: In this case, for all items, you will not receive any discount at all.

Example 3:
Input: prices = [10,1,1,6]
Output: [9,0,1,6]

Constraints:
1 <= prices.length <= 500
1 <= prices[i] <= 1000
*/

use std::{time::Instant, vec};

pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
    // We use a stack to store the INDICES of the prices.
    let mut stack: Vec<usize> = Vec::new();

    // Iterate through the prices using just the index 0 to len-1
    for i in 0..prices.len() {
        
        // While the stack is not empty AND the current price is less than or equal to 
        // the price at the index on top of the stack:
        while !stack.is_empty() && prices[i] <= prices[*stack.last().unwrap()] {
            
            // Pop the index of the item that is getting the discount
            let popped_index = stack.pop().unwrap();
            
            // Apply the discount in place!
            prices[popped_index] -= prices[i];
        }
        
        // Push the current index onto the stack to see if it gets a discount later
        stack.push(i);
    }

    // Since we modified prices in place, we just return it
    prices
}

fn main() {
    println!("=== 1475. Final Prices With a Special Discount in a Shop ===");

    let tests = vec![
        (vec![8, 4, 6, 2, 3], vec![4, 2, 4, 2, 3]),
        (vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
        (vec![10, 1, 1, 6], vec![9, 0, 1, 6]),
    ];

    let mut all_passed = true;
    for (i, (prices, expected)) in tests.iter().enumerate() {
        let result = final_prices(prices.clone());
        if result == *expected {
            println!("Test {} PASSED", i + 1);
        } else {
            println!("Test {} FAILED", i + 1);
            println!("  Input:    {:?}", prices);
            println!("  Expected: {:?}", expected);
            println!("  Got:      {:?}", result);
            all_passed = false;
        }
    }

    if all_passed {
        println!("\nAll basic tests PASSED! 🎉");
        println!("\n--- Benchmarking ---");

        let test_input = vec![8, 4, 6, 2, 3];

        // Memory check for capacity
        let result = final_prices(test_input.clone());
        println!(
            "Memory Allocation: Vector length = {}, capacity = {}",
            result.len(),
            result.capacity()
        );
        if result.capacity() > result.len() {
            println!(
                "  Hint: You might be over-allocating or resizing dynamically. Consider exact capacity allocation (e.g., Vec::with_capacity)."
            );
        } else {
            println!("  Memory allocation is perfectly sized!");
        }

        // Speed benchmark
        let start = Instant::now();
        let iterations = 100_000;
        for _ in 0..iterations {
            let _ = final_prices(test_input.clone());
        }
        let duration = start.elapsed();
        println!("Execution time ({} iterations): {:?}", iterations, duration);
        println!("Let's see if we can optimize it further!");
    } else {
        println!("\nSome tests FAILED. Let's focus on fixing the logic first!");
        println!(
            "For the first item in Example 1 (price=8), we need to find the FIRST subsequent item that is <= 8. How would you like to search for that item?"
        );
    }
}

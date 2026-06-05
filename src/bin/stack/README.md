# Stack Problems

This folder contains solutions for problems that can be solved optimally using a Stack data structure. 
Stacks are LIFO (Last-In-First-Out) structures, making them perfect for problems involving parsing, reversing, or matching elements (like parenthesis), as well as tracking function execution (call stacks).

## Key Patterns:

### 1. Simple Stack
Used for simulating operations, evaluating expressions (like Reverse Polish Notation), or matching pairs (like valid parentheses).
- `p01441_build_array_with_stack_operations`: Basic stack simulation.
- `p0150_evaluate_reverse_polish_notation`: Parsing and evaluating math expressions.
- `p0636_exclusive_time_of_functions`: Simulating an execution call stack.

### 2. Monotonic Stack
A specialized use case of a stack where the elements are kept strictly increasing or decreasing.
- **When to use:** It is primarily used for finding the "Next Greater Element" or "Next Smaller Element" in an array in O(n) time.
- **How it works:** We iterate through the array, and before pushing an element onto the stack, we pop elements that break the monotonic property. 
- **Examples:** Daily Temperatures, Trapping Rain Water, Largest Rectangle in Histogram.

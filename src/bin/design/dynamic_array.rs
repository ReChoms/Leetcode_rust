/*
Design a Dynamic Array (aka a resizable array) class, such as an ArrayList in Java or a vector in C++.

Your DynamicArray class should support the following operations:

- DynamicArray(int capacity) will initialize an empty array with a capacity of capacity, where capacity > 0.
- int get(int i) will return the element at index i. Assume that index i is valid.
- void set(int i, int n) will set the element at index i to n. Assume that index i is valid.
- void pushback(int n) will push the element n to the end of the array.
- int popback() will pop and return the element at the end of the array. Assume that the array is non-empty.
- void resize() will double the capacity of the array.
- int getSize() will return the number of elements in the array.
- int getCapacity() will return the capacity of the array.
If we call pushback(int n) but the array is full, we should resize() the array first.

Note:
The index i provided to get(int i) and set(int i) is guaranteed to be greater than or equal to 0 and less than the number of elements in the array.
*/

pub struct DynamicArray {
    capacity: i32,
    length: i32,
    memory: Vec<i32>,
    // TODO: Add the 3 state fields we discussed!
    // Hint: For the fixed-size memory block in safe Rust, you can use a Vector filled with zeros
    // to simulate the allocated space: vec![0; capacity as usize]
}

impl DynamicArray {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity,
            length: 0,
            memory: vec![0; capacity as usize],
        }
    }

    pub fn get(&self, i: i32) -> i32 {
        self.memory[i as usize]
    }

    pub fn set(&mut self, i: i32, n: i32) {
        self.memory[i as usize] = n;
    }

    pub fn pushback(&mut self, n: i32) {
        if self.capacity == self.length {
            self.resize();
        }
        self.memory[self.length as usize] = n;

        // 3. Increase the length by 1
        self.length += 1;
    }

    pub fn popback(&mut self) -> i32 {
        self.length -= 1;
        let popped_value = self.memory[self.length as usize];

        // Bonus: Automatic shrinking when length drops to 25% of capacity!
        // We ensure capacity >= 4 so we never shrink capacity down to 0.
        if self.capacity >= 4 && self.length <= self.capacity / 4 {
            self.shrink();
        }

        popped_value
    }

    pub fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut new_memory: Vec<i32> = vec![0; new_capacity as usize];
        for i in 0..self.length {
            new_memory[i as usize] = self.memory[i as usize];
        }
        self.memory = new_memory;
        self.capacity = new_capacity;
    }

    pub fn shrink(&mut self) {
        let new_capacity = self.capacity / 2;
        let mut new_memory: Vec<i32> = vec![0; new_capacity as usize];
        for i in 0..self.length {
            new_memory[i as usize] = self.memory[i as usize];
        }
        self.memory = new_memory;
        self.capacity = new_capacity;
    }

    pub fn get_size(&self) -> i32 {
        self.length
    }

    pub fn get_capacity(&self) -> i32 {
        self.capacity
    }
}

fn main() {
    println!("--- Running Dynamic Array Tests ---");

    // Example 1
    let mut arr1 = DynamicArray::new(1);
    assert_eq!(arr1.get_size(), 0, "Ex 1: size should be 0");
    assert_eq!(arr1.get_capacity(), 1, "Ex 1: capacity should be 1");
    println!("✅ Example 1 Passed!");

    // Example 2
    let mut arr2 = DynamicArray::new(1);
    arr2.pushback(1);
    assert_eq!(arr2.get_capacity(), 1, "Ex 2: capacity should remain 1");
    arr2.pushback(2);
    assert_eq!(arr2.get_capacity(), 2, "Ex 2: capacity should double to 2");
    println!("✅ Example 2 Passed!");

    // Example 3
    let mut arr3 = DynamicArray::new(1);
    assert_eq!(arr3.get_size(), 0);
    assert_eq!(arr3.get_capacity(), 1);

    arr3.pushback(1);
    assert_eq!(arr3.get_size(), 1);
    assert_eq!(arr3.get_capacity(), 1);

    arr3.pushback(2);
    assert_eq!(arr3.get_size(), 2);
    assert_eq!(arr3.get_capacity(), 2);

    assert_eq!(arr3.get(1), 2);
    arr3.set(1, 3);
    assert_eq!(arr3.get(1), 3);

    assert_eq!(arr3.popback(), 3);
    assert_eq!(arr3.get_size(), 1);
    assert_eq!(arr3.get_capacity(), 2);
    println!("✅ Example 3 Passed!");

    // --- Additional Edge Case Tests ---

    // Test pushing past multiple resizes
    let mut arr_stress = DynamicArray::new(2);
    for i in 0..10 {
        arr_stress.pushback(i);
    }
    assert_eq!(arr_stress.get_size(), 10, "Stress test: size should be 10");
    assert_eq!(
        arr_stress.get_capacity(),
        16,
        "Stress test: capacity should be 16 (2 -> 4 -> 8 -> 16)"
    );
    println!("✅ Multiple Resizes Passed!");

    println!("\n🎉 All tests passed successfully!");
}

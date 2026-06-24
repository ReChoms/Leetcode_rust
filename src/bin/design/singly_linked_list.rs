/*
Design a Singly Linked List class.

Your LinkedList class should support the following operations:

LinkedList() will initialize an empty linked list.
int get(int i) will return the value of the ith node (0-indexed). If the index is out of bounds, return -1.
void insertHead(int val) will insert a node with val at the head of the list.
void insertTail(int val) will insert a node with val at the tail of the list.
bool remove(int i) will remove the ith node (0-indexed). If the index is out of bounds, return false, otherwise return true.
int[] getValues() return an array of all the values in the linked list, ordered from head to tail.

Example 1:
Input:
["insertHead", 1, "insertTail", 2, "insertHead", 0, "remove", 1, "getValues"]
Output:
[null, null, null, true, [0, 2]]

Example 2:
Input:
["insertHead", 1, "insertHead", 2, "get", 5]
Output:
[null, null, -1]

Note:
The index int i provided to get(int i) and remove(int i) is guaranteed to be greater than or equal to 0.
*/

pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    size: i32,
}

impl LinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        let mut node = &self.head;
        for _ in 0..index {
            if let Some(n) = node {
                // Safely jump to the next node
                node = &n.next;
            } else {
                // If we hit None before reaching the index, it's out of bounds!
                return -1;
            }
        }
        if let Some(n) = node { n.value } else { -1 }
    }

    pub fn insert_head(&mut self, val: i32) {
        let new_node = Box::new(Node {
            value: val,
            next: self.head.take(),
        });
        self.size += 1;
        self.head = Some(new_node);
    }

    pub fn insert_tail(&mut self, val: i32) {
        let mut node = &mut self.head;

        for _ in 0..self.size {
            if let Some(n) = node {
                node = &mut n.next;
            } else {
                return;
            }
        }
        let new_node = Box::new(Node {
            value: val,
            next: None,
        });
        self.size += 1;
        *node = Some(new_node);
    }

    pub fn remove(&mut self, index: i32) -> bool {
        if index < 0 || index >= self.size {
            return false;
        }

        let mut node = &mut self.head;

        for _ in 0..index {
            if let Some(n) = node {
                node = &mut n.next;
            }
        }

        // Steal the node out of the pointer, and replace it with whatever came after it!
        if let Some(mut target) = node.take() {
            *node = target.next.take();
            self.size -= 1;
            true
        } else {
            false
        }
    }

    pub fn get_values(&self) -> Vec<i32> {
        // Pre-allocating memory since we know the exact size!
        let mut values = Vec::with_capacity(self.size as usize);
        let mut node = &self.head;

        while let Some(n) = node {
            values.push(n.value);
            node = &n.next;
        }
        values
    }
}

fn main() {
    println!("Run `cargo test --bin singly_linked_list` to test your solution!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut list = LinkedList::new();
        list.insert_head(1);
        list.insert_tail(2);
        list.insert_head(0);
        assert_eq!(list.remove(1), true);
        assert_eq!(list.get_values(), vec![0, 2]);
    }

    #[test]
    fn test_example_2() {
        let mut list = LinkedList::new();
        list.insert_head(1);
        list.insert_head(2);
        assert_eq!(list.get(5), -1);
    }

    #[test]
    fn test_edge_cases() {
        let mut list = LinkedList::new();
        // Edge Case: Remove from empty list
        assert_eq!(list.remove(0), false);

        // Edge Case: Get from empty list
        assert_eq!(list.get(0), -1);

        // Edge Case: Get values from empty list
        assert_eq!(list.get_values(), Vec::<i32>::new());

        // Edge Case: Single element insertion and removal
        list.insert_tail(10);
        assert_eq!(list.get(0), 10);
        assert_eq!(list.remove(0), true);
        assert_eq!(list.get_values(), Vec::<i32>::new());

        // Edge Case: Remove out of bounds index
        list.insert_head(5);
        assert_eq!(list.remove(1), false);
        assert_eq!(list.get_values(), vec![5]);
    }
}

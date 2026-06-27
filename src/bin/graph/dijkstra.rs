/*
Dijkstra's Algorithm

Given a weighted, directed graph, and a starting vertex, return the shortest distance from the starting vertex to every vertex in the graph.

Input:

n - the number of vertices in the graph, where (2 <= n <= 100). Each vertex is labeled from 0 to n - 1.
edges - a list of tuples, each representing a directed edge in the form (u, v, w), where u is the source vertex, v is the destination vertex, and w is the weight of the edge, where (1 <= w <= 10).
src - the source vertex from which to start the algorithm, where (0 <= src < n).
Note: If a vertex is unreachable from the source vertex, the shortest path distance for the unreachable vertex should be -1.

Example 1:

Input:
n = 5
edges = [[0,1,10], [0,2,3], [1,3,2], [2,1,4], [2,3,8], [2,4,2], [3,4,5]]
src = 0

Output:
{{0:0}, {1:7}, {2:3}, {3:9}, {4:5}}
*/

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, src: i32) -> HashMap<i32, i32> {
    // 1. Build the Adjacency List
    // adj[u] will contain a list of tuples (destination_v, weight)
    let mut adj = vec![Vec::new(); n as usize];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        let w = edge[2];
        adj[u].push((v, w));
    }

    // 2. Initialize Distances Map and Priority Queue (Min-Heap)
    let mut distances = HashMap::new();
    
    // The heap stores Reverse((current_cost, current_node)) to act as a Min-Heap
    let mut pq = BinaryHeap::new();
    
    // Start at the source node with a cost of 0
    pq.push(Reverse((0, src as usize)));
    // 3. Process the Queue
    while let Some(Reverse((current_cost, current_node))) = pq.pop() {
        // Step 1: If we already found a shorter path to this node, throw this one in the trash!
        if let Some(&best_cost) = distances.get(&(current_node as i32)) {
            if current_cost > best_cost {
                continue;
            }
        }
        
        // Step 2: Otherwise, this is the shortest path we've found so far, so save it!
        distances.insert(current_node as i32, current_cost);

        // Step 3: Check all neighbors of `current_node` using `adj` and push them into the `pq`!
        for &(next_node, edge_weight) in &adj[current_node] {
            let new_cost = current_cost + edge_weight;
            
            // Optimization: Only push to the queue if this new cost is actually better 
            // than what we currently have saved for `next_node`.
            let is_better = match distances.get(&(next_node as i32)) {
                Some(&best_cost) => new_cost < best_cost,
                None => true, // We haven't visited it yet, so it's definitely better!
            };
            
            if is_better {
                pq.push(Reverse((new_cost, next_node)));
            }
        }
    }

    // Fill in missing unreachable nodes with -1 before returning
    for i in 0..n {
        distances.entry(i).or_insert(-1);
    }

    distances
}

fn main() {
    // ---------------------------------------------------------
    // Test Suites & Benchmarks
    // ---------------------------------------------------------

    // Test Case 1: Standard Example
    let n1 = 5;
    let edges1 = vec![
        vec![0, 1, 10],
        vec![0, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 4],
        vec![2, 3, 8],
        vec![2, 4, 2],
        vec![3, 4, 5],
    ];
    let res1 = shortest_path(n1, edges1, 0);
    println!("Test 1 Output: {:?}", res1);

    // We will add more edge cases (e.g., unreachable nodes,
    // single node graph, disconnected components) as we build out the code!
}

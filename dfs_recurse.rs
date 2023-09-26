use std::collections::HashMap;
use std::collections::HashSet;

// Recursive depth first search (DFS).
fn dfs(mut visited: &mut HashSet<usize>, graph: &HashMap<usize, Vec<usize>>, node: usize) {

    // Check if the node has been visited.
    if !visited.contains(&node) {

        // It hasn't been visited, so mark it as visited now.
        visited.insert(node);
        println!("{}", node);

        // Get all children of this node.
        let vertexs: &Vec<usize> = graph.get(&node).unwrap();

        // For each child, visit.
        for v in vertexs {

            // Recusive call.
            dfs(visited, graph, v.clone());
        }
    }
}

// Entry point.
fn main() {
    // Set of visited (no duplicates).
    let mut visited: HashSet<usize> = HashSet::new();

    // A hashmap of tuples which represent a tree.  The first item is the node, 
    // and the second item which is the Vector is the node's children.  
    // In this case, the tree is a binary tree since each node has 
    // 2 or less children.
    let mut graph = HashMap::from([
        (5 , vec![3, 7]),
        (3 , vec![2, 4]),
        (7 , vec![8]),
        (2 , vec![]),
        (4 , vec![8]),
        (8 , vec![])
]);

    // Start DFS at the root node 5.
    dfs(&mut visited, &mut graph, 5);
}

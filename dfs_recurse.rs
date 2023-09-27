use std::collections::HashMap;
use std::collections::HashSet;

// Recursive depth first search.
fn dfs(visited: &mut HashSet<usize>, graph: &HashMap<usize, Vec<usize>>, vertex: usize) {
    // Check if the vertex has been visited.
    if !visited.contains(&vertex) {
        println!("visit {}", vertex);

        // It hasn't been visited, so mark it as visited now.
        visited.insert(vertex);

        // Get all edges of this vertex.
        let edges: &Vec<usize> = graph.get(&vertex).unwrap();

        // For each child, visit.
        for &v in edges {

            // Recusive call.
            dfs(visited, graph, v);
        }
    }
}

// Iterative depth first search.
fn dfs_iter(visited: &mut HashSet<usize>, graph: &HashMap<usize, Vec<usize>>, vertex: usize) {
    let l = graph.len();

    // The stack will never be larger than the graph. 
    let mut s: Vec<usize> = Vec::with_capacity(l);
    s.push(vertex);

    while !s.is_empty() {
        let v: usize = s.pop().unwrap();

        // Check if the vertex has been visited.
        if !visited.contains(&v) {
            println!("visit {}", v);

            // It hasn't been visited, so mark it as visited now.
            visited.insert(v);

            // Get all edges of this vertex.
            let edges: &Vec<usize> = graph.get(&v).unwrap();

            for &v in edges {
                s.push(v);
            }
        }
    }
}

// Entry point.
fn main() {
    // A hashmap of tuples which represent a graph.  The first item is the vertex, 
    // and the second item which are edges.  
    let mut graph = HashMap::from([
        (5 , vec![3, 7]),
        (3 , vec![2, 4]),
        (7 , vec![8]),
        (2 , vec![]),
        (4 , vec![8]),
        (8 , vec![])
    ]);

    let l = graph.len();

    // Start DFS at the root node 5.
    // Set of visited (no duplicates)
    let mut visited: HashSet<usize> = HashSet::with_capacity(l);
    dfs(&mut visited, &mut graph, 5);

    // Set of visited (no duplicates).
    let mut visited: HashSet<usize> = HashSet::with_capacity(l);
    dfs_iter(&mut visited, &mut graph, 5);
}

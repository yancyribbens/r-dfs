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

// If the input is a tree and not a graph (without cycles), then
// the solution becomes simpler since there is no need to maintain
// a visitied list.

fn dfs_tree(tree: &HashMap<usize, Vec<usize>>, root: usize) {
    println!("visit {}", root);

    let branches: &Vec<usize> = tree.get(&root).unwrap();

    for &node in branches {

        dfs_tree(tree, node);
    }
}

fn dfs_tree_iter(tree: &HashMap<usize, Vec<usize>>, root: usize) {
    let mut s = Vec::new();

    s.push(root);

    while !s.is_empty() {
        let n = s.pop().unwrap();

        println!("visit {}", n);

        let branches = tree.get(&n).unwrap();

        for &n in branches {
            s.push(n);
        }
    }
}

// Iterative depth first search.
fn dfs_iter(visited: &mut HashSet<usize>, graph: &HashMap<usize, Vec<usize>>, vertex: usize) {
    let l = graph.len();

    // The stack will never be larger than the graph. 
    // The efficency of s could be improved by using a fixed size array
    // however, there is no push() nor pop() and so an index pointer is needed.
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
        (4 , vec![]),
        (8 , vec![])
    ]);

    //let l = graph.len();

    //let mut visited: HashSet<usize> = HashSet::with_capacity(l);
    dfs_tree_iter(&mut graph, 5);

    // Start DFS at the root node 5.
    // Set of visited (no duplicates)
    //let mut visited: HashSet<usize> = HashSet::with_capacity(l);
    //dfs(&mut visited, &mut graph, 5);

    // Set of visited (no duplicates).
    //let mut visited: HashSet<usize> = HashSet::with_capacity(l);
    //dfs_iter(&mut visited, &mut graph, 5);
}

use std::collections::HashMap;
use std::collections::HashSet;

fn dfs(mut visited: &mut HashSet<usize>, graph: &HashMap<usize, Vec<usize>>, node: usize) {
	if !visited.contains(&node) {
		visited.insert(node);
		println!("{}", node);
		let vertexs: &Vec<usize> = graph.get(&node).unwrap();

		for v in vertexs {
			dfs(visited, graph, v.clone());
		}
	}
}

fn main() {
		let mut visited: HashSet<usize> = HashSet::new();

		let mut graph = HashMap::from([
				(5 , vec![3, 7]),
				(3 , vec![2, 4]),
  			(7 , vec![8]),
  			(2 , vec![]),
				(4 , vec![8]),
  			(8 , vec![])
		]);

		dfs(&mut visited, &mut graph, 5);
}

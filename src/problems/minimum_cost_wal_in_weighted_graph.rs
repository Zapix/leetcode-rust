use std::collections::{HashMap, HashSet, VecDeque};
struct Solution;

impl Solution {
    pub fn insert_value(
        graph: &mut HashMap<usize, HashMap<usize, i32>>,
        x: usize,
        y: usize,
        w: i32,
    ) {
        let connected_nodes = graph.entry(x).or_insert(HashMap::new());
        let entry = connected_nodes.entry(y).or_insert(i32::MAX);
        *entry &= w;
    }
    pub fn build_graph(n: usize, edges: &Vec<Vec<i32>>) -> HashMap<usize, HashMap<usize, i32>> {
        let mut graph = HashMap::from_iter((0..n).map(|x| (x, HashMap::new())));
        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            let w = edge[2];

            Self::insert_value(&mut graph, x, y, w);
            Self::insert_value(&mut graph, y, x, w);
        }

        graph
    }

    pub fn get_groups(
        n: usize,
        graph: &HashMap<usize, HashMap<usize, i32>>,
    ) -> (HashMap<usize, usize>, HashMap<usize, i32>) {
        let mut visited = HashSet::new();

        let mut group_count = 0;
        let mut node_to_group = HashMap::new();
        let mut group_to_value = HashMap::new();

        for node in 0..n {
            if visited.contains(&node) {
                continue;
            }
            let mut value = i32::MAX;
            let mut queue = VecDeque::new();
            queue.push_back(node);
            while let Some(node) = queue.pop_front() {
                node_to_group.insert(node, group_count);
                visited.insert(node);
                for (i, weight) in graph.get(&node).unwrap_or(&HashMap::new()) {
                    if *i == node || visited.contains(&i) {
                        continue;
                    }
                    value &= *weight;
                    queue.push_back(*i);
                }
            }
            group_to_value.insert(group_count, value);
            group_count += 1;
        }
        (node_to_group, group_to_value)
    }
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let graph = Self::build_graph(n as usize, &edges);
        let (node_to_group, group_to_value) = Self::get_groups(n as usize, &graph);

        queries
            .iter()
            .map(|x| {
                let u = x[0] as usize;
                let v = x[1] as usize;
                if node_to_group[&u] == node_to_group[&v] {
                    let group = node_to_group[&u];
                    group_to_value[&group]
                } else {
                    -1
                }
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 5;
        let edges = vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]];
        let queries = vec![vec![0, 3], vec![3, 4]];
        assert_eq!(Solution::minimum_cost(n, edges, queries), vec![1, -1]);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let edges = vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]];
        let queries = vec![vec![1, 2]];
        assert_eq!(Solution::minimum_cost(n, edges, queries), vec![0]);
    }
}

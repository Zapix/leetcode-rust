use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
struct Solution;

impl Solution {
    pub fn build_graph(c: usize, connections: &Vec<Vec<i32>>) -> HashMap<usize, HashSet<usize>> {
        let mut graph = HashMap::new();
        for conn in connections {
            let a = conn[0] as usize;
            let b = conn[1] as usize;
            graph.entry(a).or_insert_with(HashSet::new).insert(b);
            graph.entry(b).or_insert_with(HashSet::new).insert(a);
        }
        graph
    }
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut groups, node_to_group) = {
            let graph = Solution::build_graph(c as usize, &connections);

            let mut visited = vec![false; (c + 1) as usize];
            let mut node_to_group = vec![0; (c + 1) as usize];
            let mut groups = vec![];
            for i in 1..=c as usize {
                if visited[i] {
                    continue;
                }
                let mut que = VecDeque::new();
                que.push_back(i);
                let group_idx = groups.len();
                let mut group = BTreeSet::new();
                while let Some(node) = que.pop_front() {
                    if visited[node] {
                        continue;
                    }
                    visited[node] = true;
                    group.insert(node);
                    node_to_group[node] = group_idx;
                    for &neighbor in graph.get(&node).unwrap_or(&HashSet::new()) {
                        if !visited[neighbor] {
                            que.push_back(neighbor);
                        }
                    }
                }
                groups.push(group);
            }
            (groups, node_to_group)
        };

        let mut offline = HashSet::new();
        let mut result = vec![];

        for query in queries {
            let t = query[0];
            let x = query[1] as usize;

            match t {
                1 => {
                    if !offline.contains(&x) {
                        result.push(x as i32);
                    } else {
                        let group_idx = node_to_group[x];
                        let mut value = match groups[group_idx].first(){
                            Some(&val) => val as i32,
                            None => -1,
                        };
                        result.push(value);
                    }
                }
                2 => {
                    offline.insert(x);
                    groups[node_to_group[x]].remove(&x);
                }
                _ => panic!("Invalid query type"),
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let c = 5;
        let connections = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        let queries = vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]];
        assert_eq!(
            Solution::process_queries(c, connections, queries),
            vec![3, 2, 3]
        );
    }
    #[test]
    fn test_example_2() {
        let c = 3;
        let connections = vec![];
        let queries = vec![vec![1, 1], vec![2, 1], vec![1, 1]];
        assert_eq!(
            Solution::process_queries(c, connections, queries),
            vec![1, -1]
        );
    }
}

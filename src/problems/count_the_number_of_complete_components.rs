use std::collections::{HashMap, HashSet, VecDeque};

type Graph = HashMap<usize, HashSet<usize>>;
type Group = HashSet<usize>;

struct Solution;

impl Solution {
    fn build_graph(n: usize, edges: &Vec<Vec<i32>>) -> Graph {
        let mut graph = HashMap::from_iter((0..n).map(|x| (x, HashSet::new())));
        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;

            if let Some(nodes) = graph.get_mut(&x) {
                nodes.insert(y);
            }
            if let Some(nodes) = graph.get_mut(&y) {
                nodes.insert(x);
            }
        }
        graph
    }

    fn get_groups(n: usize, graph: &Graph) -> Vec<Group> {
        let mut visited = HashSet::new();
        let mut groups = vec![];

        for node in 0..n {
            if visited.contains(&node) {
                continue;
            }
            let mut que = VecDeque::new();
            let mut group = HashSet::new();
            que.push_back(node);
            while let Some(node) = que.pop_front() {
                if visited.contains(&node) {
                    continue;
                }
                visited.insert(node);
                group.insert(node);
                for next_node in graph.get(&node).unwrap() {
                    que.push_back(*next_node);
                }
            }
            groups.push(group);
        }

        groups
    }

    fn check_completeness(group: &Group, graph: &Graph) -> bool {
        let n = group.len();
        group.iter().all(|x| graph.get(x).unwrap().len() == n - 1)
    }

    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let graph = Solution::build_graph(n, &edges);
        let groups = Solution::get_groups(n, &graph);
        groups.iter().filter(|x| Solution::check_completeness(x, &graph)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_complete_components_example1() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::count_complete_components(n, edges), 3);
    }

    #[test]
    fn test_count_complete_components_example2() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]];
        assert_eq!(Solution::count_complete_components(n, edges), 1);
    }

    #[test]
    fn test_count_complete_components_example3() {
        let n = 4;
        let edges = vec![vec![0, 1], vec![2, 3]];
        assert_eq!(Solution::count_complete_components(n, edges), 2);
    }

    #[test]
    fn test_count_complete_components_example4() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![3, 4]];
        assert_eq!(Solution::count_complete_components(n, edges), 2);
    }

    #[test]
    fn test_count_complete_components_example5() {
        let n = 3;
        let edges = vec![];
        assert_eq!(Solution::count_complete_components(n, edges), 3);
    }
}

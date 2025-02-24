use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn build_nodes(edges: &Vec<Vec<i32>>) -> HashMap<usize, Vec<usize>> {
        let mut nodes = HashMap::new();
        for edge in edges {
            let i = usize::try_from(edge[0]).unwrap();
            let j = usize::try_from(edge[1]).unwrap();
            let entry = nodes.entry(i).or_insert(vec![]);
            entry.push(j);
            let entry = nodes.entry(j).or_insert(vec![]);
            entry.push(i);
        }
        nodes
    }

    fn find_path(path: &mut Vec<usize>, target: usize, nodes: &HashMap<usize, Vec<usize>>) -> bool {
        match path.last() {
            Some(x) if *x == target => true,
            Some(x) => {
                for node in nodes.get(x).unwrap_or(&vec![]) {
                    if path.contains(node) {
                        continue;
                    }
                    path.push(*node);
                    if Self::find_path(path, target, nodes) {
                        return true;
                    }
                    path.pop();
                }
                false
            }
            None => false,
        }
    }
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let nodes = Self::build_nodes(&edges);
        let mut bobs_path = vec![0];
        Self::find_path(&mut bobs_path, usize::try_from(bob).unwrap(), &nodes);
        let bobs_times = bobs_path
            .iter()
            .rev()
            .enumerate()
            .map(|x| (*x.1, x.0))
            .collect::<HashMap<usize, usize>>();

        let mut max_value = i32::MIN;
        let mut stack = vec![(0usize, 0usize, amount[0])];
        let mut visited = HashSet::new();
        while let Some((node, time, value)) = stack.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            match nodes.get(&node) {
                Some(next_nodes) if next_nodes.len() == 1 && node != 0 => {
                    max_value = max_value.max(value);
                }
                Some(next_nodes) => {
                    for next_node in next_nodes {
                        let bobs_node_time = bobs_times.get(next_node).unwrap_or(&usize::MAX);
                        let curr = match (time + 1).cmp(bobs_node_time) {
                            Ordering::Less => amount[*next_node],
                            Ordering::Equal => amount[*next_node] / 2,
                            Ordering::Greater => 0,
                        };
                        stack.push((*next_node, time + 1, value + curr))
                    }
                }
                None => {
                    max_value = max_value.max(value);
                }
            }
        }
        max_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]];
        let bob = 3;
        let amount = vec![-2, 4, 2, -4, 6];
        assert_eq!(Solution::most_profitable_path(edges, bob, amount), 6);
    }

    #[test]
    fn test_example_2() {
        let edges = vec![vec![0, 1]];
        let bob = 1;
        let amount = vec![-7280, 2350];
        assert_eq!(Solution::most_profitable_path(edges, bob, amount), -7280);
    }

    #[test]
    fn test_example_3() {
        let edges = vec![vec![0, 2], vec![0, 4], vec![1, 3], vec![1, 2]];
        let bob = 1;
        let amount = vec![3958, -9854, -8334, -9388, 3410];
        assert_eq!(Solution::most_profitable_path(edges, bob, amount), 7368);
    }

    #[test]
    fn test_example_4() {
        let edges = vec![vec![0, 2], vec![0, 5], vec![1, 3], vec![1, 5], vec![2, 4]];
        let bob = 4;
        let amount = vec![5018, 8388, 6224, 3466, 3808, 3456];
        assert_eq!(Solution::most_profitable_path(edges, bob, amount), 20328);
    }
}

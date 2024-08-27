use std::cmp::{Eq, Ord, PartialEq};
use std::collections::{BinaryHeap, HashSet};
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
struct Probability(f64);

impl PartialOrd for Probability {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for Probability {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 1e-5
    }
}
impl Eq for Probability {}

impl Ord for Probability {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Mul for Probability {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Probability(self.0 * rhs.0)
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let mut graph: Vec<Vec<Probability>> = vec![vec![Probability(0.0); n as usize]; n as usize];
        for i in 0..edges.len() {
            let edge = &edges[i];
            graph[edge[0] as usize][edge[1] as usize] = Probability(succ_prob[i]);
            graph[edge[1] as usize][edge[0] as usize] = Probability(succ_prob[i]);
        }

        let mut visited = HashSet::new();
        let mut max_heap = BinaryHeap::new();
        max_heap.push((Probability(1.0 as f64), start_node));
        let mut probs = vec![Probability(0.0); n as usize];
        probs[start_node as usize] = Probability(1.0);
        while let Some((prob, node)) = max_heap.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            for i in 0..n {
                if graph[node as usize][i as usize] > Probability(0.0) {
                    let new_prob = prob * graph[node as usize][i as usize];
                    if new_prob > probs[i as usize] {
                        probs[i as usize] = new_prob;
                        max_heap.push((new_prob, i));
                    }
                }
            }
        }
        probs[end_node as usize].0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        let succ_prob = vec![0.5, 0.5, 0.2];
        let start_node = 0;
        let end_node = 2;
        let result = Solution::max_probability(n, edges, succ_prob, start_node, end_node);
        assert_eq!(result, 0.25000);
    }

    #[test]
    fn leetcode_2() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        let succ_prob = vec![0.5, 0.5, 0.3];
        let start_node = 0;
        let end_node = 2;
        let result = Solution::max_probability(n, edges, succ_prob, start_node, end_node);
        assert_eq!(result, 0.30000);
    }

    #[test]
    fn leetcode_3() {
        let n = 3;
        let edges = vec![vec![0, 1]];
        let succ_prob = vec![0.5];
        let start_node = 0;
        let end_node = 2;
        let result = Solution::max_probability(n, edges, succ_prob, start_node, end_node);
        assert_eq!(result, 0.00000);
    }
}

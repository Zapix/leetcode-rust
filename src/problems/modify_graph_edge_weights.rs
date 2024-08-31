use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_shortest_path(
        n: usize,
        graph: &Vec<Vec<i32>>,
        source: usize,
        destination: usize,
    ) -> Option<i32> {
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), source));
        while !queue.is_empty() {
            let (Reverse(distance), node) = queue.pop().unwrap();
            if (visited.contains(&node)) {
                continue;
            }
            visited.insert(node);
            if node == destination {
                return Some(distance);
            }
            for i in 0..n {
                if graph[node][i] > 0 && !visited.contains(&i) {
                    queue.push((Reverse(distance + graph[node][i]), i));
                }
            }
        }
        None
    }

    pub fn fill_with_max(edges: &mut Vec<Vec<i32>>) {
        for edge in edges.iter_mut() {
            if edge[2] == -1 {
                edge[2] = 2_000_000_000;
            }
        }
    }

    pub fn modified_graph_edges(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut graph = vec![vec![0; n]; n];
        for edge in edges.clone() {
            let (x, y, w) = (edge[0], edge[1], edge[2]);
            graph[x as usize][y as usize] = w;
            graph[y as usize][x as usize] = w;
        }

        let mut edges = edges;

        match Solution::get_shortest_path(n, &graph, source as usize, destination as usize) {
            Some(distance) if distance < target => {
                return vec![];
            }
            Some(distance) if distance == target => {
                Solution::fill_with_max(&mut edges);
                return edges;
            }
            _ => {}
        }

        for i in 0..edges.len() {
            if edges[i][2] != -1 {
                continue;
            }
            edges[i][2] = 1;
            let (x, y) = (edges[i][0], edges[i][1]);
            graph[x as usize][y as usize] = 1;
            graph[y as usize][x as usize] = 1;
            let result =
                Solution::get_shortest_path(n, &graph, source as usize, destination as usize);
            match result {
                Some(distance) if distance == target => {
                    Solution::fill_with_max(&mut edges);
                    return edges;
                }
                Some(distance) if distance < target => {
                    edges[i][2] += target - distance;
                    Solution::fill_with_max(&mut edges);
                    return edges;
                }
                _ => {}
            }
        }

        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let n = 5;
        let edges = [[4, 1, -1], [2, 0, -1], [0, 3, -1], [4, 3, -1]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();

        let source = 0;
        let destination = 1;
        let target = 5;
        let result = Solution::modified_graph_edges(n, edges, source, destination, target);
        assert_eq!(result, [[4, 1, 1], [2, 0, 1], [0, 3, 1], [4, 3, 3]]);
    }

    #[test]
    fn leetcode_2() {
        let n = 3;
        let edges = [[0, 1, -1], [0, 2, 5]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();

        let source = 0;
        let destination = 2;
        let target = 6;
        let result = Solution::modified_graph_edges(n, edges, source, destination, target);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn leetcode_3() {
        let n = 4;
        let edges = [[0, 1, -1], [1, 2, -1], [3, 1, -1], [3, 0, 2], [0, 2, 5]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        let source = 2;
        let destination = 3;
        let target = 8;
        let result = Solution::modified_graph_edges(n, edges, source, destination, target);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn leetcode_4() {
        let n = 5;
        let edges = [
            [1, 3, 10],
            [4, 2, -1],
            [0, 3, 7],
            [4, 0, 7],
            [3, 2, -1],
            [1, 4, 5],
            [2, 0, 8],
            [1, 0, 3],
            [1, 2, 5],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();
        let source = 3;
        let destination = 4;
        let target = 11;
        let result = Solution::modified_graph_edges(n, edges, source, destination, target);
        assert_eq!(
            result,
            [
                [1, 3, 10],
                [4, 2, 1],
                [0, 3, 7],
                [4, 0, 7],
                [3, 2, 10],
                [1, 4, 5],
                [2, 0, 8],
                [1, 0, 3],
                [1, 2, 5]
            ]
        );
    }
}

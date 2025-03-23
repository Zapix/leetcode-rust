use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

const MODULO: i32 = 1_000_000_007;

struct Graph {
    data: HashMap<usize, HashMap<usize, i32>>
}

impl Graph {
    fn new(n: usize) -> Self {
        let data = HashMap::from_iter((0..n).map(|x| (x, HashMap::new())));
        Self {
            data
        }
    }

    fn build(n: usize, edges: &Vec<Vec<i32>>) -> Self {
        let mut graph = Self::new(n);
        for edge in edges {
            graph.add_edge(edge);
        }
        graph
    }

    fn add_edge(&mut self, edge: &Vec<i32>) {
        let x = edge[0] as usize;
        let y = edge[1] as usize;
        let v = edge[2];
        let entry = self.data.get_mut(&x).unwrap();
        entry.insert(y, v);
        let entry = self.data.get_mut(&y).unwrap();
        entry.insert(x, v);
    }

    fn add_connected_nodes(&self, x: usize) -> &HashMap<usize, i32> {
        self.data.get(&x).unwrap().into()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

struct Solution;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let graph = Graph::build(n as usize, &roads);
        let mut min_paths = vec![None; graph.len()];
        min_paths[0] = Some(0);
        let mut count_paths = vec![0; graph.len()];
        count_paths[0] = 1;
        let mut que = BinaryHeap::new();
        que.push((Reverse(0), 0));

        while let Some((Reverse(time), node)) = que.pop() {
            match min_paths[node] {
                Some(x) if x < time => {
                    continue;
                },
                _ => {}
            }
            for (next_node, road_time) in graph.add_connected_nodes(node) {
                min_paths[*next_node] = match min_paths[*next_node] {
                    None => {
                        count_paths[*next_node] = count_paths[node];
                        que.push((Reverse(time + *road_time), *next_node));
                        Some(time + road_time)
                    },
                    Some(x) if x > time + *road_time => {
                        count_paths[*next_node] = count_paths[node];
                        que.push((Reverse(time + *road_time), *next_node));
                        Some(time + road_time)
                    }
                    Some(x) if x == time + *road_time => {
                        count_paths[*next_node] = (count_paths[*next_node] + count_paths[node]) % MODULO;
                        Some(x)
                    },
                    Some(x) => Some(x)
                };
            }

        }

        *count_paths.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 7;
        let roads = vec![
            vec![0,6,7],
            vec![0,1,2],
            vec![1,2,3],
            vec![1,3,3],
            vec![6,3,3],
            vec![3,5,1],
            vec![6,5,1],
            vec![2,5,1],
            vec![0,4,5],
            vec![4,6,2]
        ];
        assert_eq!(Solution::count_paths(n, roads), 4);
    }

    #[test]
    fn test_example_2() {
        let n = 2;
        let roads = vec![
            vec![1, 0, 10] 
        ];
        assert_eq!(Solution::count_paths(n, roads), 1);
    }
}
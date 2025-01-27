use std::collections::{HashSet, VecDeque};
type Graph = Vec<Vec<bool>>;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn build_graph(num_courses: usize, prerequisites: &Vec<Vec<i32>>) -> Graph {
        let mut graph = vec![vec![false; num_courses]; num_courses];
        for edge in prerequisites {
            graph[edge[0] as usize][edge[1] as usize] = true;
        }
        for i in 0..num_courses {
            let mut que = VecDeque::new();
            que.push_back(i);
            let mut visited = HashSet::new();
            while let Some(node) = que.pop_front() {
                if visited.contains(&node) {
                    continue;
                }
                graph[i][node] = true;
                for j in 0..num_courses {
                    if graph[node][j] {
                        que.push_back(j);
                    }
                }
                visited.insert(node);
            }
        }
        graph
    }

    fn is_prerequisite(start: usize, end: usize, graph: &Graph) -> bool {
        graph[start][end]
    }

    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let graph = Solution::build_graph(num_courses as usize, &prerequisites);

        queries
            .iter()
            .map(|x| Solution::is_prerequisite(x[0] as usize, x[1] as usize, &graph))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let queries = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(
            Solution::check_if_prerequisite(num_courses, prerequisites, queries),
            vec![false, true]
        );
    }

    #[test]
    fn test_example_2() {
        let num_courses = 2;
        let prerequisites = vec![];
        let queries = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(
            Solution::check_if_prerequisite(num_courses, prerequisites, queries),
            vec![false, false]
        );
    }

    #[test]
    fn test_example_3() {
        let num_courses = 3;
        let prerequisites = vec![vec![1, 2], vec![1, 0], vec![2, 0]];
        let queries = vec![vec![1, 0], vec![1, 2]];
        assert_eq!(
            Solution::check_if_prerequisite(num_courses, prerequisites, queries),
            vec![true, true]
        );
    }
}

use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn dfs(i: usize, visited: &mut HashSet<usize>, in_cycle: &mut Vec<bool>, graph: &Vec<Vec<i32>>) -> bool {
        if in_cycle[i] {
            return false;
        }
        if visited.contains(&i) {
            return true;
        }
        in_cycle[i] = true;
        visited.insert(i);
        for j in graph[i].iter() {
            if !Solution::dfs(*j as usize, visited, in_cycle, graph) {
                return false;
            }
        }
        in_cycle[i] = false;
        return true;
    }

    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut in_cycle = vec![false; graph.len()];
        let mut visited = HashSet::new();
        for i in 0..graph.len() {
            Solution::dfs(i, &mut visited, &mut in_cycle, &graph);
        }

        in_cycle.iter().enumerate().filter(|x| !x.1).map(|x| x.0 as i32).collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eventual_safe_nodes_example1() {
        let graph = vec![vec![1, 2], vec![2, 3], vec![5], vec![0], vec![5], vec![], vec![]];
        let result = Solution::eventual_safe_nodes(graph);
        assert_eq!(result, vec![2, 4, 5, 6]);
    }

    #[test]
    fn test_eventual_safe_nodes_example2() {
        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        let result = Solution::eventual_safe_nodes(graph);
        assert_eq!(result, vec![4]);
    }
}

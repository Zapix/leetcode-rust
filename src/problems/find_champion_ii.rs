use std::collections::{HashMap, HashSet, VecDeque};
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_winner(team: i32, n: i32, graph: &HashMap<i32, Vec<i32>>) -> bool {
        let mut visited = HashSet::new();
        let mut que = VecDeque::new();
        que.push_back(team);

        while let Some(team) = que.pop_front() {
            if visited.contains(&team) {
                continue;
            }
            for next_team in graph.get(&team).unwrap_or(&vec![]) {
                que.push_back(*next_team)
            }
            visited.insert(team);
        }
        visited.len() == n as usize
    }
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut loosers = HashSet::new();
        let mut graph = HashMap::new();

        for edge in edges {
            let (winner, looser) = (edge[0], edge[1]);
            let entry = graph.entry(winner).or_insert(vec![]);
            entry.push(looser);
            loosers.insert(looser);
        }

        let mut result: Option<i32> = None;

        for i in 0..n {
            if loosers.contains(&i) {
                continue;
            }
            if Solution::is_winner(i, n, &graph) {
                if result.is_some() {
                    result = None;
                    break;
                } else {
                    result = Some(i);
                }
            }
        }

        result.unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let result = Solution::find_champion(n, edges);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_example_2() {
        let n = 4;
        let edges = vec![vec![0, 2], vec![1, 3], vec![1, 2]];
        let result = Solution::find_champion(n, edges);
        assert_eq!(result, -1);
    }
}

use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_min_distance(roads: &HashMap<i32, Vec<i32>>, n: i32) -> i32 {
        let mut dp = vec![0i32; n as usize];

        for current_node in (0..(n - 1)).rev() {
            let mut min_distance = n;
            for neighbor in roads.get(&current_node).unwrap() {
                min_distance = min_distance.min(dp[*neighbor as usize] + 1);
            }
            dp[current_node as usize] = min_distance
        }
        dp[0]
    }
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut roads = HashMap::new();
        for i in 0..n {
            roads.insert(i, vec![i + 1]);
        }
        let mut result = vec![];
        for query in queries {
            let entry = roads.entry(query[0]).or_insert(vec![]);
            entry.push(query[1]);

            result.push(Solution::find_min_distance(&roads, n))
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 5;
        let queries = vec![vec![2, 4], vec![0, 2], vec![0, 4]];
        let expected = vec![3, 2, 1];
        assert_eq!(
            Solution::shortest_distance_after_queries(n, queries),
            expected
        );
    }

    #[test]
    fn test_example_2() {
        let n = 4;
        let queries = vec![vec![0, 3], vec![0, 2]];
        let expected = vec![1, 1];
        assert_eq!(
            Solution::shortest_distance_after_queries(n, queries),
            expected
        );
    }

    #[test]
    fn test_example_3() {
        let n = 6;
        let queries = vec![vec![1, 4], vec![0, 2]];
        let expected = vec![3, 3];
        assert_eq!(
            Solution::shortest_distance_after_queries(n, queries),
            expected
        );
    }
}

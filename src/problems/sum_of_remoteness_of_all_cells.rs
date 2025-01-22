use std::collections::{HashSet, VecDeque};
use std::convert::{TryFrom, From};

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum_remoteness(grid: Vec<Vec<i32>>) -> i64 {
        let mut all_values = grid.iter().flatten().filter(|x| **x > -1).map(|x| i64::from(*x)).sum::<i64>();
        let mut result = 0i64;

        let mut visited = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if visited.contains(&(i,j)) || grid[i][j] == -1 {
                    continue;
                }
                let mut island_sum = 0i64;
                let mut count = 0i64;
                let mut que = VecDeque::new();
                que.push_back((i, j));
                while let Some((i, j)) = que.pop_front() {
                    if visited.contains(&(i, j)) || grid[i][j] == -1 {
                        continue;
                    }
                    count += 1;
                    island_sum += i64::from(grid[i][j]);
                    for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        match (usize::try_from(i as i32 + di), usize::try_from(j as i32 + dj)) {
                            (Ok(i), Ok(j)) if i < grid.len() && j < grid[0].len() => {
                                que.push_back((i, j));
                            },
                            _ => {}
                        }
                    }
                    visited.insert((i, j));
                }
                result += (all_values - island_sum) * count;
            }
        }

        result 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_remoteness_example1() {
        let grid = vec![vec![-1, 1, -1], vec![5, -1, 4], vec![-1, 3, -1]];
        assert_eq!(Solution::sum_remoteness(grid), 39);
    }

    #[test]
    fn test_sum_remoteness_example2() {
        let grid = vec![vec![-1, 3, 4], vec![-1, -1, -1], vec![3, -1, -1]];
        assert_eq!(Solution::sum_remoteness(grid), 13);
    }

    #[test]
    fn test_sum_remoteness_example3() {
        let grid = vec![vec![1]];
        assert_eq!(Solution::sum_remoteness(grid), 0);
    }
}

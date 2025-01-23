use std::collections::{VecDeque, HashSet};

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut total_count = 0;
        let mut visited = HashSet::new();
        let mut visited_row = HashSet::new();
        let mut visited_col = HashSet::new();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if visited.contains(&(i, j)) || grid[i][j] == 0 {
                    continue;
                }
                let mut current_count = 0;
                let mut que = VecDeque::new();
                que.push_back((i, j));
                while let Some(coord) = que.pop_front() {
                    if visited.contains(&coord) {
                        continue;
                    }

                    if !visited_row.contains(&coord.0) {
                        for dj in 0..grid[0].len() {
                            if grid[coord.0][dj] == 1 {
                                que.push_back((coord.0, dj));
                            }
                        }
                        visited_row.insert(coord.0);
                    }

                    if !visited_col.contains(&coord.1) {
                        for di in 0..grid.len() {
                            if grid[di][coord.1] == 1{
                                que.push_back((di, coord.1));
                            }
                        }
                        visited_col.insert(coord.1);
                    }

                    current_count += 1;
                    visited.insert(coord);
                }
                total_count += if current_count > 1 { current_count } else { 0 };
            }
        }

        total_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(Solution::count_servers(grid), 0);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec![1, 0], vec![1, 1]];
        assert_eq!(Solution::count_servers(grid), 3);
    }

    #[test]
    fn test_example_3() {
        let grid = vec![vec![1, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]];
        assert_eq!(Solution::count_servers(grid), 4);
    }

    #[test]
    fn test_empty_grid() {
        let grid: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::count_servers(grid), 0);
    }

    #[test]
    fn test_no_servers() {
        let grid = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::count_servers(grid), 0);
    }

    #[test]
    fn test_all_servers() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::count_servers(grid), 4);
    }
}

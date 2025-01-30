use std::collections::{HashSet, VecDeque};
use std::convert::TryFrom;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = HashSet::new();
        let mut result = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 || visited.contains(&(i, j)) {
                    continue;
                }
                let mut value = 0;
                let mut que = VecDeque::new();
                que.push_back((i, j));
                while let Some((i, j)) = que.pop_front() {
                    if grid[i][j] == 0 || visited.contains(&(i, j)) {
                        continue;
                    }
                    value += grid[i][j];
                    for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        match (
                            usize::try_from(i as i32 + di),
                            usize::try_from(j as i32 + dj),
                        ) {
                            (Ok(i), Ok(j)) if i < grid.len() && j < grid[0].len() => {
                                que.push_back((i, j))
                            }
                            _ => {}
                        }
                    }
                    visited.insert((i, j));
                }
                result = result.max(value);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec![0, 2, 1, 0],
            vec![4, 0, 0, 3],
            vec![1, 0, 0, 4],
            vec![0, 3, 2, 0],
        ];
        assert_eq!(Solution::find_max_fish(grid), 7);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1],
        ];
        assert_eq!(Solution::find_max_fish(grid), 1);
    }
}

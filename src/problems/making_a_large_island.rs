use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut island_grid = vec![vec![0; grid[0].len()]; grid.len()];
        let mut visited = HashSet::new();
        let mut island = 1;
        let mut island_sizes = HashMap::new();
        island_sizes.insert(0, 0);

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 || visited.contains(&(i, j)) {
                    continue;
                }
                let mut size = 0;
                let mut que = VecDeque::new();
                que.push_back((i, j));
                while let Some((i, j)) = que.pop_front() {
                    if grid[i][j] == 0 || visited.contains(&(i, j)) {
                        continue;
                    }
                    size += 1;
                    island_grid[i][j] = island;
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
                island_sizes.insert(island, size);
                island += 1;
            }
        }
        let mut result = *island_sizes.values().max().unwrap_or(&0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if island_grid[i][j] != 0 {
                    continue;
                }
                let mut islands_to_connect = HashSet::new();
                for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    match (
                        usize::try_from(i as i32 + di),
                        usize::try_from(j as i32 + dj),
                    ) {
                        (Ok(i), Ok(j)) if i < grid.len() && j < grid[0].len() => {
                            islands_to_connect.insert(island_grid[i][j]);
                        }
                        _ => {}
                    }
                }
                let value = islands_to_connect
                    .iter()
                    .map(|x| *island_sizes.get(x).unwrap_or(&0))
                    .sum::<i32>()
                    + 1;
                result = result.max(value)
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
        let grid = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(Solution::largest_island(grid), 3);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec![1, 1], vec![1, 0]];
        assert_eq!(Solution::largest_island(grid), 4);
    }

    #[test]
    fn test_example_3() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::largest_island(grid), 4);
    }
}

use std::collections::{BinaryHeap, HashSet};
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let n = grid1.len();
        let m = grid1[0].len();

        let mut grid1 = grid1;
        let mut grid2 = grid2;
        let mut result = 0;

        let mut island_mark = 2;
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();

        for i in 0..n {
            for j in 0..m {
                if grid1[i][j] == 1 && !visited.contains(&(i, j)) {
                    grid1[i][j] = island_mark;
                    queue.push((i, j));
                    visited.insert((i, j));
                    while !queue.is_empty() {
                        let (x, y) = queue.pop().unwrap();
                        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
                                let nx = nx as usize;
                                let ny = ny as usize;
                                if grid1[nx][ny] == 1 && !visited.contains(&(nx, ny)) {
                                    queue.push((nx, ny));
                                    visited.insert((nx, ny));
                                    grid1[nx][ny] = island_mark;
                                }
                            }
                        }
                    }
                    island_mark += 1;
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if grid2[i][j] == 1 && grid1[i][j] > 1 {
                    let island_mark = grid1[i][j];
                    let mut is_sub_island = true;
                    let mut queue = vec![(i, j)];
                    while !queue.is_empty() {
                        let (x, y) = queue.pop().unwrap();
                        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
                                let nx = nx as usize;
                                let ny = ny as usize;
                                if grid2[nx][ny] == 1 && grid1[nx][ny] == island_mark {
                                    queue.push((nx, ny));
                                    grid2[nx][ny] = 0;
                                } else if grid2[nx][ny] == 1 {
                                    is_sub_island = false;
                                }
                            }
                        }
                    }
                    if is_sub_island {
                        result += 1;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let grid1 = vec![vec![1, 1, 0], vec![0, 1, 1], vec![0, 0, 0]];
        let grid2 = vec![vec![1, 1, 0], vec![0, 0, 1], vec![0, 0, 0]];
        let result = Solution::count_sub_islands(grid1, grid2);
        assert_eq!(result, 2);
    }

    #[test]
    fn leetcode_2() {
        let grid1 = vec![
            vec![1, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
        ];
        let grid2 = vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 1, 0, 1],
        ];
        let result = Solution::count_sub_islands(grid1, grid2);
        assert_eq!(result, 3);
    }
}

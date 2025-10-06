use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        let mut heap = BinaryHeap::new(); // Min-heap to store (time, x, y)
        let mut visited = HashSet::new(); //
        let mut current_time = 0;
        heap.push((Reverse(grid[0][0]), 0i32, 0i32));
        while let Some((Reverse(time), x, y)) = heap.pop() {
            current_time = current_time.max(time);
            if visited.contains(&(x, y)) {
                continue;
            }

            if x == n - 1 && y == n - 1 {
                break;
            }

            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0
                    && nx < grid.len() as i32
                    && ny >= 0
                    && ny < grid.len() as i32
                    && !visited.contains(&(nx, ny))
                {
                    heap.push((Reverse(grid[nx as usize][ny as usize]), nx, ny));
                }
            }
            visited.insert((x, y));
        }
        current_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swim_in_water_1() {
        let grid = vec![vec![0, 2], vec![1, 3]];
        assert_eq!(Solution::swim_in_water(grid), 3);
    }

    #[test]
    fn test_swim_in_water_2() {
        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        assert_eq!(Solution::swim_in_water(grid), 16);
    }
}

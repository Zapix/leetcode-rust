use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

#[allow(dead_code)]
struct Solution;

#[derive(Eq, PartialEq)]
struct HeapItem(i32, i32, (usize, usize));

impl PartialOrd<Self> for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            self.1.partial_cmp(&other.1)
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            self.1.cmp(&other.1)
        } else {
            self.0.cmp(&other.0)
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(HeapItem(0, 0, (0, 0))));

        let mut visted = HashSet::new();

        let mut removed_count = -1;
        while let Some(Reverse(HeapItem(removed, cnt, (x, y)))) = heap.pop() {
            if x == grid.len() - 1 && y == grid[0].len() - 1 {
                removed_count = removed;
                break;
            }
            if visted.contains(&(x, y)) {
                continue;
            }
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || ny < 0 || nx as usize >= grid.len() || ny as usize >= grid[0].len() {
                    continue;
                }
                heap.push(Reverse(HeapItem(
                    removed + grid[nx as usize][ny as usize],
                    cnt + 1,
                    (nx as usize, ny as usize),
                )));
            }
            visted.insert((x, y));
        }

        removed_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::minimum_obstacles(grid), 2);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 0],
        ];
        assert_eq!(Solution::minimum_obstacles(grid), 0);
    }
}

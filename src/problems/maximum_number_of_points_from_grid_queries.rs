use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut find_values = HashMap::new();
        let mut visited = HashSet::new();
        let mut que = BinaryHeap::new();
        que.push((Reverse(grid[0][0]), grid[0][0], (0usize, 0usize)));
        while let Some((Reverse(_grid_value), current, (i, j))) = que.pop() {
            if visited.contains(&(i, j)) {
                continue;
            }
            visited.insert((i, j));
            let entry = find_values.entry(current).or_insert(0);
            *entry += 1;

            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                match (
                    usize::try_from(i as i32 + dx),
                    usize::try_from(j as i32 + dy),
                ) {
                    (Ok(x), Ok(y)) if x < grid.len() && y < grid[0].len() => {
                        que.push((Reverse(grid[x][y]), current.max(grid[x][y]), (x, y)));
                    }
                    _ => {}
                }
            }
        }
        let values = {
            let mut x = find_values.iter().map(|x| (*x.0, *x.1)).collect::<Vec<_>>();
            x.sort_by_key(|x| x.0);
            let mut sum = 0;
            for i in 0..x.len() {
                sum += x[i].1;
                x[i].1 = sum;
            }
            x
        };
        println!("{:?}", values);
        queries
            .iter()
            .map(|x| match values.binary_search_by_key(x, |&y| y.0) {
                Ok(0) => 0,
                Ok(i) => values[i - 1].1,
                Err(0) => 0,
                Err(i) => values[i - 1].1,
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]];
        let queries = vec![5, 6, 2];
        assert_eq!(Solution::max_points(grid, queries), vec![5, 8, 1]);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec![5, 2, 1], vec![1, 1, 2]];
        let queries = vec![3];
        assert_eq!(Solution::max_points(grid, queries), vec![0]);
    }
}

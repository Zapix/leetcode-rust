use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;
use std::convert::TryFrom;

#[allow(dead_code)]
struct Solution;


#[allow(dead_code)]
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let delta_map = HashMap::from([
            (1, (0, 1)),
            (2, (0, -1)),
            (3, (1, 0)),
            (4, (-1, 0))
        ]);

        let mut visited = HashSet::new();
        let mut que = BinaryHeap::new();
        que.push((Reverse(0), (0, 0)));
        while let Some((Reverse(cost), position)) = que.pop() {
            if position.0 == grid.len() - 1 && position.1 == grid[0].len() - 1 {
                return cost;
            }
            if visited.contains(&position) {
                continue
            }
            for (dir, delta) in delta_map.iter() {
                match (usize::try_from(position.0 as i32 + delta.0), usize::try_from(position.1 as i32 + delta.1)) {
                    (Ok(x), Ok(y)) if x < grid.len() && y < grid[0].len() => {
                        que.push((
                            Reverse(cost + if grid[position.0][position.1] == *dir { 0 } else { 1 }),
                            (x, y),
                        ));
                    },
                    _ => {},
                }
            }
            visited.insert(position);
        }

        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_example1() {
        let grid = vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2]
        ];
        assert_eq!(Solution::min_cost(grid), 3);
    }

    #[test]
    fn test_min_cost_example2() {
        let grid = vec![
            vec![1, 1, 3],
            vec![3, 2, 2],
            vec![1, 1, 4]
        ];
        assert_eq!(Solution::min_cost(grid), 0);
    }

    #[test]
    fn test_min_cost_example3() {
        let grid = vec![
            vec![1, 2],
            vec![4, 3]
        ];
        assert_eq!(Solution::min_cost(grid), 1);
    }
}



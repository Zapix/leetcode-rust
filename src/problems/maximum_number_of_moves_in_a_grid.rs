#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut column = vec![0; grid.len()];

        for i in (0..grid[0].len() - 1).rev() {
            let mut next_column = vec![0; grid.len()];
            for j in 0..grid.len() {
                let deltas = [0, 1, -1];
                for delta in deltas.iter() {
                    let next_j = j as i32 + delta;
                    if next_j >= 0
                        && next_j < grid.len() as i32
                        && grid[j][i] < grid[next_j as usize][i + 1]
                    {
                        next_column[j] = next_column[j].max(column[next_j as usize] + 1);
                    }
                }
            }

            column = next_column;
        }

        *column.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_moves_case1() {
        let grid = vec![
            vec![2, 4, 3, 5],
            vec![5, 4, 9, 3],
            vec![3, 4, 2, 11],
            vec![10, 9, 13, 15],
        ];
        assert_eq!(Solution::max_moves(grid), 3);
    }

    #[test]
    fn test_max_moves_case2() {
        let grid = vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]];
        assert_eq!(Solution::max_moves(grid), 0);
    }

    fn test_max_moves_case3() {
        let grid = vec![
            vec![2, 4, 3, 5, 1, 2],
            vec![5, 4, 9, 3, 1, 2],
            vec![3, 4, 2, 11, 1, 3],
            vec![10, 9, 13, 15, 1, 2],
        ];
        assert_eq!(Solution::max_moves(grid), 3);
    }
}

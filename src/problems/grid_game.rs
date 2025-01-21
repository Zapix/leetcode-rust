#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut first_row_sum = grid[0].iter().map(|x| *x as i64).sum::<i64>();
        let mut second_row_sum = 0i64;
        let mut minimum_row_sum = i64::MAX;
        for i in 0..grid[0].len() {
            first_row_sum -= grid[0][i] as i64;
            minimum_row_sum = minimum_row_sum.min(first_row_sum.max(second_row_sum));
            second_row_sum += grid[1][i] as i64;
        }
        minimum_row_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let grid = vec![vec![2, 5, 4], vec![1, 5, 1]];
        assert_eq!(Solution::grid_game(grid), 4);
    }

    #[test]
    fn test_example2() {
        let grid = vec![vec![3, 3, 1], vec![8, 5, 2]];
        assert_eq!(Solution::grid_game(grid), 4);
    }

    #[test]
    fn test_example3() {
        let grid = vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]];
        assert_eq!(Solution::grid_game(grid), 7);
    }
}

struct Solution;

impl Solution {
    fn can_be_unified(grid: &Vec<Vec<i32>>, x: i32, rest: i32) -> bool {
        grid.iter().map(|item| item.iter()).flatten().all(|val| *val % x == rest)
    }

    fn as_vector(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut grid_vec = grid.iter().map(|item| item.iter()).flatten().map(|x| *x).collect::<Vec<_>>();
        grid_vec.sort();
        grid_vec
    }

    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32{
        let rest = grid[0][0] % x;
        if !Solution::can_be_unified(&grid, x, rest) {
            return -1;
        }
        let grid_vec = Solution::as_vector(&grid);
        let median = grid_vec[grid_vec.len() / 2];
        grid_vec.iter().fold(0, |acc, val| {
            let result = acc + ((median - *val).abs() / x);
            result
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations_example1() {
        let grid = vec![vec![2, 4], vec![6, 8]];
        let x = 2;
        let result = Solution::min_operations(grid, x);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_min_operations_example2() {
        let grid = vec![vec![1, 5], vec![2, 3]];
        let x = 1;
        let result = Solution::min_operations(grid, x);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_min_operations_example3() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let x = 2;
        let result = Solution::min_operations(grid, x);
        assert_eq!(result, -1);
    }
}

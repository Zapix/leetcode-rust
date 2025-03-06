#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_missing_and_repeated_values(mut grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut repeated = 0;
        let mut current_sum = 0;
        let k = n * n;
        let real_sum =  (k * (k + 1) / 2) as i32;

        let mut i = 0;
        while i < n.pow(2) {
            let row = i / n;
            let col = i % n;
            if repeated != 0 {
                current_sum += grid[row][col];
                i += 1;
                continue;
            }

            if grid[row][col] == (i + 1) as i32 {
                current_sum += grid[row][col];
                i += 1;
                continue;
            } else {
                let val = grid[row][col] - 1;
                let val_usize = val as usize;
                let n_row = val_usize / n;
                let n_col = val_usize % n;
                if grid[n_row][n_col] == val + 1 {
                    repeated = val + 1;
                } else {
                    grid[row][col] = grid[n_row][n_col];
                    grid[n_row][n_col] = val + 1;
                }
            }
        }
        vec![repeated, real_sum - current_sum + repeated]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_and_repeated_values_example1() {
        let grid = vec![vec![1, 3], vec![2, 2]];
        let result = Solution::find_missing_and_repeated_values(grid);
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_find_missing_and_repeated_values_example2() {
        let grid = vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]];
        let result = Solution::find_missing_and_repeated_values(grid);
        assert_eq!(result, vec![9, 5]);
    }
}

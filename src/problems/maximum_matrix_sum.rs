#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut result = 0i64;
        let mut odd_negative = false;
        let mut abs_min = i32::MAX;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                result += if matrix[i][j] < 0 {
                    odd_negative = !odd_negative;
                    abs_min = abs_min.min(matrix[i][j] * -1);
                    (matrix[i][j] * -1) as i64
                } else {
                    abs_min = abs_min.min(matrix[i][j]);
                    matrix[i][j] as i64
                };
            }
        }

        if odd_negative {
            result -= 2 * abs_min as i64
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let matrix = vec![vec![1, -1], vec![-1, 1]];
        assert_eq!(Solution::max_matrix_sum(matrix), 4);
    }

    #[test]
    fn test_example_2() {
        let matrix = vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]];
        assert_eq!(Solution::max_matrix_sum(matrix), 16);
    }

    #[test]
    fn test_example_3() {
        let matrix = vec![vec![-1, 2, -3], vec![1, 2, -3], vec![1, 2, -3]];
        assert_eq!(Solution::max_matrix_sum(matrix), 18);
    }

    #[test]
    fn test_example_4() {
        let matrix = vec![vec![-1, 2, -3], vec![1, 2, -3], vec![1, -2, -3]];
        assert_eq!(Solution::max_matrix_sum(matrix), 16);
    }
}

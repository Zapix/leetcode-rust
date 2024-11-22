use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut pattern_frequency = HashMap::new();

        for row in matrix {
            let first_value = row.first().unwrap();
            let pattern = row
                .iter()
                .map(|x| if x == first_value { "T" } else { "F" })
                .collect::<Vec<_>>()
                .join("");
            pattern_frequency
                .entry(pattern)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        *pattern_frequency.values().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let matrix = vec![vec![0, 1], vec![1, 0]];
        let result = Solution::max_equal_rows_after_flips(matrix);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_2() {
        let matrix = vec![vec![0, 1], vec![1, 1]];
        let result = Solution::max_equal_rows_after_flips(matrix);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_example_3() {
        let matrix = vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]];
        let result = Solution::max_equal_rows_after_flips(matrix);
        assert_eq!(result, 2);
    }
}

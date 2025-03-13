struct Solution;

impl Solution {
    fn can_form_zero_array(nums: &Vec<i32>, queries: &Vec<Vec<i32>>, k: usize) -> bool {
        let mut diff_array = vec![0; nums.len() + 1];
        for i in 0..k {
            let (start, end, val) = (queries[i][0], queries[i][1], queries[i][2]);
            diff_array[start as usize] += val;
            diff_array[end as usize + 1] -= val;
        }
        let mut current = 0;
        for i in 0..nums.len() {
            current += diff_array[i];
            if current < nums[i] {
                return false;
            }
        }
        true
    }

    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut left = 0;
        let mut right = queries.len() as i32;

        if !Self::can_form_zero_array(&nums, &queries, right as usize) {
            return -1;
        }

        while left <= right {
            let middle = left + (right - left) / 2;
            if Self::can_form_zero_array(&nums, &queries, middle as usize) {
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 0, 2];
        let queries = vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]];
        assert_eq!(Solution::min_zero_array(nums, queries), 2);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![4, 3, 2, 1];
        let queries = vec![vec![1, 3, 2], vec![0, 2, 1]];
        assert_eq!(Solution::min_zero_array(nums, queries), -1);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![0];
        let queries = vec![
            vec![0, 0, 2],
            vec![0, 0, 4],
            vec![0, 0, 4],
            vec![0, 0, 3],
            vec![0, 0, 5],
        ];
        assert_eq!(Solution::min_zero_array(nums, queries), 0);
    }
}

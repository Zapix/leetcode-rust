struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut left_bound = -1;
        let mut count = 0i64;
        let mut min_position = -1;
        let mut max_position = -1;
        for i in 0..nums.len() {
            if nums[i] < min_k || nums[i] > max_k {
                left_bound = i as i32;
            }
            if nums[i] == min_k {
                min_position = i as i32;
            }
            if nums[i] == max_k {
                max_position = i as i32;
            }
            count += (0.max(min_position.min(max_position) - left_bound)) as i64
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 3, 5, 2, 7, 5];
        let min_k = 1;
        let max_k = 5;
        let result = Solution::count_subarrays(nums, min_k, max_k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 1, 1, 1];
        let min_k = 1;
        let max_k = 1;
        let result = Solution::count_subarrays(nums, min_k, max_k);
        assert_eq!(result, 10);
    }
}

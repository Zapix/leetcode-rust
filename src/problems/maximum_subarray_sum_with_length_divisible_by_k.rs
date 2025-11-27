struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut max_sum = i64::MIN;
        let mut prefix_sum = 0i64;
        let k = k as usize;
        let mut k_sum = vec![i64::MAX / 2; k];
        k_sum[k - 1] = 0;
        for i in 0..nums.len() {
            prefix_sum += nums[i] as i64;
            let idx = i % k;
            max_sum = max_sum.max(prefix_sum - k_sum[idx]);
            k_sum[idx] = k_sum[idx].min(prefix_sum);
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_subarray_sum(vec![1, 2], 1), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_subarray_sum(vec![-1, -2, -3, -4, -5], 4), -10);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::max_subarray_sum(vec![-5, 1, 2, -3, 4], 2), 4);
    }
}

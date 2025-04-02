struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max_values_after = vec![0;nums.len()];
        max_values_after[nums.len() - 1] = nums[nums.len() - 1];
        for i in (0..nums.len() - 1).rev() {
            max_values_after[i] = nums[i].max(max_values_after[i + 1]);
        }
        let mut max_value_before = nums[0];
        let mut max_value = 0i64;

        for i in 1..(nums.len() - 1) {
            max_value = max_value.max((max_value_before as i64 - nums[i] as i64) * max_values_after[i + 1] as i64);
            max_value_before = max_value_before.max(nums[i])
        }

        max_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![12, 6, 1, 2, 7];
        let result = Solution::maximum_triplet_value(nums);
        assert_eq!(result, 77);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 10, 3, 4, 19];
        let result = Solution::maximum_triplet_value(nums);
        assert_eq!(result, 133);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 3];
        let result = Solution::maximum_triplet_value(nums);
        assert_eq!(result, 0);
    }
}



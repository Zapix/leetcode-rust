struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 2..nums.len() {
            if nums[i - 2] == 0 {
                count += 1;
                nums[i - 2] = nums[i - 2] ^ 1;
                nums[i - 1] = nums[i - 1] ^ 1;
                nums[i] = nums[i] ^ 1;
            }
        }
        if nums.len() as i32 == nums.into_iter().sum::<i32>() {
            count
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![0, 1, 1, 1, 0, 0];
        let result = Solution::min_operations(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0, 1, 1, 1];
        let result = Solution::min_operations(nums);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_all_zeros() {
        let nums = vec![0, 0, 0, 0];
        let result = Solution::min_operations(nums);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_all_ones() {
        let nums = vec![1, 1, 1, 1];
        let result = Solution::min_operations(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mixed_case() {
        let nums = vec![1, 0, 1, 0, 1];
        let result = Solution::min_operations(nums);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_length_21_divisible_zeros() {
        let nums = vec![1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1];
        let result = Solution::min_operations(nums);
        assert_eq!(result, 4);
    }
}

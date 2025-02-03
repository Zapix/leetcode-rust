#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut max_value = 1;
        let mut increasing = 1;
        let mut decreasing = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i-1] {
                increasing += 1;
            } else {
                max_value = increasing.max(max_value);
                increasing = 1;
            }
            if nums[i] < nums[i-1] {
                decreasing += 1;
            } else {
                max_value = decreasing.max(max_value);
                decreasing = 1;
            }
        }
        max_value = max_value.max(increasing);
        max_value = max_value.max(decreasing);
        max_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_monotonic_subarray() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 2, 1]), 3);
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(Solution::longest_monotonic_subarray(vec![5, 4, 3, 2, 1]), 5);
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 2, 2, 3, 4]), 3);
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 1, 1, 1, 1]), 1);
    }
}

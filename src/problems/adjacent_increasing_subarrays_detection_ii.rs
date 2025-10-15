struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut max_count = 0;
        let mut prev_count = 0;
        let mut current_count = 1;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                current_count += 1;
            } else {
                prev_count = current_count;
                current_count = 1;
            }
            let current = (current_count / 2).max(current_count.min(prev_count));
            max_count = max_count.max(current);
        }

        max_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_increasing_subarrays() {
        let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
        assert_eq!(Solution::max_increasing_subarrays(nums), 3);
    }

    #[test]
    fn test_max_increasing_subarrays_1() {
        let nums = vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7];
        assert_eq!(Solution::max_increasing_subarrays(nums), 2);
    }
}

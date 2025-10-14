struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut increasing_count = vec![0; nums.len()];
        increasing_count[0] = 1;
        let mut prev_value = nums[0];
        for (i, &n) in nums.iter().enumerate().skip(1) {
            if n > prev_value {
                increasing_count[i] = increasing_count[i - 1] + 1;
            } else {
                increasing_count[i] = 1;
            }
            if increasing_count[i] >= k && i >= k && increasing_count[i - k] >= k {
                return true;
            }
            prev_value = n;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_increasing_subarrays() {
        let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
        let k = 3;
        assert_eq!(Solution::has_increasing_subarrays(nums, k), true);
    }

    #[test]
    fn test_has_increasing_subarrays_1() {
        let nums = vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7];
        let k = 5;
        assert_eq!(Solution::has_increasing_subarrays(nums, k), false);
    }
}

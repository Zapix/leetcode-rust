use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut repeats = HashMap::new();
        let n = nums.len();
        let mut right = 0;
        let mut count = 0;
        let mut same = 0;
        for left in 0..n {
             while right < n && same < k {
                let val = nums[right];
                same += repeats.get(&val).unwrap_or(&0);
                let entry = repeats.entry(val).or_insert(0);
                *entry += 1;
                right += 1;
             }
             if same >= k {
                count += (n - right + 1) as i64;
             }
            let val = nums[left];
            let entry = repeats.entry(val).or_insert(0);
            *entry -= 1;
            same -= *entry;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_good_example1() {
        let nums = vec![1, 1, 1, 1, 1];
        let k = 10;
        let result = Solution::count_good(nums, k);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_count_good_example2() {
        let nums = vec![3, 1, 4, 3, 2, 2, 4];
        let k = 2;
        let result = Solution::count_good(nums, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_count_good_single_element() {
        let nums = vec![1];
        let k = 1;
        let result = Solution::count_good(nums, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_good_no_good_subarrays() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 10;
        let result = Solution::count_good(nums, k);
        assert_eq!(result, 0);
    }
}

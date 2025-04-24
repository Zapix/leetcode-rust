use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let unique_count = nums
            .iter()
            .fold(HashSet::new(), |mut acc, item| {
                acc.insert(*item);
                acc
            })
            .len();
        let mut right = 0;
        let mut count = 0;
        let mut window = HashMap::new();
        for left in 0..nums.iter().count() {
            if left > 0 {
                let entry = window.entry(&nums[left - 1]).or_insert(1);
                *entry -= 1;
                if *entry == 0 {
                    window.remove(&nums[left - 1]);
                }
            }
            while window.len() < unique_count && right < nums.len() {
                let entry = window.entry(&nums[right]).or_insert(0);
                *entry += 1;
                right += 1;
            }
            if window.len() == unique_count {
                count += nums.len() - right + 1;
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 3, 1, 2, 2];
        let result = Solution::count_complete_subarrays(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![5, 5, 5, 5];
        let result = Solution::count_complete_subarrays(nums);
        assert_eq!(result, 10);
    }
}

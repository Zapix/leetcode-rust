use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let mut longest = 0;
        let mut prefixes = HashMap::new();
        let mut prefix = 0;
        for (i, val) in nums.iter().enumerate() {
            prefix += *val;
            let req_val = prefix - k;
            if req_val == 0 {
                longest = i + 1;
            }
            if let Some(j) = prefixes.get(&req_val) {
                longest = longest.max(i - j);
            }
            if !prefixes.contains_key(&prefix) {
                prefixes.insert(prefix, i);
            }
        }
        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, -1, 5, -2, 3];
        let k = 3;
        let result = Solution::max_sub_array_len(nums, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![-2, -1, 2, 1];
        let k = 1;
        let result = Solution::max_sub_array_len(nums, k);
        assert_eq!(result, 2);
    }
}

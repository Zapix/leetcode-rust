use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut bad_pairs = 0;
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let diff = num - i as i32;
            let good_pairs_count = map.entry(diff).or_insert(0);
            bad_pairs += (i as i32 - *good_pairs_count) as i64;
            *good_pairs_count += 1;
        }
        bad_pairs
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bad_pairs_example1() {
        let nums = vec![4, 1, 3, 3];
        assert_eq!(Solution::count_bad_pairs(nums), 5);
    }

    #[test]
    fn test_count_bad_pairs_example2() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::count_bad_pairs(nums), 0);
    }

    #[test]
    fn test_count_bad_pairs_empty() {
        let nums = vec![];
        assert_eq!(Solution::count_bad_pairs(nums), 0);
    }

    #[test]
    fn test_count_bad_pairs_single_element() {
        let nums = vec![1];
        assert_eq!(Solution::count_bad_pairs(nums), 0);
    }

    #[test]
    fn test_count_bad_pairs_all_same() {
        let nums = vec![2, 2, 2, 2];
        assert_eq!(Solution::count_bad_pairs(nums), 6);
    }
}

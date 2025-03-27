use std::collections::HashMap;

struct Solution;

impl Solution {
    fn dominant(nums: &Vec<i32>) -> (i32, i32) {
        nums.iter()
            .fold(HashMap::new(), |mut acc, x| {
                let entry = acc.entry(x).or_insert(0);
                *entry += 1;
                acc
            })
            .iter()
            .max_by_key(|x| *x.1)
            .map(|x| (**x.0, *x.1))
            .unwrap()
    }
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let (x, right_count) = Solution::dominant(&nums);
        let mut right_count = right_count;
        let mut left_count = 0;
        let mut result = None;
        for (i, val) in nums.iter().enumerate() {
            if *val == x {
                left_count += 1;
                right_count -= 1;
            }
            if left_count * 2 > (i + 1) as i32 && right_count * 2 > (nums.len() - i - 1) as i32 {
                result = Some(i as i32);
                break;
            }
        }
        result.unwrap_or(-1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 2, 2];
        assert_eq!(Solution::minimum_index(nums), 2);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1];
        assert_eq!(Solution::minimum_index(nums), 4);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![3, 3, 3, 3, 7, 2, 2];
        assert_eq!(Solution::minimum_index(nums), -1);
    }
}

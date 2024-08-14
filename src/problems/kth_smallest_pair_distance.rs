use std::collections::{HashMap};
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let min_value = *nums.iter().min().unwrap();
        let max_value = *nums.iter().max().unwrap();
        let mut diffs = (0..(max_value - min_value + 1)).map(|_| 0).collect::<Vec<i32>>();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let diff = (nums[i] - nums[j]).abs() as usize;
                diffs[diff] += 1;
            }
        }
        let mut counter = 0;
        for (i, count) in diffs.iter().enumerate() {
            counter += count;
            if counter >= k {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::smallest_distance_pair(vec![1, 3, 1], 1),
            0
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::smallest_distance_pair(vec![1, 1, 1], 2),
            0
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::smallest_distance_pair(vec![1, 6, 1], 3),
            5
        );
    }
}
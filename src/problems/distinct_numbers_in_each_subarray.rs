use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn distinct_numbers(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut window = HashMap::new();
        for i in 0..k {
            let entry = window.entry(nums[i]).or_insert(0);
            *entry += 1;
        }
        let mut result = vec![];
        result.push(window.len() as i32);
        for i in k..nums.len() {
            let entry = window.entry(nums[i]).or_insert(0);
            *entry += 1;

            let j = i - k;
            let entry = window.entry(nums[j]).or_insert(0);
            *entry -= 1;
            if *entry <= 0 {
                window.remove(&nums[j]);
            }
            result.push(window.len() as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3, 2, 2, 1, 3];
        let k = 3;
        assert_eq!(Solution::distinct_numbers(nums, k), vec![3, 2, 2, 2, 3]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 1, 1, 1, 2, 3, 4];
        let k = 4;
        assert_eq!(Solution::distinct_numbers(nums, k), vec![1, 2, 3, 4]);
    }
}

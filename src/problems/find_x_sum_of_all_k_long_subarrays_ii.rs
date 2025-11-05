use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap};
struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut counter = HashMap::new();
        let mut window = BTreeSet::new();
        let k = k as usize;
        let x = x as usize;

        let mut result = Vec::new();

        for (i, &value) in nums.iter().enumerate() {
            let value = value as usize;
            let ptr = counter.entry(value).or_insert(0);
            if *ptr > 0 {
                window.remove(&Reverse((*ptr, value)));
            }
            *ptr += 1;
            window.insert(Reverse((*ptr, value)));

            if i < k - 1 {
                continue;
            }
            if i >= k {
                let left_value = nums[i - k] as usize;
                let left_ptr = counter.get_mut(&left_value).unwrap();
                window.remove(&Reverse((*left_ptr, left_value)));
                *left_ptr -= 1;
                if *left_ptr > 0 {
                    window.insert(Reverse((*left_ptr, left_value)));
                }
            }

            let value = window
                .iter()
                .take(x)
                .fold(0, |acc, &Reverse((count, num))| acc + (count * num));

            result.push(value as i32);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_1() {
        let nums = vec![1, 1, 2, 2, 3, 4, 2, 3];
        let k = 6;
        let x = 2;
        assert_eq!(Solution::find_x_sum(nums, k, x), vec![6, 10, 12]);
    }
    #[test]
    fn test_example_2() {
        let nums = vec![3, 8, 7, 8, 7, 5];
        let k = 2;
        let x = 2;
        assert_eq!(Solution::find_x_sum(nums, k, x), vec![11, 15, 15, 15, 12]);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![3, 8, 7, 8, 7, 5];
        let k = 1;
        let x = 1;
        assert_eq!(Solution::find_x_sum(nums, k, x), vec![3, 8, 7, 8, 7, 5]);
    }
}

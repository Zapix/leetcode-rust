use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let nums = {
            let mut nums = nums;
            nums.sort_unstable();
            nums
        };
        let counts = nums.iter().fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0i32) += 1;
            acc
        });
        let mut max_frequency = 0;
        for num in (*nums.first().unwrap_or(&0)..(*nums.last().unwrap_or(&0) + 1)) {
            let left = nums.partition_point(|&x| x < num - k);
            let right = nums.partition_point(|&x| x <= num + k);
            let freq = match counts.get(&num) {
                Some(&count) => ((right - left) as i32).min(count + num_operations),
                None => ((right - left) as i32).min(num_operations),
            };
            max_frequency = max_frequency.max(freq as i32);
        }
        max_frequency
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_frequency() {
        let nums = vec![1, 4, 5];
        let k = 1;
        let num_operations = 2;
        assert_eq!(Solution::max_frequency(nums, k, num_operations), 2);
    }

    #[test]
    fn test_max_frequency_1() {
        let nums = vec![5, 11, 20, 20];
        let k = 5;
        let num_operations = 2;
        assert_eq!(Solution::max_frequency(nums, k, num_operations), 2);
    }
}

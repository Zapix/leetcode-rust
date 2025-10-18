use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn max_distince_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq_map = nums.iter().fold(BTreeMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0i32) += 1;
            acc
        });
        let mut count = 0;
        let mut last_used = i32::MIN;
        for (&num, &frec) in freq_map.iter() {
            let mut frec = frec;
            if last_used >= num + k {
                continue;
            }
            if last_used < num - 1 {
                let min_value = (last_used + 1).max(num - k);
                let dec_steps = frec.min((num - min_value).abs());
                count += dec_steps;
                frec -= dec_steps;
                last_used = min_value + dec_steps - 1;
            }
            if frec <= 0 {
                continue;
            }
            if last_used + 1 == num {
                frec -= 1;
                last_used = num;
                count += 1;
            }
            if frec <= 0 {
                continue;
            }
            let dec_steps = frec.min((num + k - last_used).abs());
            println!("dec_steps: {}", dec_steps);
            count += dec_steps;
            last_used += dec_steps;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_max_distinct_elements() {
        let nums = vec![1, 2, 2, 3, 3, 4];
        let k = 2;
        assert_eq!(Solution::max_distince_elements(nums, k), 6);
    }

    #[test]
    pub fn test_max_distinct_elements_1() {
        let nums = vec![4, 4, 4, 4];
        let k = 1;
        assert_eq!(Solution::max_distince_elements(nums, k), 3);
    }

    #[test]
    pub fn test_max_distinct_elements_2() {
        let nums = vec![8, 9, 9, 9, 10, 8, 10, 8];
        let k = 2;
        assert_eq!(Solution::max_distince_elements(nums, k), 7);
    }
}

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = nums.iter().sum::<i32>();
        if sum % 2 == 1 {
            return false;
        }
        let mut dp = HashSet::new();
        dp.insert(0);
        for num in nums {
            let mut next_dp = HashSet::new();
            sum -= num;
            for target in dp {
                let next_target = target + num;
                if next_target.abs() == sum {
                    return true;
                }
                if next_target.abs() < sum {
                    next_dp.insert(next_target.abs());
                }
                let next_target = target - num;
                if next_target.abs() == sum {
                    return true;
                }
                if next_target.abs() < sum {
                    next_dp.insert(next_target.abs());
                }
            }
            dp = next_dp;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 5, 11, 5];
        let result = Solution::can_partition(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2, 3, 5];
        let result = Solution::can_partition(nums);
        assert_eq!(result, false);
    }
}

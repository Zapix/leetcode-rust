struct Solution;
impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let (min_reward, max_reward) = nums.iter().fold((i32::MAX, i32::MIN), |acc, x| {
            (acc.0.min(*x), acc.1.max(*x))
        });
        let mut min_reward = min_reward;
        let mut max_reward = max_reward;
        while min_reward < max_reward {
            let mid_reward = (min_reward + max_reward) / 2;
            let mut possible_thefts = 0;
            let mut index = 0;
            while index < nums.len() {
                index += if nums[index] <= mid_reward {
                    possible_thefts += 1;
                    2
                } else {
                    1
                };
            }

            if possible_thefts >= k {
                max_reward = mid_reward
            } else {
                min_reward = mid_reward + 1
            }
        }
        min_reward
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 3, 5, 9];
        let k = 2;
        assert_eq!(Solution::min_capability(nums, k), 5);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 7, 9, 3, 1];
        let k = 2;
        assert_eq!(Solution::min_capability(nums, k), 2);
    }
}

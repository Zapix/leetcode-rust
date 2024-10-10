#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut result = 0;
        for i in 0..nums.len() {
            if stack.is_empty() || nums[i] < nums[*stack.last().unwrap()] {
                stack.push(i);
            }
        }

        for i in (0..nums.len()).rev() {
            while !stack.is_empty() && nums[i] >= nums[*stack.last().unwrap()] {
                result = result.max(i - stack.pop().unwrap());
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let nums = vec![6, 0, 8, 2, 1, 5];
        let result = Solution::max_width_ramp(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn leetcode_2() {
        let nums = vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1];
        let result = Solution::max_width_ramp(nums);
        assert_eq!(result, 7);
    }
}

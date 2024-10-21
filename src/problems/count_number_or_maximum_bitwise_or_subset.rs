#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let result = nums.iter().fold(0, |acc, x| acc | *x);
        let mut count = 0;
        for i in 1..2_i32.pow(nums.len() as u32) {
            let mut value = 0;
            for j in 0..nums.len() {
                if 1 << j & i > 0 {
                    value |= nums[j];
                }
            }
            if value == result {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let nums = vec![3, 1];
        let res = Solution::count_max_or_subsets(nums);
        assert_eq!(res, 2);
    }

    #[test]
    fn leetcode_2() {
        let nums = vec![2, 2, 2];
        let res = Solution::count_max_or_subsets(nums);
        assert_eq!(res, 7);
    }

    #[test]
    fn leetcode_3() {
        let nums = vec![3, 2, 1, 5];
        let res = Solution::count_max_or_subsets(nums);
        assert_eq!(res, 6);
    }
}

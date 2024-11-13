#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut result = 0i64;
        let mut nums = nums;
        nums.sort();

        for (i, num) in nums.iter().enumerate() {
            let left = nums[(i + 1)..].partition_point(|x| *x < lower - *num);
            let right = nums[(i + 1)..].partition_point(|x| *x <= upper - *num);
            result += (right - left) as i64
        }

        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![0, 1, 7, 4, 4, 5];
        let lower = 3;
        let upper = 6;
        let result = Solution::count_fair_pairs(nums, lower, upper);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 7, 9, 2, 5];
        let lower = 11;
        let upper = 11;
        let result = Solution::count_fair_pairs(nums, lower, upper);
        assert_eq!(result, 1);
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn compute_left_side_removals(nums: &Vec<i32>) -> Vec<i32> {
        let mut result = vec![0i32; nums.len()];
        for i in 0..nums.len() {
            result[i] = i as i32;
            for j in 0..i {
                if nums[i] > nums[j] {
                    result[i] = result[i].min(i as i32 - j as i32 - 1 + result[j])
                }
            }
        }
        result
    }

    fn compute_right_side_removals(nums: &Vec<i32>) -> Vec<i32> {
        let mut result = vec![0i32; nums.len()];
        for i in (0..nums.len()).rev() {
            result[i] = nums.len() as i32 - 1 - i as i32;
            for j in (i..nums.len()).rev() {
                if nums[i] > nums[j] {
                    result[i] = result[i].min(j as i32 - i as i32 - 1 + result[j])
                }
            }
        }
        result
    }
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let left_side = Solution::compute_left_side_removals(&nums);
        let right_side = Solution::compute_right_side_removals(&nums);
        let mut min_value = i32::MAX;
        for i in 1..(nums.len() - 1) {
            if i as i32 != left_side[i] && right_side[i] != (nums.len() - i - 1) as i32 {
                min_value = min_value.min(left_side[i] + right_side[i])
            }
        }

        min_value
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_mountain_removals_case1() {
        let nums = vec![1, 3, 1];
        let expected = 0;
        assert_eq!(Solution::minimum_mountain_removals(nums), expected);
    }

    #[test]
    fn test_minimum_mountain_removals_case2() {
        let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];
        let expected = 3;
        assert_eq!(Solution::minimum_mountain_removals(nums), expected);
    }

    #[test]
    fn test_minimum_mountain_removals_case3() {
        let nums = vec![9, 8, 1, 7, 6, 5, 4, 3, 2, 1];
        let expected = 2;
        assert_eq!(Solution::minimum_mountain_removals(nums), expected);
    }

    #[test]
    fn test_minimum_mountain_removals_case4() {
        let nums = vec![100, 92, 89, 77, 74, 66, 64, 66, 64];
        let expected = 6;
        assert_eq!(Solution::minimum_mountain_removals(nums), expected);
    }

    #[test]
    fn test_minimum_mountain_removals_case5() {
        let nums = vec![4, 5, 13, 17, 1, 7, 6, 11, 2, 8, 10, 15, 3, 9, 12, 14, 16];
        let expected = 10;
        assert_eq!(Solution::minimum_mountain_removals(nums), expected);
    }

    #[test]
    fn test_minimum_mountain_removals_case6() {
        let nums = vec![1, 2, 1, 3, 4, 4];
        let expected = 3;
        assert_eq!(Solution::minimum_mountain_removals(nums), expected);
    }
}

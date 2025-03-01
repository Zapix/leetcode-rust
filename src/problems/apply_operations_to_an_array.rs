#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }

        let mut shift = 0;
        for i in 0..n {
            if nums[i] == 0 {
                shift += 1;
            } else {
                nums[i - shift] = nums[i];
            }
        }

        for i in 0..shift {
            nums[n - shift + i] = 0;
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 2, 1, 1, 0];
        assert_eq!(Solution::apply_operations(nums), vec![1, 4, 2, 0, 0, 0]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0, 1];
        assert_eq!(Solution::apply_operations(nums), vec![1, 0]);
    }
}

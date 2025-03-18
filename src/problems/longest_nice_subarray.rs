struct Solution;

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut max_value = 1;
        let mut prefix = 0;
        let mut current = 0;
        let mut left = 0;
        let mut right = 0;
        while right < nums.len() {
            while left < right && prefix & nums[right] != 0 {
                prefix &= !nums[left];
                left += 1;
                current -= 1;
            }
            prefix |= nums[right];
            current += 1;
            max_value = max_value.max(current);
            right += 1;
        }

        max_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 3, 8, 48, 10];
        let result = Solution::longest_nice_subarray(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![3, 1, 5, 11, 13];
        let result = Solution::longest_nice_subarray(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![7];
        let result = Solution::longest_nice_subarray(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_no_overlap() {
        let nums = vec![1, 2, 4, 8];
        let result = Solution::longest_nice_subarray(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_all_overlap() {
        let nums = vec![7, 7, 7, 7];
        let result = Solution::longest_nice_subarray(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_large_input() {
        let nums = vec![1, 2, 4, 8, 16, 32, 64, 128];
        let result = Solution::longest_nice_subarray(nums);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_failed_1() {
        let nums = vec![904163577,321202512,470948612,490925389,550193477,87742556,151890632,655280661,4,263168,32,573703555,886743681,937599702,120293650,725712231,257119393];
        let result = Solution::longest_nice_subarray(nums);
        assert_eq!(result, 3);
    }
}

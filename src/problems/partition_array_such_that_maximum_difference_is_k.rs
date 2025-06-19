struct Solution;

impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut count = 0;
        let mut min_value = i32::MIN;
        for num in nums {
            if min_value == i32::MIN || num - min_value > k {
                min_value = num;
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
    fn test_partition_array_example1() {
        let nums = vec![3, 6, 1, 2, 5];
        let k = 2;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_partition_array_example2() {
        let nums = vec![1, 2, 3];
        let k = 1;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_partition_array_example3() {
        let nums = vec![2, 2, 4, 5];
        let k = 0;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_partition_array_empty() {
        let nums = vec![];
        let k = 1;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_partition_array_single_element() {
        let nums = vec![10];
        let k = 5;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_partition_array_large_k() {
        let nums = vec![1, 10, 20, 30];
        let k = 50;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_partition_array_all_same_elements() {
        let nums = vec![5, 5, 5, 5];
        let k = 0;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, 1);
    }
}

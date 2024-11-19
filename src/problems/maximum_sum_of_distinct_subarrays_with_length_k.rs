use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut max_sum = 0i64;
        let mut current_sum = 0i64;
        let mut num_to_index: HashMap<i32, usize> = HashMap::new();
        let mut begin = 0usize;
        let mut end = 0usize;

        while end < nums.len() {
            let curr_num = nums[end];
            let last_occurance = num_to_index.get(&curr_num);

            while (last_occurance.is_some() && begin <= *last_occurance.unwrap())
                || end - begin + 1 > k
            {
                current_sum -= nums[begin] as i64;
                begin += 1;
            }
            num_to_index.insert(curr_num, end);
            current_sum += curr_num as i64;
            if end - begin + 1 == k {
                max_sum = max_sum.max(current_sum)
            }
            end += 1
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 5, 4, 2, 9, 9, 9];
        let k = 3;
        let expected = 15;
        assert_eq!(Solution::maximum_subarray_sum(nums, k), expected);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![4, 4, 4];
        let k = 3;
        let expected = 0;
        assert_eq!(Solution::maximum_subarray_sum(nums, k), expected);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 2];
        let k = 2;
        let expected = 3;
        assert_eq!(Solution::maximum_subarray_sum(nums, k), expected);
    }
}

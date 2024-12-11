#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let mut i = 0;
        let mut max_sequence = 0;
        while i < nums.len() {
            let mut j = i;
            while j < nums.len() && nums[j] <= nums[i] + 2 * k {
                j += 1;
            }
            max_sequence = max_sequence.max(j - i);
            let j = i;
            while i < nums.len() && nums[j] == nums[i] {
                i += 1
            }
        }
        max_sequence as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![4, 6, 1, 2];
        let k = 2;
        assert_eq!(Solution::maximum_beauty(nums, k), 3);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 1, 1, 1];
        let k = 10;
        assert_eq!(Solution::maximum_beauty(nums, k), 4);
    }
}

struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        (0..2usize.pow(nums.len() as u32))
            .map(|val| {
                nums.iter()
                    .enumerate()
                    .filter_map(|(i, x)| if val & (1 << i) > 0 { Some(*x) } else { None })
                    .fold(0, |acc, x| acc ^ x)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 3];
        let result = Solution::subset_xor_sum(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![5, 1, 6];
        let result = Solution::subset_xor_sum(nums);
        assert_eq!(result, 28);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![3, 4, 5, 6, 7, 8];
        let result = Solution::subset_xor_sum(nums);
        assert_eq!(result, 480);
    }
}

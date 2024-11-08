#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let max_val = (1 << maximum_bit) - 1;
        let mut result = vec![0; nums.len()];
        let mut acc = 0;

        for (i, num) in nums.iter().enumerate() {
            acc ^= num;
            result[i] = max_val ^ acc;
        }
        result.reverse();
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_maximum_xor_case_1() {
        let nums = vec![0, 1, 1, 3];
        let maximum_bit = 2;
        assert_eq!(
            Solution::get_maximum_xor(nums, maximum_bit),
            vec![0, 3, 2, 3]
        );
    }

    #[test]
    fn test_get_maximum_xor_case_2() {
        let nums = vec![2, 3, 4, 7];
        let maximum_bit = 3;
        assert_eq!(
            Solution::get_maximum_xor(nums, maximum_bit),
            vec![5, 2, 6, 5]
        );
    }

    #[test]
    fn test_get_maximum_xor_case_3() {
        let nums = vec![0, 1, 2, 2];
        let maximum_bit = 2;
        assert_eq!(
            Solution::get_maximum_xor(nums, maximum_bit),
            vec![2, 0, 2, 3]
        );
    }

    #[test]
    fn test_get_maximum_xor_case_4() {
        let nums = vec![8, 8, 8, 8];
        let maximum_bit = 4;
        assert_eq!(
            Solution::get_maximum_xor(nums, maximum_bit),
            vec![15, 7, 15, 7]
        );
    }

    #[test]
    fn test_get_maximum_xor_case_5() {
        let nums = vec![0, 1, 2, 2, 5, 7];
        let maximum_bit = 3;
        assert_eq!(
            Solution::get_maximum_xor(nums, maximum_bit),
            vec![4, 3, 6, 4, 6, 7]
        );
    }
}

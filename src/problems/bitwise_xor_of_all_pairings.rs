#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let a = if nums2.len() % 2 == 0 {
            0
        } else {
            nums1.iter().fold(0, |acc, x| acc ^ x)
        };
        let b = if nums1.len() % 2 == 0 {
            0
        } else {
            nums2.iter().fold(0, |acc, x| acc ^ x)
        };

        a ^ b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums1 = vec![2, 1, 3];
        let nums2 = vec![10, 2, 5, 0];
        assert_eq!(Solution::xor_all_nums(nums1, nums2), 13);
    }

    #[test]
    fn test_example_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(Solution::xor_all_nums(nums1, nums2), 0);
    }
}

use std::collections::BinaryHeap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums.iter().map(|x| *x as i64).collect::<BinaryHeap<_>>();

        let mut result = 0;
        for _ in (0..k) {
            let value = nums.pop().unwrap();
            result += value;
            nums.push(value / 3 + if value % 3 == 0 { 0 } else { 1 });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_kelements_case1() {
        let nums = vec![10, 10, 10, 10, 10];
        let k = 5;
        let result = Solution::max_kelements(nums, k);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_max_kelements_case2() {
        let nums = vec![1, 10, 3, 3, 3];
        let k = 3;
        let result = Solution::max_kelements(nums, k);
        assert_eq!(result, 17);
    }
}

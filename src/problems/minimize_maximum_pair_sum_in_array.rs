#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let mut max_sum = 0;
        for i in 0..(nums.len() / 2) {
            let j = nums.len() - i - 1;
            max_sum = max_sum.max(nums[i] + nums[j])
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::minimize_maximum_pair_sum_in_array::Solution;

    #[test]
    fn sample_1() {
        assert_eq!(
            Solution::min_pair_sum(vec![3,5,2,3]),
            7
        );
    }

    #[test]
    fn sample_2() {
        assert_eq!(
            Solution::min_pair_sum(vec![3,5,3,2,4,6]),
            8
        );
    }
}
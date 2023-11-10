#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums.to_owned();
        sorted_nums.sort();
        let mut result = 0;
        for i in (0..sorted_nums.len()).step_by(2) {
            result = result + sorted_nums.get(i).unwrap();
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_1() {
        assert_eq!(
            Solution::array_pair_sum(vec![1, 4, 3, 2]),
            4
        );
    }

    #[test]
    fn simple_2() {
        assert_eq!(
            Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]),
            9
        );
    }
}
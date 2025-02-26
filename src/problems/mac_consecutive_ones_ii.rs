#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut total_max = 0;
        let mut consec_ones = 0;
        let mut consec_ones_with_extra = 0;
        for num in nums {
            if num == 0 {
                consec_ones_with_extra = consec_ones + 1;
                consec_ones = 0;
                total_max = total_max.max(consec_ones_with_extra);
            } else {
                consec_ones_with_extra += 1;
                consec_ones += 1;
                total_max = total_max.max(consec_ones_with_extra).max(consec_ones);
            }
        }
        total_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 0, 1, 1, 0];
        assert_eq!(Solution::find_max_consecutive_ones(nums), 4);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        assert_eq!(Solution::find_max_consecutive_ones(nums), 4);
    }
}

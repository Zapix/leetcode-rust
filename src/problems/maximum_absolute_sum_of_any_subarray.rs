#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut negative_sum = 0;
        let mut positive_sum = 0;
        let mut current_sum = 0;
        for num in nums {
            current_sum += num;
            if current_sum > 0 {
                max_sum = max_sum.max(current_sum - negative_sum);
                positive_sum = positive_sum.max(current_sum);
            } else {
                max_sum = max_sum.max(positive_sum - current_sum);
                negative_sum = negative_sum.min(current_sum);
            }
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_absolute_sum_example1() {
        let nums = vec![1, -3, 2, 3, -4];
        assert_eq!(Solution::max_absolute_sum(nums), 5);
    }

    #[test]
    fn test_max_absolute_sum_example2() {
        let nums = vec![2, -5, 1, -4, 3, -2];
        assert_eq!(Solution::max_absolute_sum(nums), 8);
    }
}

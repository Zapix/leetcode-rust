#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        let mut right_sum = nums.iter().sum::<i32>();
        let mut left_sum = 0;
        for value in nums.iter().take(nums.len() -1) {
            left_sum += value;
            right_sum -= value;
            if left_sum >= right_sum {
                counter += 1;
            }
        }
        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ways_to_split_array_case1() {
        let nums = vec![10, 4, -8, 7];
        assert_eq!(Solution::ways_to_split_array(nums), 2);
    }

    #[test]
    fn test_ways_to_split_array_case2() {
        let nums = vec![2, 3, 1, 0];
        assert_eq!(Solution::ways_to_split_array(nums), 2);
    }
}
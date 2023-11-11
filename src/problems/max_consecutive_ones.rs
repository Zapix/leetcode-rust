#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_ones = 0;
        let mut cnt = 0;
        for num in nums {
            if num == 1 {
                cnt += 1;
            } else {
                max_ones = max_ones.max(cnt);
                cnt = 0;
            }
        }
        max_ones = max_ones.max(cnt);
        max_ones
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_1() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1,1,0,1,1,1]),
            3
        );
    }

    #[test]
    fn simple_2() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1,0,1,1,0,1]),
            2
        );
    }
}
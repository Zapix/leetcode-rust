const MODULO: usize = 1_000_000_007;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut dp = vec![0usize; high as usize + 1];
        dp[0] = 1;

        for i in 1..=high as usize {
            let dp_zeros = if i as i32 - zero >= 0 { dp[i - zero as usize] } else { 0 };
            let dp_ones = if i as i32 - one >= 0 { dp[i - one as usize] } else { 0 };
            dp[i] = (dp_zeros + dp_ones) % MODULO;
        }

        (dp[low as usize..=high as usize].iter().sum::<usize>() % MODULO) as i32

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_1() {
        assert_eq!(Solution::count_good_strings(3, 3, 1, 1), 8);
    }

    #[test]
    fn test_leetcode_2() {
        assert_eq!(Solution::count_good_strings(2, 3, 1, 2), 5);
    }

    #[test]
    fn test_leetcode_3() {
        assert_eq!(Solution::count_good_strings(500, 500, 5, 2), 873327137)
    }
}
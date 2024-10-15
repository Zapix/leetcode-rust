#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut result = 0i64;
        let mut position: usize = 0;

        for (i, ch) in s.chars().enumerate() {
            if ch == '0' {
                result += (i - position) as i64;
                position += 1
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_minimum_steps_case1() {
        assert_eq!(Solution::minimum_steps("101".to_string()), 1);
    }

    #[test]
    fn test_minimum_steps_case2() {
        assert_eq!(Solution::minimum_steps("100".to_string()), 2);
    }

    #[test]
    fn test_minimum_steps_case3() {
        assert_eq!(Solution::minimum_steps("0111".to_string()), 0);
    }

    #[test]
    fn test_minimum_steps_case4() {
        assert_eq!(Solution::minimum_steps("1001111001".to_string()), 12);
    }

    #[test]
    fn test_minimum_steps_case5() {
        assert_eq!(Solution::minimum_steps("11001".to_string()), 4);
    }
}

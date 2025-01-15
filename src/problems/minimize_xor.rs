#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut count_ones = num2.count_ones();
        let mut x = 0i32;
        for i in (0..32).rev() {
            if count_ones > 0 && num1 & (1 << i) > 0 {
                x |= 1 << i;
                count_ones -= 1;
            }
        }

        for i in 0..32 {
            if x & (1 << i) == 0 && count_ones > 0 {
                x |= 1 << i;
                count_ones -= 1;
            }
        }

        x
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimize_xor_example1() {
        let num1 = 3;
        let num2 = 5;
        let result = Solution::minimize_xor(num1, num2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_minimize_xor_example2() {
        let num1 = 1;
        let num2 = 12;
        let result = Solution::minimize_xor(num1, num2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_minimize_xor_case1() {
        let num1 = 4;
        let num2 = 7;
        let result = Solution::minimize_xor(num1, num2);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_minimize_xor_case2() {
        let num1 = 10;
        let num2 = 15;
        let result = Solution::minimize_xor(num1, num2);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_minimize_xor_case3() {
        let num1 = 0;
        let num2 = 0;
        let result = Solution::minimize_xor(num1, num2);
        assert_eq!(result, 0);
    }
}


const MODULO: i64 = 1_000_000_007;

struct Solution;

impl Solution {
    fn pow(val: i64, n: usize) -> i64 {
        let mut n = n;
        let mut ret = 1;
        let mut mul = val;
        while n > 0 {
            if n % 2 == 1 {
                ret = ret * mul % MODULO;
            }
            mul = mul * mul % MODULO;
            n /= 2;
        }
        ret
    }

    pub fn count_good_numbers(n: i64) -> i32 {
        let n = n as usize;
        (Solution::pow(5, (n + 1) / 2) * Solution::pow(4, n / 2) % MODULO) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 1;
        let result = Solution::count_good_numbers(n);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_example_2() {
        let n = 4;
        let result = Solution::count_good_numbers(n);
        assert_eq!(result, 400);
    }

    #[test]
    fn test_example_3() {
        let n = 50;
        let result = Solution::count_good_numbers(n);
        assert_eq!(result, 564908303);
    }
}

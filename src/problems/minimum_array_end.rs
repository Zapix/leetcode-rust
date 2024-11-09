#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let x = x as i64;
        let mut n = (n - 1) as i64;
        let mut result = x;
        let mut mask = 1 as i64;

        while n > 0 {
            if (mask & x) == 0 {
                result |= (n & 1) * mask;
                n >>= 1
            }
            mask <<= 1
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_end(3, 4), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_end(2, 7), 15);
    }
}

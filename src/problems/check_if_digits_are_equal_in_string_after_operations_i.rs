struct Solution;

impl Solution {
    fn factorial(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        }
        n * Solution::factorial(n - 1)
    }

    fn binom(n: i32, k: i32) -> i32 {
        Solution::factorial(n) / (Solution::factorial(k) * Solution::factorial(n - k))
    }

    pub fn has_same_digits(s: String) -> bool {
        let size = s.len() as i32 - 1;
        let n = s.len() as i32 - 2;
        let s = s
            .as_bytes()
            .iter()
            .map(|&x| (x - b'0') as i32)
            .collect::<Vec<_>>();

        let v1 = s
            .iter()
            .take(size as usize)
            .enumerate()
            .map(|(i, &d)| (d * Solution::binom(n, i as i32)) % 10)
            .sum::<i32>()
            % 10;

        let v2 = s
            .iter()
            .skip(1)
            .enumerate()
            .map(|(i, &d)| (d * Solution::binom(n, i as i32)) % 10)
            .sum::<i32>()
            % 10;

        v1 == v2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::has_same_digits("3902".to_string()));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::has_same_digits("34789".to_string()));
    }
}

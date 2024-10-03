#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn generate_palindrome(half: i64, even: bool) -> i64 {
        let mut res = half;
        let mut left = half;
        if !even {
            left /= 10
        }
        while left > 0 {
            res = res * 10 + left % 10;
            left /= 10;
        }
        res
    }
    pub fn nearest_palindromic(n: String) -> String {
        let len = n.len();
        let mut num = n.parse::<i64>().unwrap();
        let i = match len % 2 {
            0 => len / 2,
            _ => len / 2 + 1,
        };
        let half = String::from_iter(n.chars().take(i)).parse::<i64>().unwrap();

        let mut possibilities = vec![];
        possibilities.push(Solution::generate_palindrome(half, len % 2 == 0));
        possibilities.push(Solution::generate_palindrome(half - 1, len % 2 == 0));
        possibilities.push(Solution::generate_palindrome(half + 1, len % 2 == 0));
        possibilities.push(10_i64.pow((len as i32 - 1) as u32) - 1);
        possibilities.push(10_i64.pow(len as u32) + 1);

        format!(
            "{}",
            possibilities
                .iter()
                .filter(|x| **x != num)
                .min_by_key(|x| ((num - **x).abs(), **x))
                .unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let n = "123".to_string();
        let result = Solution::nearest_palindromic(n);
        assert_eq!(result, "121".to_string());
    }

    #[test]
    fn leetcode_2() {
        let n = "1".to_string();
        let result = Solution::nearest_palindromic(n);
        assert_eq!(result, "0".to_string());
    }

    #[test]
    fn leetcode_3() {
        let n = "10".to_string();
        let result = Solution::nearest_palindromic(n);
        assert_eq!(result, "9".to_string());
    }

    #[test]
    fn leetcode_4() {
        let n = "99".to_string();
        let result = Solution::nearest_palindromic(n);
        assert_eq!(result, "101".to_string());
    }

    #[test]
    fn leetcode_5() {
        let n = "100".to_string();
        let result = Solution::nearest_palindromic(n);
        assert_eq!(result, "99".to_string());
    }

    #[test]
    fn leetcode_6() {
        let n = "1000".to_string();
        let result = Solution::nearest_palindromic(n);
        assert_eq!(result, "999".to_string());
    }

    #[test]
    fn leetcode_7() {
        let n = "9990111".to_string();
        let result = Solution::nearest_palindromic(n);
        assert_eq!(result, "9989899".to_string());
    }
}

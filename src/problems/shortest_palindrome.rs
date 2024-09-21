#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn is_palindrome(chars: &Vec<char>, start: usize) -> bool {
        let last = chars.len() - 1;
        let n = last - start;
        for i in 0..(n / 2) + 1 {
            if chars[start + i] != chars[last - i] {
                return false;
            }
        }
        true
    }

    pub fn shortest_palindrome(s: String) -> String {
        let mut chars = s.chars().rev().collect::<Vec<char>>();
        let n = chars.len();
        for i in 0..n {
            if Solution::is_palindrome(&chars, i) {
                for j in (0..i).rev() {
                    chars.push(chars[j]);
                }
                break;
            }
        }

        String::from_iter(chars)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_string()),
            "aaacecaaa".to_string()
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::shortest_palindrome("abcd".to_string()),
            "dcbabcd".to_string(),
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::shortest_palindrome("aaacecaa".to_string()),
            "aacecaaacecaa".to_string(),
        );
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::shortest_palindrome("abcba".to_string()),
            "abcba".to_string(),
        );
    }

    #[test]
    fn leetcode_5() {
        assert_eq!(
            Solution::shortest_palindrome("bcba".to_string()),
            "abcba".to_string(),
        );
    }
}

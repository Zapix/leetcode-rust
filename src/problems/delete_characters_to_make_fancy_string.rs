#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for (i, ch) in s.chars().enumerate() {
            if i >= 2 && stack[stack.len() - 1] == ch && stack[stack.len() - 2] == ch {
                continue;
            }
            stack.push(ch);
        }
        String::from_iter(stack)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_fancy_string_case1() {
        let s = String::from("leeetcode");
        let expected = String::from("leetcode");
        assert_eq!(Solution::make_fancy_string(s), expected);
    }

    #[test]
    fn test_make_fancy_string_case2() {
        let s = String::from("aaabaaaa");
        let expected = String::from("aabaa");
        assert_eq!(Solution::make_fancy_string(s), expected);
    }

    #[test]
    fn test_make_fancy_string_case3() {
        let s = String::from("aab");
        let expected = String::from("aab");
        assert_eq!(Solution::make_fancy_string(s), expected);
    }
}

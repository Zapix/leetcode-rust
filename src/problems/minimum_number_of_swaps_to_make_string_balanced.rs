#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut stack = 0;

        for ch in s.chars() {
            if ch == '[' {
                stack += 1;
            } else {
                stack = (stack - 1).max(0);
            }
        }
        (stack + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        let s = "][][".to_string();
        let result = Solution::min_swaps(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn leetcode_1() {
        let s = "]]][[[[".to_string();
        let result = Solution::min_swaps(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn leetcode_2() {
        let s = "[]".to_string();
        let result = Solution::min_swaps(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn leetcode_3() {
        let s = "][[]".to_string();
        let result = Solution::min_swaps(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn leetcode_4() {
        let s = "[]][[]".to_string();
        let result = Solution::min_swaps(s);
        assert_eq!(result, 1);
    }
}

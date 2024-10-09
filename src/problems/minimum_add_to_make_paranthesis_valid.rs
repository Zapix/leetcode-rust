#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut opened = 0;
        let mut unbalanced = 0;

        for c in s.chars() {
            if c == '(' {
                opened += 1;
            } else {
                if opened > 0 {
                    opened -= 1
                } else {
                    unbalanced += 1
                }
            }
        }

        opened + unbalanced
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        let s = "())".to_string();
        let result = Solution::min_add_to_make_valid(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn leetcode_1() {
        let s = "(((".to_string();
        let result = Solution::min_add_to_make_valid(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn leetcode_2() {
        let s = "()".to_string();
        let result = Solution::min_add_to_make_valid(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn leetcode_3() {
        let s = "()))((".to_string();
        let result = Solution::min_add_to_make_valid(s);
        assert_eq!(result, 4);
    }
}

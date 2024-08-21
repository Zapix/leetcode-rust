#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_non_repeating_chars(s: String) -> Vec<char> {
        let mut result = vec![];
        let mut chars = s.chars().collect::<Vec<char>>();
        result.push(chars[0]);
        for i in (1..chars.len()) {
            if (chars[i] == chars[(i as i32 - 1) as usize]) {
                continue
            }
            result.push(chars[i])
        }
        result
    }
    pub fn strange_printer(s: String) -> i32 {
        let chars = Solution::get_non_repeating_chars(s);
        let n = chars.len();
        let mut dp = vec![vec![n; n]; n];
        for i in 0..n {
            dp[i][i] = 1
        }

        for length in 2..(n+1) {
            for start  in 0..(n - length + 1) {
                let end = start + (length as i32 - 1) as usize;
                dp[start][end] = length;
                for j in 0..(length - 1) {
                    let mut total_turns = dp[start][start + j] + dp[start + j + 1][end];
                    if chars[start + j] == chars[end] {
                        total_turns -= 1;
                    }
                    dp[start][end] = dp[start][end].min(total_turns);
                }
            }
        }
        dp[0][n - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::strange_printer("aaabbb".to_string()),
            2
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::strange_printer("aba".to_string()),
            2
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::strange_printer("abcba".to_string()),
            3
        );
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::strange_printer("leetcode".to_string()),
            6
        );
    }

    #[test]
    fn leetcode_5() {
        assert_eq!(
            Solution::strange_printer(
                "baacdddaaddaaaaccbddbcabdaabdbbcdcbbbacbddcabcaaa".to_string()
            ),
            19
        );
    }

    #[test]
    fn leetcode_6() {
        assert_eq!(
            Solution::strange_printer(
                "letcoe".to_string()
            ),
            5
        );
    }
}
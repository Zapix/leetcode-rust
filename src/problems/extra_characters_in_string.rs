#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut dp = vec![0i32; s.len()];
        let get = |i: i32, dp: &Vec<i32>| if i == -1 { 0i32 } else { dp[i as usize] };

        for i in 0..s.len() {
            dp[i] = get(i as i32 - 1, &dp) + 1;
            for j in 0..dictionary.len() {
                let word = &dictionary[j];
                if i + 1 < word.len() {
                    continue;
                }
                if s[(i - (word.len() - 1))..(i + 1)] == *word {
                    dp[i] = dp[i].min(get(i as i32 - word.len() as i32, &dp))
                }
            }
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::min_extra_char(
                "abc".to_string(),
                vec!["a".to_string(), "b".to_string(), "c".to_string()]
            ),
            0
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::min_extra_char("abc".to_string(), vec!["a".to_string(), "b".to_string()]),
            1
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::min_extra_char(
                "leetscode".to_string(),
                vec!["leet".to_string(), "code".to_string()]
            ),
            1
        )
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::min_extra_char(
                "sayhellosworld".to_string(),
                vec!["hello".to_string(), "world".to_string()]
            ),
            4
        );
    }
}

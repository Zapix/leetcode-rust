struct Solution;

impl Solution {
    pub fn valid_length(x: &str) -> bool {
        x.len() >= 3
    }

    pub fn has_one_vowel(x: &str) -> bool {
        "aeouiAEOUI".chars().any(|vowel| x.contains(vowel))
    }

    pub fn has_one_conosant(x: &str) -> bool {
        "qwrtpsdfghjklzxcvbnmQWRTPSDFGHJKLZXCVBNM"
            .chars()
            .any(|conosant| x.contains(conosant))
    }

    pub fn consist_only_allowed_chars(x: &str) -> bool {
        const ALLOWED_CHARS: &'static str =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        x.chars().all(|ch| ALLOWED_CHARS.contains(ch))
    }

    pub fn is_valid(word: String) -> bool {
        [
            Solution::valid_length,
            Solution::has_one_vowel,
            Solution::has_one_conosant,
            Solution::consist_only_allowed_chars,
        ]
        .iter()
        .all(|f| f(word.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::is_valid("234Adas".to_string()), true);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::is_valid("b3".to_string()), false);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::is_valid("s3$e".to_string()), false);
    }
}

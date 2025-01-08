
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn is_prefix_suffix_pair(word1: &str, word2: &str) -> bool {
        word2.starts_with(word1) && word2.ends_with(word1)
    }

    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;
        for i in 0..words.len() {
            for j in i+1..words.len() {
                if i != j && Solution::is_prefix_suffix_pair(&words[i], &words[j]) {
                    count += 1;
                }
            }
        }
        count
    }
}
#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_character_words() {
        let words = vec!["a".to_string(), "aba".to_string(), "ababa".to_string(), "aa".to_string()];
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 4);
    }

    #[test]
    fn test_mixed_words() {
        let words = vec!["pa".to_string(), "papa".to_string(), "ma".to_string(), "mama".to_string()];
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 2);
    }

    #[test]
    fn test_no_pairs() {
        let words = vec!["abab".to_string(), "ab".to_string()];
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 0);
    }

    #[test]
    fn test_single_word() {
        let words = vec!["a".to_string()];
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 0);
    }

    #[test]
    fn test_all_pairs() {
        let words = vec!["ab".to_string(), "abab".to_string(), "ab".to_string(), "ababab".to_string()];
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 5);
    }

    #[test]
    fn test_no_common_prefix_suffix() {
        let words = vec!["cat".to_string(), "dog".to_string(), "fish".to_string(), "bird".to_string()];
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 0);
    }

    #[test]
    fn test_repeated_words() {
        let words = vec!["abc".to_string(), "abc".to_string(), "abc".to_string(), "abc".to_string()];
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 6);
    }

    #[test]
    fn test_long_words() {
        let words = vec![
            "a".repeat(1000),
            "a".repeat(2000),
            "a".repeat(3000),
            "a".repeat(4000),
        ];
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 6);
    }
}

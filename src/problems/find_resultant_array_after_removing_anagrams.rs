struct Solution;

impl Solution {
    fn is_anagram(s1: &str, s2: &str) -> bool {
        let mut count = [0; 26];
        for b in s1.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        for b in s2.bytes() {
            count[(b - b'a') as usize] -= 1;
        }
        count.iter().all(|&c| c == 0)
    }

    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut remove_vec = vec![false; words.len()];
        for i in 1..words.len() {
            if Self::is_anagram(&words[i - 1], &words[i]) {
                remove_vec[i] = true;
            }
        }
        words
            .into_iter()
            .enumerate()
            .filter_map(|(i, w)| if remove_vec[i] { None } else { Some(w) })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_remove_anagrams() {
        let words = vec![
            "abba".to_string(),
            "baba".to_string(),
            "bbaa".to_string(),
            "cd".to_string(),
            "cd".to_string(),
        ];
        let result = Solution::remove_anagrams(words);
        assert_eq!(result, vec!["abba".to_string(), "cd".to_string()]);
    }

    #[test]
    pub fn test_remove_anagrams_1() {
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let result = Solution::remove_anagrams(words);
        assert_eq!(
            result,
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]
        );
    }
}

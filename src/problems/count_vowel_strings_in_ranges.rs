#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_vowel_string(s: &String) -> bool {
        let vowels = ["a", "e", "i", "o", "u"];
        vowels.iter().any(|&x| s.starts_with(x)) && vowels.iter().any(|&x| s.ends_with(x))
    }

    pub fn get_vowel_counts(words: &Vec<String>) -> Vec<i32> {
        let mut counter = 0;
        let mut vowel_counts = vec![];
        for word in words {
            counter += if Self::is_vowel_string(word) { 1 } else { 0 };
            vowel_counts.push(counter);
        }
        vowel_counts
    }
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowel_counts = Self::get_vowel_counts(&words);

        queries
            .iter()
            .map(|x| {
                let left = match x[0] {
                    0 => 0,
                    val => vowel_counts[(val - 1) as usize],
                };
                let right = vowel_counts[x[1] as usize];
                right - left
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let words = vec![
            "aba".to_string(),
            "bcb".to_string(),
            "ece".to_string(),
            "aa".to_string(),
            "e".to_string(),
        ];
        let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
        assert_eq!(Solution::vowel_strings(words, queries), vec![2, 3, 0]);
    }

    #[test]
    fn test_example_2() {
        let words = vec!["a".to_string(), "e".to_string(), "i".to_string()];
        let queries = vec![vec![0, 2], vec![0, 1], vec![2, 2]];
        assert_eq!(Solution::vowel_strings(words, queries), vec![3, 2, 1]);
    }
}

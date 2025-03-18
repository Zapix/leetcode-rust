use std::collections::HashMap;

trait IsVowelCheck {
    fn is_vowel(&self) -> bool;
}

impl IsVowelCheck for char {
    fn is_vowel(&self) -> bool {
        ['a', 'e', 'i', 'o', 'u'].contains(self)
    }
}

#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word = word.chars().collect::<Vec<_>>();
        let mut result = 0;
        let mut left = 0;
        let mut right = 0;
        let mut vowels = HashMap::new();
        let mut consonants_count = 0;
        let mut next_consonant = vec![0; word.len()];
        let mut next_consonants_index = word.len();
        for i in (0..word.len()).rev() {
            next_consonant[i] = next_consonants_index;
            if !word[i].is_vowel() {
                next_consonants_index = i;
            }
        }

        while right < word.len() {
            match word[right] {
                ch if ch.is_vowel() => {
                    let entry = vowels.entry(ch).or_insert(0);
                    *entry += 1;
                }
                _ => {
                    consonants_count += 1;
                }
            }
            while consonants_count > k && left < right {
                match word[left] {
                    ch if ch.is_vowel() => {
                        let entry = vowels.entry(ch).or_insert(0);
                        *entry -= 1;
                        if *entry == 0 {
                            vowels.remove(&ch);
                        }
                    }
                    _ => {
                        consonants_count -= 1;
                    }
                }
                left += 1
            }
            while left < word.len() && vowels.len() == 5 && consonants_count == k {
                result += (next_consonant[right] - right) as i64;
                match word[left] {
                    ch if ch.is_vowel() => {
                        let entry = vowels.entry(ch).or_insert(0);
                        *entry -= 1;
                        if *entry == 0 {
                            vowels.remove(&ch);
                        }
                    }
                    _ => {
                        consonants_count -= 1;
                    }
                }
                left += 1;
            }

            right += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let word = String::from("aeioqq");
        let k = 1;
        assert_eq!(Solution::count_of_substrings(word, k), 0);
    }

    #[test]
    fn test_example_2() {
        let word = String::from("aeiou");
        let k = 0;
        assert_eq!(Solution::count_of_substrings(word, k), 1);
    }

    #[test]
    fn test_example_3() {
        let word = String::from("ieaouqqieaouqq");
        let k = 1;
        assert_eq!(Solution::count_of_substrings(word, k), 3);
    }
}

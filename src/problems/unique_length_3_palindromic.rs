use std::collections::{HashSet};
#[allow(dead_code)]
struct Solution;

impl Solution {
    fn get_unique_chars(s: &String) -> HashSet<char> {
        let mut set: HashSet<char> = HashSet::new();
        for c in s.chars() {
            set.insert(c);
        }
        set
    }

    fn get_all_possible_palindroms(s: &String) -> Vec<String> {
        let unique_chars = Solution::get_unique_chars(s);
        let mut palindromes: Vec<String> = Vec::new();

        for wrap_c in unique_chars.clone().into_iter() {
            for inner_c in unique_chars.clone().into_iter() {
                palindromes.push(
                    String::from(
                        [wrap_c, inner_c, wrap_c]
                            .iter()
                            .map(|x|String::from(*x))
                            .collect::<Vec<String>>()
                            .join("")
                    )
                );
            }
        }

        palindromes
    }

    fn is_substring(s: &String, pattern: &String) -> bool {
        let mut p_iter = pattern.chars();
        let mut p_c = p_iter.next();

        for c in s.chars() {
            match p_c {
                Some(n_c) => {
                    if n_c == c {
                        p_c = p_iter.next();
                    }
                }
                None => break,
            }
        }

        match p_c {
            None => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let palindroms = Solution::get_all_possible_palindroms(&s);
        palindroms.iter().filter(|x| Solution::is_substring(&s, *x)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            Solution::count_palindromic_subsequence(String::from("aabca")),
            3,
        )
    }

    #[test]
    fn sample2() {
        assert_eq!(
            Solution::count_palindromic_subsequence(String::from("adc")),
            0,
        );
    }

    #[test]
    fn sample3() {
        assert_eq!(
            Solution::count_palindromic_subsequence(String::from("bbcbaba")),
            4
        );
    }
}
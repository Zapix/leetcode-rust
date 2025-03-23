use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut first = vec![None; 26];
        let mut last = vec![None; 26];
        let chars = s.chars().collect::<Vec<_>>();

        for (i, ch) in chars.iter().enumerate() {
            let idx = (*ch as u8 - 'a' as u8) as usize;
            first[idx] = match first[idx] {
                None => Some(i),
                Some(x) => Some(x),
            };
            last[idx] = Some(i);
        }

        let mut ans = 0;

        for i in 0..26 {
            match (first[i], last[i]) {
                (Some(begin), Some(end)) => {
                    let mut variants = HashSet::new();
                    for i in (begin + 1)..end {
                        variants.insert(chars[i]);
                    }
                    ans += variants.len();
                }
                _ => {}
            }
        }
        ans as i32
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

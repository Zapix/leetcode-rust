use std::collections::HashMap;
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1map = s1.chars().fold(HashMap::new(), |mut acc, item| {
            if let Some(val) = acc.get_mut(&item) {
                *val += 1;
            } else {
                acc.insert(item, 1);
            }
            acc
        });

        let chars = s2.chars().collect::<Vec<_>>();

        let mut window = s2
            .chars()
            .take(s1.len())
            .fold(HashMap::new(), |mut acc, item| {
                if let Some(val) = acc.get_mut(&item) {
                    *val += 1;
                } else {
                    acc.insert(item, 1);
                }
                acc
            });

        if s1map == window {
            return true;
        }

        for (i, ch) in chars.iter().enumerate().skip(s1.len()) {
            let exclude_char = chars[i - s1.len()];
            let val = window.get_mut(&exclude_char).unwrap();
            *val -= 1;
            if *val == 0 {
                window.remove(&exclude_char);
            }

            if let Some(val) = window.get_mut(ch) {
                *val += 1;
            } else {
                window.insert(*ch, 1);
            }

            if s1map == window {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::check_inclusion("abc".to_string(), "bc".to_string()),
            false,
        );
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::check_inclusion("abc".to_string(), "bcaa".to_string()),
            true,
        );
    }
}

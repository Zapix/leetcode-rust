use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let frequency = word
            .chars()
            .fold(HashMap::new(), |mut acc, ch| {
                let entry = acc.entry(ch).or_insert(0);
                *entry += 1;
                acc
            });
                
        let mut res = word.len() as i32;
        for &a in frequency.values() {
            let mut deleted = 0;
            for &b in frequency.values() {
                if a > b {
                    deleted += b;
                } else if b > a + k {
                    deleted += b - (a + k);
                }
            }
            res = res.min(deleted)
        }
        res
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let word = "aabcaba".to_string();
        let k = 0;
        assert_eq!(Solution::minimum_deletions(word, k), 3);
    }

    #[test]
    fn test_example_2() {
        let word = "dabdcbdcdcd".to_string();
        let k = 2;
        assert_eq!(Solution::minimum_deletions(word, k), 2);
    }

    #[test]
    fn test_example_3() {
        let word = "aaabaaa".to_string();
        let k = 2;
        assert_eq!(Solution::minimum_deletions(word, k), 1);
    }
}

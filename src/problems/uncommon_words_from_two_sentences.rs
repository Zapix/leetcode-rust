use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        s1.split(" ")
            .chain(s2.split(" "))
            .fold(HashMap::new(), |mut acc, x| {
                if acc.contains_key(x) {
                    let mut val = acc.get_mut(x).unwrap();
                    *val += 1;
                } else {
                    acc.insert(x, 1);
                }
                acc
            })
            .iter()
            .filter(|x| *x.1 == 1)
            .map(|x| x.0.to_string())
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        let result = Solution::uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string(),
        );
        for word in vec!["sweet", "sour"] {
            assert!(result.iter().find(|x| *x == word).is_some())
        }
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string()),
            vec!["banana"]
        );
    }
}

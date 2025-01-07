#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        words
            .clone()
            .into_iter()
            .filter(|x| {
                words
                    .clone()
                    .into_iter()
                    .find(|check| *check != *x && check.contains(x.as_str()))
                    .is_some()
            })
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let words = vec![
            "mass".to_string(),
            "as".to_string(),
            "hero".to_string(),
            "superhero".to_string(),
        ];
        assert_eq!(
            Solution::string_matching(words),
            vec!["as".to_string(), "hero".to_string()]
        );
    }

    #[test]
    fn test_example_2() {
        let words = vec!["leetcode".to_string(), "et".to_string(), "code".to_string()];
        assert_eq!(
            Solution::string_matching(words),
            vec!["et".to_string(), "code".to_string()]
        );
    }

    #[test]
    fn test_example_3() {
        let words = vec!["blue".to_string(), "green".to_string(), "bu".to_string()];
        assert_eq!(Solution::string_matching(words), Vec::<String>::new());
    }
}

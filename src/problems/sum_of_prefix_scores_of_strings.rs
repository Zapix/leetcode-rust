use std::collections::HashMap;

struct Trie {
    count: i32,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            count: 0,
            children: HashMap::new(),
        }
    }

    pub fn insert_word(&mut self, word: String) {
        let mut current = self;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert(Trie::new());
            current.count += 1;
        }
    }

    pub fn count_value(&self, word: String) -> i32 {
        let mut current = self;
        let mut result = 0;
        for ch in word.chars() {
            current = current.children.get(&ch).unwrap();
            result += current.count;
        }
        result
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();
        for word in words.clone() {
            trie.insert_word(word);
        }

        words
            .iter()
            .map(|x| trie.count_value(x.to_string()))
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(
            Solution::sum_prefix_scores(vec![
                "abc".to_string(),
                "ab".to_string(),
                "bc".to_string(),
                "b".to_string(),
            ]),
            vec![5, 4, 3, 2]
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::sum_prefix_scores(vec!["abcd".to_string(),]),
            vec![4]
        );
    }
}

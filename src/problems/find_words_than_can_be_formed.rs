#[allow(dead_code)]
struct Solution;

use std::collections::{HashMap};

impl Solution {

    pub fn get_counter(chars: &String) -> HashMap<char, i32> {
        let mut char_map: HashMap<char, i32> = HashMap::new();
        for ch in chars.chars() {
            let count = match char_map.get(&ch) {
                None => 0,
                Some(x) => *x,
            };
            char_map.insert(ch, count + 1);
        }
        char_map
    }

    pub fn includes(a: &HashMap<char, i32>, b: &HashMap<char, i32>) -> bool {
        for (key, value) in b.iter() {
            let a_value = match a.get(key) {
                Some(x) => *x,
                None => 0,
            };
            if a_value < *value {
                return false;
            }
        }
        return true
    }

    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut counter = 0;
        let chars_map = Solution::get_counter(&chars);
        for word in words {
            let word_map = Solution::get_counter(&word);
            if Solution::includes(&chars_map, &word_map) {
                counter += word.len() as i32;
            }
        }
        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        assert_eq!(
            Solution::count_characters(
                vec![
                    String::from("cat"),
                    String::from("bt"),
                    String::from("hat"),
                    String::from("tree"),
                ],
                String::from("atach"),
            ),
            6
        )
    }

    #[test]
    fn sample_2() {
        assert_eq!(
            Solution::count_characters(
                vec![
                    String::from("hello"),
                    String::from("world"),
                    String::from("leetcode"),
                ],
                String::from("welldonehoneyr"),
            ),
            10
        );
    }
}
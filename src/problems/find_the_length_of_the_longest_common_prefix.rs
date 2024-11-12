use std::collections::HashMap;

struct Trie {
    children: HashMap<i32, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, arr: Vec<i32>) {
        let mut current = self;
        for x in arr {
            current = current.children.entry(x).or_insert(Trie::new());
        }
    }

    pub fn longest_common_prefix(&self, arr: Vec<i32>) -> i32 {
        let mut current = self;
        let mut result = 0;
        for x in arr {
            if let Some(child) = current.children.get(&x) {
                result += 1;
                current = child;
            } else {
                break;
            }
        }
        result
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        let (arr1, arr2) = if arr1.len() > arr2.len() {
            (arr1, arr2)
        } else {
            (arr2, arr1)
        };

        for x in arr1 {
            let number = x
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>();
            trie.insert(number);
        }

        let mut result = 0;
        for x in arr2 {
            let number = x
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>();
            result = result.max(trie.longest_common_prefix(number));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 10, 100], vec![1000]),
            3
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4]),
            0
        );
    }
}

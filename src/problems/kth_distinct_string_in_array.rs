use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut hash_map = HashMap::new();
        for word in arr.iter() {
            let value = *hash_map.get((*word).as_str()).unwrap_or(&0);
            hash_map.insert((*word).as_str(), value + 1);
        }

        let mut counter = 0;
        let mut result: Option<String> = None;
        for word in arr.iter() {
            if  *hash_map.get((*word).as_str()).unwrap_or(&0) == 1 {
                counter += 1;
            }
            if counter == k {
                result = Some(String::from((*word).as_str()));
                break
            }
        }
        result.unwrap_or(String::from(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::kth_distinct(
                vec![
                    String::from("d"),
                    String::from("b"),
                    String::from("c"),
                    String::from("b"),
                    String::from("c"),
                    String::from("a"),
                ],
                2
            ),
            String::from("a"),
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::kth_distinct(
                vec![
                    String::from("aaa"),
                    String::from("aa"),
                    String::from("a"),
                ],
                1
            ),
            String::from("aaa")
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::kth_distinct(
                vec![
                    String::from("a"),
                    String::from("b"),
                    String::from("a"),
                ],
                3
            ),
            String::from("")
        )
    }
}
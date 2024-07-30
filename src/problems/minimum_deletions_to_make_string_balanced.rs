use std::cmp::min;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut end_a = 0;
        let mut end_b = 0;
        for ch in s.split("") {
            if ch.is_empty(){
                continue;
            }

            if ch == "a" {
                end_b += 1;
            } else {
                end_b = min(end_a, end_b);
                end_a = end_a + 1;
            }
        }
        min(end_a, end_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::minimum_deletions(String::from("aababbab")),
            2
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::minimum_deletions(String::from("bbaaaaabb")),
            2
        );
    }
}
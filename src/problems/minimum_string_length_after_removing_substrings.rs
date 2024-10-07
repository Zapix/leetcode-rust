use std::collections::HashSet;

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    fn max_removes(s: &String, removed: HashSet<usize>) -> i32 {
        let mut prev: Option<char> = None;
        let mut prev_id: Option<i32> = None;
        let mut new_removed = removed.clone();
        let mut current_removes = 0;
        for (i, c) in s.chars().enumerate() {
            if removed.contains(&i) {
                continue;
            }
            if let Some(p) = prev {
                if p == 'A' && c == 'B' {
                    new_removed.insert(i);
                    new_removed.insert(prev_id.unwrap() as usize);
                    prev = None;
                    prev_id = None;
                    current_removes += 2;
                    continue;
                }
                if p == 'C' && c == 'D' {
                    new_removed.insert(i);
                    new_removed.insert(prev_id.unwrap() as usize);
                    prev = None;
                    prev_id = None;
                    current_removes += 2;
                    continue;
                };
            }
            prev = Some(c);
            prev_id = Some(i as i32);
        }
        if current_removes == 0 {
            return 0;
        }
        current_removes + Solution::max_removes(s, new_removed)
    }
    pub fn min_length(s: String) -> i32 {
        s.len() as i32 - Solution::max_removes(&s, HashSet::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let s = "ABFCACDB".to_string();
        let result = Solution::min_length(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn leetcode_2() {
        let s = "CAB".to_string();
        let result = Solution::min_length(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn leetcode_3() {
        let s = "ACDBC".to_string();
        let result = Solution::min_length(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn leetcode_4() {
        let s = "ACBBD".to_string();
        let result = Solution::min_length(s);
        assert_eq!(result, 5);
    }
}

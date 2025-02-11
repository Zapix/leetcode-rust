#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut stack = vec![];
        for ch in s.chars() {
            stack.push(ch);
            if stack.len() >= part.len() {
                let mut found = true;
                for (i, part_ch) in part.chars().enumerate() {
                    if stack[stack.len() - part.len() + i] != part_ch {
                        found = false;
                        break;
                    }
                }
                if found {
                    for _ in 0..part.len() {
                        stack.pop();
                    }
                }
            }
        }
        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_occurrences() {
        let s = "daabcbaabcbc".to_string();
        let part = "abc".to_string();
        assert_eq!(Solution::remove_occurrences(s, part), "dab");
        let s = "axxxxyyyyb".to_string();
        let part = "xy".to_string();
        assert_eq!(Solution::remove_occurrences(s, part), "ab");
        let s = "abccba".to_string();
        let part = "abc".to_string();
        assert_eq!(Solution::remove_occurrences(s, part), "cba");
    }
}
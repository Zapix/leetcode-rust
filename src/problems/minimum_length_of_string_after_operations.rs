#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut counter = [0; 26];
        for ch in s.chars() {
            counter[(ch as usize) - ('a' as usize)] += 1;
        }
        counter.iter().filter(|x| **x > 0).map(|x| if x  % 2 == 0 { 2 } else { 1 }).sum()   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_length_example_cases() {
        assert_eq!(Solution::minimum_length("abaacbcbb".to_string()), 5);
        assert_eq!(Solution::minimum_length("aa".to_string()), 2);
    }
}

use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_k_length_substr_no_repeats(s: String, k: i32) -> i32 {
        let chars = s.chars().collect::<Vec<_>>();
        let k = k as usize;
        let mut window = HashMap::new();
        let mut more_then_once = HashSet::new();
        let mut result = 0;
        let mut size = 0;

        for (i, chr) in chars.iter().enumerate() {
            let entry = window.entry(*chr).or_insert(0);
            *entry += 1;
            if *entry > 1 {
                more_then_once.insert(*chr);
            }
            size += 1;

            if size > k {
                let rm_chr = chars[i - k];
                let entry = window.entry(rm_chr).or_insert(0);
                *entry -= 1;
                if *entry <= 1 {
                    more_then_once.remove(&rm_chr);
                }
                size -= 1;
            }

            if size == k && more_then_once.len() == 0 {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("havefunonleetcode");
        let k = 5;
        assert_eq!(Solution::num_k_length_substr_no_repeats(s, k), 6);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("home");
        let k = 5;
        assert_eq!(Solution::num_k_length_substr_no_repeats(s, k), 0);
    }
}

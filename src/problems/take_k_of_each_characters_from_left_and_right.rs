use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let chars = s.chars().collect::<Vec<_>>();
        let mut counter = s.chars().fold(
            HashMap::from([('a', 0), ('b', 0), ('c', 0)]),
            |mut cnt, item| {
                let count = cnt.entry(item).or_insert(0);
                *count += 1;
                cnt
            },
        );

        if counter.values().any(|x| *x < k) {
            return -1;
        }
        let mut result = 0;
        let mut left = 0;
        let mut window = HashMap::from([('a', 0), ('b', 0), ('c', 0)]);
        for right in 0..chars.len() {
            *window.entry(chars[right]).or_insert(0) += 1;

            while left <= right
                && (counter[&'a'] - window[&'a'] < k
                    || counter[&'b'] - window[&'b'] < k
                    || counter[&'c'] - window[&'c'] < k)
            {
                *window.entry(chars[left]).or_insert(0) -= 1;
                left += 1;
            }
            result = result.max((right as i32 - left as i32 + 1i32) as usize);
        }

        (s.len() - result) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "aabaaaacaabc".to_string();
        let k = 2;
        let expected = 8;
        assert_eq!(Solution::take_characters(s, k), expected);
    }

    #[test]
    fn test_example_2() {
        let s = "a".to_string();
        let k = 1;
        let expected = -1;
        assert_eq!(Solution::take_characters(s, k), expected);
    }
}

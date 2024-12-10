use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut counter: HashMap<String, usize> = HashMap::new();
        let n = s.len();
        let chars = s.chars().collect::<Vec<_>>();
        for i in 0..n {
            for j in i..n {
                if chars[i] == chars[j] {
                    let str = String::from_iter(chars[i..(j + 1)].iter());
                    let entry = counter.entry(str).or_insert(0);
                    *entry += 1;
                } else {
                    break;
                }
            }
        }
        let max_value = counter
            .iter()
            .filter(|x| *x.1 >= 3)
            .map(|x| x.0.len())
            .max();
        match max_value {
            Some(value) => value as i32,
            _ => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("aaaa");
        assert_eq!(Solution::maximum_length(s), 2);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("abcdef");
        assert_eq!(Solution::maximum_length(s), -1);
    }

    #[test]
    fn test_example_3() {
        let s = String::from("abcaba");
        assert_eq!(Solution::maximum_length(s), 1);
    }
}

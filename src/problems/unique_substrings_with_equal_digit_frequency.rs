use std::collections::HashSet;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_same_frequency(digits: &[i32; 10]) -> bool {
        let digits = digits.into_iter().filter(|x| **x > 0).collect::<Vec<_>>();
        let count = *digits.first().unwrap();
        digits.iter().all(|x| *x == count)
    }
    pub fn equal_digit_frequency(s: String) -> i32 {
        let s = s.as_str();
        let s_chars = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut sub_strings = HashSet::new();
        for i in 0..n {
            let mut digits = [0; 10];
            for j in i..n {
                let idx = (s_chars[j] as u8 - '0' as u8) as usize;
                digits[idx] += 1;
                if Self::is_same_frequency(&digits) {
                    sub_strings.insert(s[i..=j].to_string());
                }
            }
        }

        sub_strings.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("1212");
        assert_eq!(Solution::equal_digit_frequency(s), 5);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("12321");
        assert_eq!(Solution::equal_digit_frequency(s), 9);
    }
}

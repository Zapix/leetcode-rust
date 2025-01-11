#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut counter = [0; 26];
        for ch in s.chars() {
            let idx = (ch as u8 - 'a' as u8) as usize;
            counter[idx] += 1;
        }
        s.len() >= (k as usize) && counter.iter().filter(|x| *x % 2 == 1).count() <= (k as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("annabelle");
        let k = 2;
        assert_eq!(Solution::can_construct(s, k), true);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("leetcode");
        let k = 3;
        assert_eq!(Solution::can_construct(s, k), false);
    }

    #[test]
    fn test_example_3() {
        let s = String::from("true");
        let k = 4;
        assert_eq!(Solution::can_construct(s, k), true);
    }
}

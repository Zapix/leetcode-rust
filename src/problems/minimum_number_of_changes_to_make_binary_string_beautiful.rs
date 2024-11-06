#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let chars = s.chars();
        chars
            .clone()
            .step_by(2)
            .zip(chars.clone().skip(1).step_by(2))
            .filter(|x| x.0 != x.1)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "1001".to_string();
        assert_eq!(Solution::min_changes(s), 2);
    }

    #[test]
    fn test_example_2() {
        let s = "10".to_string();
        assert_eq!(Solution::min_changes(s), 1);
    }

    #[test]
    fn test_example_3() {
        let s = "0000".to_string();
        assert_eq!(Solution::min_changes(s), 0);
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_start_with(start_value: i32, derived: &Vec<i32>) -> bool {
        let expected_last = match derived.last() {
            Some(0) => start_value,
            Some(1) => start_value ^ 1,
            _ => panic!("unexpected"),
        };
        let mut original = start_value;
        for i in 1..derived.len() {
            original = original ^ derived[i - 1]
        }
        original == expected_last
    }
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        Solution::check_start_with(0, &derived) || Solution::check_start_with(1, &derived)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let derived = vec![1, 1, 0];
        assert_eq!(Solution::does_valid_array_exist(derived), true);
    }

    #[test]
    fn test_example_2() {
        let derived = vec![1, 1];
        assert_eq!(Solution::does_valid_array_exist(derived), true);
    }

    #[test]
    fn test_example_3() {
        let derived = vec![1, 0];
        assert_eq!(Solution::does_valid_array_exist(derived), false);
    }
}

use std::cmp::Ordering;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn cmp(a: &i32, b: &i32) -> Ordering {
        let a = a.to_string();
        let b = b.to_string();
        let ab = a.clone() + &b;
        let ba = b + &a;
        ba.cmp(&ab)
    }

    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_by(|a, b| Solution::cmp(a, b));
        let result: String = nums.into_iter().map(|x| x.to_string()).collect::<String>();
        if result.chars().next().unwrap() == '0' {
            return "0".to_string();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_string()
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(Solution::largest_number(vec![10, 20]), "2010".to_string());
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(Solution::largest_number(vec![0, 0]), "0".to_string());
    }
}

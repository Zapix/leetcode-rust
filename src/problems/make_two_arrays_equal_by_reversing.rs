use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;


#[allow(dead_code)]
impl Solution {

    pub fn count(target: Vec<i32>) -> HashMap<i32, i32> {
        target
            .iter()
            .fold(
                HashMap::new(),
                |mut acc, value| {
                    let count = *acc.get(value).unwrap_or(&0);
                    acc.insert(*value, count + 1);
                    acc
                }
            )
    }
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        Self::count(target) == Self::count(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert!(
            Solution::can_be_equal(
               vec![2, 1, 3, 4],
                vec![3, 4, 2, 1]
            )
        )
    }

    #[test]
    fn leetcode_2() {
        assert!(
            Solution::can_be_equal(
                vec![7],
                vec![7],
            )
        )
    }

    #[test]
    fn leetcode_3() {
        assert!(
            !Solution::can_be_equal(
                vec![3, 7, 9],
                vec![3, 7, 11],
            )
        )
    }
}
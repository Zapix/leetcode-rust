use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as i64;
        let mut que = BinaryHeap::from([Reverse(1i64)]);
        let mut visited = HashSet::new();
        let mut result = 0;
        for _ in 0..n {
            while let Some(value) = que.pop() {
                if !visited.contains(&value.0) {
                    result = value.0;
                    visited.insert(result);
                    break;
                }
            }
            que.push(Reverse(result * 2));
            que.push(Reverse(result * 3));
            que.push(Reverse(result * 5));
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::nth_ugly_number(10),
            12
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::nth_ugly_number(1),
            1
        );
    }
    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::nth_ugly_number(1407),
            536870912
        );
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::nth_ugly_number(14),
            20
        );
    }
}
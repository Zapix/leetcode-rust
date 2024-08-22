#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut result = 1i32;
        while num > result {
            result = (result << 1) | 1;
        }
        num ^ result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::find_complement(5),
            2
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::find_complement(1),
            0
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::find_complement(7),
            0
        );
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::find_complement(10),
            5
        );
    }
}
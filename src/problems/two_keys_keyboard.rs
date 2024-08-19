#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn get_max_divider(n: i32) -> i32 {
        for i in (1..(n/2 + 1)).rev() {
            if n % i == 0 {
                return i
            }
        }
        1
    }
    pub fn min_steps(n: i32) -> i32 {
        match n {
            1 => 0,
            _ => {
                let max_divider = Solution::get_max_divider(n);
                match max_divider {
                    1 => n,
                    _ => Solution::min_steps(max_divider) + (n / max_divider)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::min_steps(3),
            3
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::min_steps(1),
            0
        );
    }


    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::min_steps(7),
            7,
        );
    }
    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::min_steps(10),
           7
        );
    }
    #[test]
    fn leetcode_5() {
        assert_eq!(
            Solution::min_steps(24),
            9
        );
    }
    #[test]
    fn leetcode_6() {
        assert_eq!(
            Solution::min_steps(13),
            13
        );
    }
    #[test]
    fn leetcode_7() {
        assert_eq!(
            Solution::min_steps(97),
            97
        );
    }
    #[test]
    fn leetcode_8() {
        assert_eq!(
            Solution::min_steps(100),
            14
        );
    }
}

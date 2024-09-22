#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn count_prefix(n: i32, curr: i32, next: i32) -> i32 {
        let n = n as i64;
        let mut curr = curr as i64;
        let mut next = next as i64;
        let mut steps = 0i64;
        while curr <= n {
            steps += (n + 1).min(next) - curr;
            curr *= 10;
            next *= 10;
        }
        steps as i32
    }
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut curr = 1;
        let mut k = k - 1;
        while k > 0 {
            let step = Solution::count_prefix(n, curr, curr + 1);
            if step <= k {
                k -= step;
                curr += 1;
            } else {
                k -= 1;
                curr *= 10;
            }
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(Solution::find_kth_number(13, 2), 10);
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(Solution::find_kth_number(10, 3), 2);
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(Solution::find_kth_number(100, 10), 17);
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(Solution::find_kth_number(100, 90), 9);
    }

    #[test]
    fn leetcode_5() {
        assert_eq!(Solution::find_kth_number(681692778, 351251360), 416126219);
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut result: Vec<i32> = vec![];

        for ch in s.chars() {
            let val = (ch as i32 - 'a' as i32) as i32 + 1;
            if val >= 10 {
                result.push(val / 10);
                result.push(val % 10);
            } else {
                result.push(val)
            }
        }
        result.reverse();

        for _ in 0..k {
            let mut next: Vec<i32> = vec![];
            let mut val: i32 = result.iter().sum();
            while val > 0 {
                next.push(val % 10);
                val /= 10;
            }
            result = next;
        }

        result
            .iter()
            .enumerate()
            .map(|x| *x.1 * 10_i32.pow(x.0 as u32))
            .sum()
    }
}

mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let s = "iiii".to_string();
        let k = 1;
        let result = Solution::get_lucky(s, k);
        assert_eq!(36, result);
    }

    #[test]
    fn leetcode_2() {
        let s = "leetcode".to_string();
        let k = 2;
        let result = Solution::get_lucky(s, k);
        assert_eq!(6, result);
    }

    #[test]
    fn leetcode_3() {
        let s = "zbax".to_string();
        let k = 2;
        let result = Solution::get_lucky(s, k);
        assert_eq!(8, result);
    }
}

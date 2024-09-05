#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn get_rolls(n: i32, x: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut x = x;
        let mut n = n;
        while n > 0 {
            for i in (1..7).rev() {
                if x - i >= n - 1 && x - i <= 6 * (n - 1) {
                    result.push(i);
                    x -= i;
                    n -= 1;
                    break;
                }
            }
        }
        result
    }
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let m = rolls.len() as i32;
        let sum: i32 = rolls.iter().sum();

        match mean * (m + n) - sum {
            x if x < n => vec![],
            x if x > 6 * n => vec![],
            x => Solution::get_rolls(n, x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let rolls = vec![3, 2, 4, 3];
        let mean = 4;
        let n = 2;
        let result = Solution::missing_rolls(rolls, mean, n);
        assert_eq!(vec![6, 6], result);
    }

    #[test]
    fn leetcode_2() {
        let rolls = vec![1, 5, 6];
        let mean = 3;
        let n = 4;
        let result = Solution::missing_rolls(rolls, mean, n);
        assert_eq!(vec![6, 1, 1, 1], result);
    }

    #[test]
    fn leetcode_3() {
        let rolls = vec![1, 2, 3, 4];
        let mean = 6;
        let n = 4;
        let result = Solution::missing_rolls(rolls, mean, n);
        assert_eq!(vec![] as Vec<i32>, result);
    }

    #[test]
    fn leetcode_4() {
        let rolls = vec![6, 6, 6];
        let mean = 1;
        let n = 3;
        let result = Solution::missing_rolls(rolls, mean, n);
        assert_eq!(vec![] as Vec<i32>, result);
    }
}

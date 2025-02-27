use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let mut dp = vec![vec![0; arr.len()]; arr.len()];
        let val_to_idx: HashMap<i32, usize> =
            HashMap::from_iter(arr.iter().enumerate().map(|x| (*x.1, x.0)));

        for curr in 0..arr.len() {
            for prev in 0..curr {
                let diff = arr[curr] - arr[prev];
                dp[prev][curr] = match val_to_idx.get(&diff) {
                    Some(prev_idx) if diff < arr[prev] => dp[*prev_idx][prev] + 1,
                    _ => 2,
                };
                max_len = max_len.max(dp[prev][curr]);
            }
        }
        if max_len >= 3 {
            max_len
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(Solution::len_longest_fib_subseq(arr), 5);
    }

    #[test]
    fn test_example_2() {
        let arr = vec![1, 3, 7, 11, 12, 14, 18];
        assert_eq!(Solution::len_longest_fib_subseq(arr), 3);
    }

    #[test]
    fn test_example_3() {
        let arr = vec![2, 4, 7, 8, 9, 10, 14, 15, 18, 23, 32, 50];
        assert_eq!(Solution::len_longest_fib_subseq(arr), 5);
    }
}

struct Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i))
            .collect::<Vec<_>>();
        nums.sort_by_key(|(x, _)| *x);
        let mut nums = nums
            .into_iter()
            .skip(n.checked_sub(k as usize).unwrap_or(n))
            .collect::<Vec<_>>();
        nums.sort_by_key(|(_, i)| *i);
        nums.into_iter().map(|(x, _)| x).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subsequence1() {
        let nums = vec![2, 1, 3, 3];
        let k = 2;
        assert_eq!(Solution::max_subsequence(nums, k), vec![3, 3]);
    }

    #[test]
    fn test_max_subsequence2() {
        let nums = vec![-1, -2, 3, 4];
        let k = 3;
        assert_eq!(Solution::max_subsequence(nums, k), vec![-1, 3, 4]);
    }

    #[test]
    fn test_max_subsequence3() {
        let nums = vec![3, 4, 3, 3];
        let k = 2;
        assert_eq!(Solution::max_subsequence(nums, k), vec![4, 3]);
    }

    #[test]
    fn test_max_subsequence4() {
        let nums = vec![4, -1, -2, 3];
        let k = 3;
        assert_eq!(Solution::max_subsequence(nums, k), vec![4, -1, 3]);
    }
}

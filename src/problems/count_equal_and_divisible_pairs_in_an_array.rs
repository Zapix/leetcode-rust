use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, item| {
                let entry = acc.entry(item.1).or_insert(vec![]);
                (*entry).push(item.0 as i32);
                acc
            })
            .values()
            .map(|x| {
                (0..x.len())
                    .map(|i| ((i + 1)..x.len()).filter(|j| x[i] * x[*j] % k == 0).count())
                    .sum::<usize>()
            })
            .sum::<usize>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 1, 2, 2, 2, 1, 3];
        let k = 2;
        let result = Solution::count_pairs(nums, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2, 3, 4];
        let k = 1;
        let result = Solution::count_pairs(nums, k);
        assert_eq!(result, 0);
    }
}

struct Solution;

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let k = k as usize;
        nums.iter()
            .enumerate()
            .filter_map(|(i, &num)| if num == key { Some(i) } else { None })
            .fold(vec![false; nums.len()], |mut acc, x| {
                for i in x.checked_sub(k).unwrap_or(0)..nums.len().min(x + k + 1) {
                    acc[i] = true;
                }
                acc
            })
            .iter()
            .enumerate()
            .filter_map(|(i, &value)| if value { Some(i as i32) } else { None })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![3, 4, 9, 1, 3, 9, 5];
        let key = 9;
        let k = 1;
        let result = Solution::find_k_distant_indices(nums, key, k);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_example2() {
        let nums = vec![2, 2, 2, 2, 2];
        let key = 2;
        let k = 2;
        let result = Solution::find_k_distant_indices(nums, key, k);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }
}

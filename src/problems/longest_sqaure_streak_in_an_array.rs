use std::collections::HashMap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut map = HashMap::new();

        for num in nums {
            let num_i64 = num as i64;
            let count = map.get(&num_i64).unwrap_or(&0);
            map.insert(num_i64 * num_i64, *count + 1);
        }
        *map.values().filter(|v| **v > 1).max().unwrap_or(&(-1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![4, 3, 6, 16, 8, 2];
        let res = Solution::longest_square_streak(nums);
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 3, 5, 6, 7];
        let res = Solution::longest_square_streak(nums);
        assert_eq!(res, -1);
    }

    #[test]
    fn test3() {
        let nums = vec![2, 4, 16, 3, 9, 81, 5, 25, 625, 390625];
        let res = Solution::longest_square_streak(nums);
        assert_eq!(res, 4);
    }
}

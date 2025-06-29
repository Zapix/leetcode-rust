struct Solution;

const MODULO: i32 = 1_000_000_007i32;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let pow_2 = {
            let cnt = nums.len();
            let mut pow = vec![1; cnt];
            for i in 1..cnt {
                pow[i] = (pow[i - 1] * 2) % MODULO;
            }
            pow
        };
        let mut result = 0;
        for l in 0..nums.len() {
            let value = target - nums[l];
            let r = match nums.binary_search(&value) {
                Ok(right) => right,
                Err(right) => right.checked_sub(1).unwrap_or(0),
            };
            if r >= l && nums[l] + nums[r] <= target {
                let cnt = r.checked_sub(l).unwrap_or(0) as u32;
                result = (result + pow_2[cnt as usize]) % MODULO;
            }
        }
        result % MODULO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![3, 5, 6, 7];
        let target = 9;
        assert_eq!(Solution::num_subseq(nums, target), 4);
    }

    #[test]
    fn test_example2() {
        let nums = vec![3, 3, 6, 8];
        let target = 10;
        assert_eq!(Solution::num_subseq(nums, target), 6);
    }

    #[test]
    fn test_example3() {
        let nums = vec![2, 3, 3, 4, 6, 7];
        let target = 12;
        assert_eq!(Solution::num_subseq(nums, target), 61);
    }
}

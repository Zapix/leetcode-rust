#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        for val in chalk.iter() {
            sum += val;
            if sum > k {
                break;
            }
        }
        let mut k = k % sum;
        let mut result = 0;
        for (i, val) in chalk.iter().enumerate() {
            if k < *val {
                result = i as i32;
                break;
            }
            k -= *val;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn leetcode_1() {
        let chalk = vec![5, 1, 5];
        let k = 22;
        let result = Solution::chalk_replacer(chalk, k);
        assert_eq!(0, result);
    }

    #[test]
    fn leetcode_2() {
        let chalk = vec![3, 4, 1, 2];
        let k = 25;
        let result = Solution::chalk_replacer(chalk, k);
        assert_eq!(1, result);
    }
}

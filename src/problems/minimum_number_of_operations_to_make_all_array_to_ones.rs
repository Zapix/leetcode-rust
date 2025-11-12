struct Solution;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a.max(b), a.min(b));

        while b != 0 {
            a = a % b;
            (a, b) = (a.max(b), a.min(b));
        }
        a
    }

    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let count_ones = nums.iter().filter(|&&x| x == 1).count();
        if count_ones > 0 {
            return (nums.len() - count_ones) as i32;
        }

        let first = *nums.first().unwrap();
        let common_gcd = nums.iter().fold(first, |acc, &item| Self::gcd(acc, item));

        if common_gcd != 1 {
            return -1;
        }
        let mut min_counts = i32::MAX;
        for i in 0..nums.len() {
            let mut g = 0;
            for j in i..nums.len() {
                g = Self::gcd(g, nums[j]);
                if g == 1 {
                    min_counts = min_counts.min((j - i + 1) as i32);
                    break;
                }
            }
        }

        min_counts + nums.len() as i32 - 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_operations_1() {
        let arr = vec![2, 6, 3, 4];
        assert_eq!(Solution::min_operations(arr), 4);
    }

    #[test]
    fn test_min_operations_2() {
        let nums = vec![2, 10, 6, 14];
        assert_eq!(Solution::min_operations(nums), -1);
    }

    #[test]
    fn test_min_operations_3() {
        let nums = vec![2, 28, 14, 7, 70, 700, 100];
        assert_eq!(Solution::min_operations(nums), 9);
    }

    #[test]
    fn test_min_operations_4() {
        let nums = vec![2, 28, 14, 70, 700, 100];
        assert_eq!(Solution::min_operations(nums), -1);
    }

    #[test]
    fn test_min_operations_5() {
        let nums = vec![1, 1, 1, 1, 12, 100];
        assert_eq!(Solution::min_operations(nums), 2);
    }

    #[test]
    fn test_min_operations_6() {
        let nums = vec![6, 10, 15];
        assert_eq!(Solution::min_operations(nums), 4);
    }
}

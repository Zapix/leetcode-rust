struct Solution;

impl Solution {
    pub fn can_divide_candies(candies: &Vec<i64>, k: &i64, amount: &i64) -> bool {
        candies.iter().map(|x| *x / *amount).sum::<i64>() >= *k
    }
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let candies = candies.iter().map(|x| *x as i64).collect::<Vec<_>>();
        let total_candies = candies.iter().sum::<i64>();
        let max_candies = *candies.iter().max().unwrap();
        if total_candies < k {
            return 0;
        }
        let mut left = 1i64;
        let mut right = max_candies;

        while left < right {
            let middle = (left + right + 1) / 2;
            if Self::can_divide_candies(&candies, &k, &middle) {
                left = middle;
            } else {
                right = middle - 1;
            }
        }

        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let candies = vec![5, 8, 6];
        let k = 3;
        assert_eq!(Solution::maximum_candies(candies, k), 5);
    }

    #[test]
    fn test_example_2() {
        let candies = vec![2, 5];
        let k = 11;
        assert_eq!(Solution::maximum_candies(candies, k), 0);
    }

    #[test]
    fn test_example_3() {
        let candies = vec![5, 6, 4, 10, 10, 1, 1, 2, 2, 2];
        let k = 9;
        assert_eq!(Solution::maximum_candies(candies, k), 3);
    }
}

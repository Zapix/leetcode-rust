use std::collections::HashMap;

struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let counter = points
            .into_iter()
            .fold(HashMap::new(), |mut acc, point| {
                *acc.entry(point[1] as i64).or_insert(0i64) += 1;
                acc
            })
            .into_iter()
            .filter(|&x| x.1 > 1)
            .collect::<Vec<_>>();

        let mut result = 0i64;
        let mut total_sum = 0i64;
        for p_num in counter {
            let edge = (p_num.1 * (p_num.1 - 1) / 2) as i64;
            result = (result + edge * total_sum) % MOD;
            total_sum = (total_sum + edge) % MOD;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_sample_1() {
        let points = vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![2, 2], vec![3, 2]];
        assert_eq!(Solution::count_trapezoids(points), 3)
    }

    #[test]
    fn test_sample_2() {
        let points = vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![2, 1]];
        assert_eq!(Solution::count_trapezoids(points), 1)
    }
}

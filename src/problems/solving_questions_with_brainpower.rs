struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![0i64; questions.len()];
        dp[questions.len() - 1] = questions[questions.len() - 1][0] as i64;

        for i in (0..questions.len() - 1).rev() {
            let points = questions[i][0] as i64;
            let brainpower = questions[i][1] as usize;
            let j = i + brainpower + 1;
            let max_extra = if j < dp.len() { dp[j] } else { 0 };
            dp[i] = (points + max_extra).max(dp[i + 1]);
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
        assert_eq!(Solution::most_points(questions), 5);
    }

    #[test]
    fn test_example_2() {
        let questions = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        assert_eq!(Solution::most_points(questions), 7);
    }
}

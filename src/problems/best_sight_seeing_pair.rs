#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut prev = a[0];

        for i in 1..a.len() {
            max = std::cmp::max(max, prev + a[i] - i as i32);
            prev = std::cmp::max(prev, a[i] + i as i32);
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]), 11);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_score_sightseeing_pair(vec![1, 2]), 2);
    }
}
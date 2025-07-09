struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        start_time
            .iter()
            .chain(vec![event_time].iter())
            .zip(vec![0].iter().chain(end_time.iter()))
            .map(|(&a, &b)| a - b)
            .collect::<Vec<_>>()
            .windows(k as usize + 1)
            .map(|x| x.iter().sum())
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let event_time = 5;
        let k = 1;
        let start_time = vec![1, 3];
        let end_time = vec![2, 5];
        assert_eq!(
            Solution::max_free_time(event_time, k, start_time, end_time),
            2
        );
    }

    #[test]
    fn test_example2() {
        let event_time = 10;
        let k = 1;
        let start_time = vec![0, 2, 9];
        let end_time = vec![1, 4, 10];
        assert_eq!(
            Solution::max_free_time(event_time, k, start_time, end_time),
            6
        );
    }

    #[test]
    fn test_example3() {
        let event_time = 5;
        let k = 2;
        let start_time = vec![0, 1, 2, 3, 4];
        let end_time = vec![1, 2, 3, 4, 5];
        assert_eq!(
            Solution::max_free_time(event_time, k, start_time, end_time),
            0
        );
    }
}

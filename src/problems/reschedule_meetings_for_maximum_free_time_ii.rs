struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut q = vec![false; n];
        let mut t1 = 0;
        let mut t2 = 0;
        for i in 0..n {
            if end_time[i] - start_time[i] <= t1 {
                q[i] = true;
            }
            t1 = t1.max(start_time[i] - if i == 0 { 0 } else { end_time[i - 1] });

            let j = n - i - 1;
            if end_time[j] - start_time[j] <= t2 {
                q[j] = true;
            }
            t2 = t2.max(
                if i == 0 {
                    event_time
                } else {
                    start_time[n - i]
                } - end_time[j],
            );
        }

        let mut res = 0;
        for i in 0..n {
            let left = if i == 0 { 0 } else { end_time[i - 1] };
            let right = if i == n - 1 {
                event_time
            } else {
                start_time[i + 1]
            };
            if q[i] {
                res = res.max(right - left);
            } else {
                res = res.max(right - left - (end_time[i] - start_time[i]));
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_example_1() {
        let event_time = 5;
        let start_time = vec![1, 3];
        let end_time = vec![2, 5];
        assert_eq!(Solution::max_free_time(event_time, start_time, end_time), 2);
    }

    #[test]
    pub fn test_example_2() {
        let event_time = 10;
        let start_time = vec![0, 7, 9];
        let end_time = vec![1, 8, 10];
        assert_eq!(Solution::max_free_time(event_time, start_time, end_time), 7);
    }

    #[test]
    pub fn test_example_3() {
        let event_time = 10;
        let start_time = vec![0, 3, 7, 9];
        let end_time = vec![1, 4, 8, 10];
        assert_eq!(Solution::max_free_time(event_time, start_time, end_time), 6);
    }

    #[test]
    pub fn test_example_4() {
        let event_time = 5;
        let start_time = vec![0, 1, 2, 3, 4];
        let end_time = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_free_time(event_time, start_time, end_time), 0);
    }
}

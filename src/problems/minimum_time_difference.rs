use std::cmp::min;
use std::ops::Sub;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Time(i32);

impl Time {
    pub fn from_string(time: &str) -> Self {
        let mut time = time.split(':');
        let hour = time.next().unwrap().parse::<i32>().unwrap();
        let minute = time.next().unwrap().parse::<i32>().unwrap();
        Time(hour * 60 + minute)
    }
}

impl Sub for Time {
    type Output = i32;

    fn sub(self, other: Self) -> Self::Output {
        let Time(a) = self;
        let Time(b) = other;
        let diff = a - b;
        if diff < 0 {
            diff + 24 * 60
        } else {
            diff
        }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time_points: Vec<Time> = time_points
            .iter()
            .map(|time| Time::from_string(time))
            .collect();
        time_points.sort();
        let mut min_diff = i32::MAX;
        for i in 0..time_points.len() {
            let diff = min(
                time_points[i] - time_points[(i + 1) % time_points.len()],
                time_points[(i + 1) % time_points.len()] - time_points[i],
            );
            if diff < min_diff {
                min_diff = diff;
            }
        }
        min_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(
            Solution::find_min_difference(vec!["23:59".to_string(), "00:00".to_string(),]),
            1
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::find_min_difference(vec![
                "00:00".to_string(),
                "23:59".to_string(),
                "00:00".to_string(),
            ]),
            0
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::find_min_difference(vec![
                "01:01".to_string(),
                "02:01".to_string(),
                "03:00".to_string(),
            ]),
            59
        );
    }
}

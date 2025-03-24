struct Solution;

const OPEN_MEETING: i32 = 0;
const CLOSE_MEETING: i32 = 1;

impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut time_marks = meetings.iter().map(|x| vec![(x[0], OPEN_MEETING), (x[1], CLOSE_MEETING)]).flatten().collect::<Vec<_>>();
        time_marks.sort();
        let time_marks = time_marks;
        let mut count_days = 0;
        let mut opened_meetings = 0;
        let mut prev_day = 0;
        for time_mark in time_marks {
            if time_mark.1 == OPEN_MEETING && opened_meetings == 0 {
                count_days += (time_mark.0 - prev_day - 1).max(0);
            }
            opened_meetings += if time_mark.1 == OPEN_MEETING {
                1
            } else {
                -1
            };
            prev_day = time_mark.0;
        }
        count_days += (days - prev_day).max(0);
        count_days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let days = 10;
        let meetings = vec![vec![5, 7], vec![1, 3], vec![9, 10]];
        let result = Solution::count_days(days, meetings);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_2() {
        let days = 5;
        let meetings = vec![vec![2, 4], vec![1, 3]];
        let result = Solution::count_days(days, meetings);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_example_3() {
        let days = 6;
        let meetings = vec![vec![1, 6]];
        let result = Solution::count_days(days, meetings);
        assert_eq!(result, 0);
    }
}

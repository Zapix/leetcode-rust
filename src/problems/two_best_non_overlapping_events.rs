#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut times = vec![];
        for event in events {
            times.push((event[0], 1, event[2]));
            times.push((event[1] + 1, 0, event[2]));
        }
        println!("{:?}", times);
        times.sort();
        println!("{:?}", times);

        let mut ans = 0;
        let mut max_value = 0;

        for time_value in times {
            if time_value.1 == 1 {
                ans = ans.max(time_value.2 + max_value);
            } else {
                max_value = max_value.max(time_value.2);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let events = vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]];
        assert_eq!(Solution::max_two_events(events), 4);
    }

    #[test]
    fn test_example_2() {
        let events = vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]];
        assert_eq!(Solution::max_two_events(events), 5);
    }

    #[test]
    fn test_example_3() {
        let events = vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]];
        assert_eq!(Solution::max_two_events(events), 8);
    }
}

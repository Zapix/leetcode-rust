use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_by_key(|x| x[0]);
        let mut max_day = events.iter().map(|x| x[1]).max().unwrap_or(0);
        let mut pq = BinaryHeap::new();
        let mut count = 0;
        let mut j = 0;
        for i in 1..=max_day {
            while j < events.len() && events[j][0] <= i {
                pq.push(Reverse(events[j][1]));
                j += 1;
            }
            while let Some(&Reverse(end)) = pq.peek() {
                if end < i {
                    pq.pop();
                } else {
                    break;
                }
            }
            if let Some(_) = pq.pop() {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let events = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::max_events(events), 3);
    }

    #[test]
    fn example2() {
        let events = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]];
        assert_eq!(Solution::max_events(events), 4);
    }

    #[test]
    fn example3() {
        let events = vec![vec![1, 2], vec![1, 2], vec![3, 3], vec![1, 5], vec![1, 5]];
        assert_eq!(Solution::max_events(events), 5);
    }
}

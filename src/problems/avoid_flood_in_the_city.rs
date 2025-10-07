use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut free_days = BinaryHeap::new();
        let mut used_lakes = HashMap::new();
        let mut days = vec![-1; rains.len()];

        for (i, &lake) in rains.iter().enumerate() {
            if lake == 0 {
                free_days.push(Reverse(i));
            } else {
                match used_lakes.get(&lake) {
                    None => {
                        used_lakes.insert(lake, i);
                    }
                    Some(&last_rain_day) => {
                        let mut earlier_days = vec![];
                        let mut found_day = false;
                        while let Some(Reverse(free_day)) = free_days.pop() {
                            if free_day > last_rain_day {
                                days[free_day] = lake;
                                used_lakes.insert(lake, i);
                                found_day = true;
                                break;
                            } else {
                                earlier_days.push(Reverse(free_day));
                            }
                        }
                        if !found_day {
                            return vec![];
                        } else {
                            for day in earlier_days {
                                free_days.push(day);
                            }
                        }
                    }
                }
            }
        }

        while let Some(Reverse(free_day)) = free_days.pop() {
            days[free_day] = 1; // Arbitrarily dry lake 1 on free days
        }
        days
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_avoid_flood_1() {
        let rains = vec![1, 2, 3, 4];
        assert_eq!(Solution::avoid_flood(rains), vec![-1, -1, -1, -1]);
    }
    #[test]
    fn test_avoid_flood_2() {
        let rains = vec![1, 2, 0, 0, 2, 1];
        assert_eq!(Solution::avoid_flood(rains), vec![-1, -1, 2, 1, -1, -1]);
    }
    #[test]
    fn test_avoid_flood_3() {
        let rains = vec![1, 2, 0, 1, 2];
        assert_eq!(Solution::avoid_flood(rains), vec![]);
    }

    #[test]fn test_avoid_flood_4() {
        let rains = vec![69, 0, 0, 0, 69];
        assert_eq!(Solution::avoid_flood(rains), vec![-1, 69, 1, 1, -1]);
    }
}

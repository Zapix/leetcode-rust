use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn dp(
        events: &Vec<Vec<i32>>,
        x: usize,
        k: i32,
        cache: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if k == 0 || x >= events.len() {
            return 0;
        }
        match cache.get(&(x, k)) {
            Some(&result) => result,
            None => {
                let j = events.partition_point(|e| (e[0] <= events[x][1]));
                let result = (Solution::dp(events, j, k - 1, cache) + events[x][2])
                    .max(Solution::dp(events, x + 1, k, cache));
                cache.insert((x, k), result);
                result
            }
        }
    }

    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut cache = HashMap::new();
        events.sort_by_key(|e| e[0]);
        Solution::dp(&events, 0, k, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let events = vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]];
        let k = 2;
        assert_eq!(Solution::max_value(events, k), 7);
    }

    #[test]
    fn test_example2() {
        let events = vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]];
        let k = 2;
        assert_eq!(Solution::max_value(events, k), 10);
    }

    #[test]
    fn test_example3() {
        let events = vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]];
        let k = 3;
        assert_eq!(Solution::max_value(events, k), 9);
    }

    #[test]
    fn test_example4() {
        let events = vec![
            vec![1, 3, 4],
            vec![2, 4, 1],
            vec![1, 1, 4],
            vec![3, 5, 1],
            vec![2, 5, 5],
        ];
        let k = 3;
        assert_eq!(Solution::max_value(events, k), 9);
    }
}

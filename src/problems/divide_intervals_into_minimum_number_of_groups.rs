enum Interval {
    Open(i32),
    Close(i32),
}

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut time_nodes = intervals.iter().fold(Vec::new(), |mut acc, interval| {
            acc.push(Interval::Open(interval[0]));
            acc.push(Interval::Close(interval[1]));
            acc
        });
        time_nodes.sort_by_key(|x| match x {
            Interval::Open(x) => (*x, 0),
            Interval::Close(x) => (*x, 1),
        });
        let mut max_opened = 0;
        let mut current = 0;

        for node in time_nodes {
            match node {
                Interval::Open(_) => current += 1,
                Interval::Close(_) => current -= 1,
            }
            max_opened = max_opened.max(current)
        }

        max_opened
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let intervals = vec![vec![1, 2], vec![2, 3], vec![2, 3], vec![1, 3]];
        let result = Solution::min_groups(intervals);
        assert_eq!(result, 4);
    }

    #[test]
    fn leetcode_2() {
        let intervals = vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]];
        let result = Solution::min_groups(intervals);
        assert_eq!(result, 3);
    }

    #[test]
    fn leetcode_3() {
        let intervals = vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]];
        let result = Solution::min_groups(intervals);
        assert_eq!(result, 1);
    }
}

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

enum TimeNode {
    In(i32, i32),
    Out(i32, i32),
}

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn smallest_chair(time: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut binary_heap = BinaryHeap::new();
        let mut friend_map = HashMap::new();
        for i in 0..time.len() {
            binary_heap.push(Reverse(i as i32));
        }
        let mut timeline = time
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (friend, t)| {
                acc.push(TimeNode::In(t[0], friend as i32));
                acc.push(TimeNode::Out(t[1], friend as i32));
                acc
            });

        timeline.sort_by_key(|x| match x {
            TimeNode::In(time, _) => (*time, 1),
            TimeNode::Out(time, _) => (*time, 0),
        });

        for node in timeline {
            match node {
                TimeNode::In(_, friend) => {
                    let seat = binary_heap.pop().unwrap();
                    if friend == target_friend {
                        return seat.0;
                    }
                    friend_map.insert(friend, seat);
                }
                TimeNode::Out(_, friend) => binary_heap.push(*friend_map.get(&friend).unwrap()),
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let times = vec![vec![1, 4], vec![2, 3], vec![4, 6]];
        let target_friend = 1;
        let result = Solution::smallest_chair(times, target_friend);
        assert_eq!(result, 1);
    }

    #[test]
    fn leetcode_2() {
        let times = vec![vec![3, 10], vec![1, 5], vec![2, 6]];
        let target_friend = 0;
        let result = Solution::smallest_chair(times, target_friend);
        assert_eq!(result, 2);
    }

    #[test]
    fn custom_test_case() {
        let times = vec![
            vec![18, 19],
            vec![10, 11],
            vec![21, 22],
            vec![5, 6],
            vec![2, 3],
            vec![6, 7],
            vec![43, 44],
            vec![48, 49],
            vec![53, 54],
            vec![12, 13],
            vec![20, 21],
            vec![34, 35],
            vec![17, 18],
            vec![1, 2],
            vec![35, 36],
            vec![16, 17],
            vec![9, 10],
            vec![14, 15],
            vec![25, 26],
            vec![37, 38],
            vec![30, 31],
            vec![50, 51],
            vec![22, 23],
            vec![3, 4],
            vec![27, 28],
            vec![29, 30],
            vec![33, 34],
            vec![39, 40],
            vec![49, 50],
            vec![15, 16],
            vec![4, 5],
            vec![46, 47],
            vec![51, 52],
            vec![32, 33],
            vec![11, 12],
            vec![28, 29],
            vec![47, 48],
            vec![36, 37],
            vec![40, 41],
            vec![42, 43],
            vec![52, 53],
            vec![41, 42],
            vec![31, 32],
            vec![23, 24],
            vec![8, 9],
            vec![19, 20],
            vec![24, 25],
            vec![26, 27],
            vec![45, 46],
            vec![44, 45],
            vec![7, 8],
            vec![13, 14],
            vec![38, 39],
        ];
        let target_friend = 8;
        let result = Solution::smallest_chair(times, target_friend);
        assert_eq!(result, 0);
    }
}

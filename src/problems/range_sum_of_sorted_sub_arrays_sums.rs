use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[allow(dead_code)]
struct Solution;
 #[allow(dead_code)]
 impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut sums_heap = BinaryHeap::new();

        for i in 0..n {
            let mut sum = 0;
            for j in i..n {
               sum += nums[j as usize];
               sums_heap.push(sum)
            }
        }

        sums_heap.into_sorted_vec().iter().skip(left as usize - 1).take((right - left + 1) as usize).fold(
            0,
            |acc, value| (acc + *value) % (10_i32.pow(9) + 7)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::range_sum(
                vec![1, 2, 3, 4],
                4,
                1,
                5
            ),
            13
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::range_sum(
                vec![1, 2, 3, 4],
                4,
                3,
                4
            ),
            6
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::range_sum(
                vec![1, 2, 3, 4],
                4,
                1,
               10
            ),
            50
        );
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::range_sum(
                vec![1, 2, 5, 4],
                4,
                1,
                5
            ),
            15
        );
    }
}
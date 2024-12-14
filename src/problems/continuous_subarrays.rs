use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut min_heap: BinaryHeap<(Reverse<i32>, Reverse<usize>)> = BinaryHeap::new();
        let mut max_heap: BinaryHeap<(i32, Reverse<usize>)> = BinaryHeap::new();
        let mut left = 0;
        let mut right = 0;
        let mut result = 0;

        while right < nums.len() {
            min_heap.push((Reverse(nums[right]), Reverse(right)));
            max_heap.push((nums[right], Reverse(right)));

            while left < right && max_heap.peek().unwrap().0 - min_heap.peek().unwrap().0 .0 > 2 {
                left += 1;

                while !min_heap.is_empty() && min_heap.peek().unwrap().1 .0 < left {
                    min_heap.pop();
                }
                while !max_heap.is_empty() && max_heap.peek().unwrap().1 .0 < left {
                    max_heap.pop();
                }
            }

            result += (right - left + 1) as i64;
            right += 1
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![5, 4, 2, 4];
        assert_eq!(Solution::continuous_subarrays(nums), 8);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::continuous_subarrays(nums), 6);
    }
}

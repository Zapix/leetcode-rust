use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut heap = nums.iter().map(|&num| Reverse(num as i64)).collect::<BinaryHeap<_>>();

        let mut count = 0i32;
        while heap.peek().unwrap_or(&Reverse(i64::MAX)).0 < k && heap.len() >= 2 {
            count += 1;
            let a = heap.pop().unwrap().0;
            let b = heap.pop().unwrap().0;
            let val = a.min(b)*2 + a.max(b);
            heap.push(Reverse(val));
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations_example1() {
        let nums = vec![2, 11, 10, 1, 3];
        let k = 10;
        assert_eq!(Solution::min_operations(nums, k), 2);
    }

    #[test]
    fn test_min_operations_example2() {
        let nums = vec![1, 1, 2, 4, 9];
        let k = 20;
        assert_eq!(Solution::min_operations(nums, k), 4);
    }

    #[test]
    fn test_min_operations_single_element() {
        let nums = vec![10];
        let k = 10;
        assert_eq!(Solution::min_operations(nums, k), 0);
    }

    #[test]
    fn test_min_operations_no_operations_needed() {
        let nums = vec![15, 20, 25];
        let k = 10;
        assert_eq!(Solution::min_operations(nums, k), 0);
    }
}

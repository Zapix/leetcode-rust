use std::collections::binary_heap::BinaryHeap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut binary_heap = BinaryHeap::new();

        for pile in piles {
            binary_heap.push(pile);
        }

        for i  in 0..k {
            let pile = binary_heap.pop().unwrap();
            binary_heap.push(pile - pile / 2);
        }

        binary_heap.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::min_stone_sum(vec![5, 4, 9], 2),
            12
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::min_stone_sum(vec![4, 3, 6, 7], 3),
            12
        );
    }
}
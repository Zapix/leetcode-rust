use std::collections::BinaryHeap;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from(gifts);
        for _ in 0..k {
            let value = heap.pop().unwrap_or(0);
            heap.push((value as f64).sqrt() as i32);
        }
        heap.iter().map(|x| (*x as i64)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let gifts = vec![25, 64, 9, 4, 100];
        let k = 4;
        assert_eq!(Solution::pick_gifts(gifts, k), 29);
    }

    #[test]
    fn test_example_2() {
        let gifts = vec![1, 1, 1, 1];
        let k = 4;
        assert_eq!(Solution::pick_gifts(gifts, k), 4);
    }
}

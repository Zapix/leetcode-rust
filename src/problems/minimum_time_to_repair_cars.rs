use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut heap = BinaryHeap::new();
        for rank in ranks {
            heap.push((Reverse(rank as i64), (rank as i64, 1)));
        }
        let mut total_time = 0;
        for _ in 0..cars {
            let (Reverse(rank), (original_rank, count)) = heap.pop().unwrap();
            total_time = rank;
            let next_count = count + 1;
            heap.push((Reverse(original_rank * next_count * next_count), (original_rank, next_count)));
        }
        total_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let ranks = vec![4, 2, 3, 1];
        let cars = 10;
        let expected = 16;
        assert_eq!(Solution::repair_cars(ranks, cars), expected);
    }

    #[test]
    fn test_example_2() {
        let ranks = vec![5, 1, 8];
        let cars = 6;
        let expected = 16;
        assert_eq!(Solution::repair_cars(ranks, cars), expected);
    }

    #[test]
    fn test_single_mechanic() {
        let ranks = vec![3];
        let cars = 5;
        let expected = 75;
        assert_eq!(Solution::repair_cars(ranks, cars), expected);
    }

    #[test]
    fn test_multiple_mechanics_equal_ranks() {
        let ranks = vec![2, 2, 2];
        let cars = 9;
        let expected = 18; // Example calculation: evenly distributed work
        assert_eq!(Solution::repair_cars(ranks, cars), expected);
    }

    #[test]
    fn test_no_cars() {
        let ranks = vec![1, 2, 3];
        let cars = 1;
        let expected = 1; // No cars to repair
        assert_eq!(Solution::repair_cars(ranks, cars), expected);
    }
}


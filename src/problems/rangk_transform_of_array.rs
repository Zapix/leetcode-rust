use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut unique = arr
            .iter()
            .map(|x| *x)
            .collect::<HashSet<i32>>()
            .iter()
            .map(|x| *x)
            .collect::<Vec<i32>>();
        unique.sort();
        let rank = unique
            .iter()
            .enumerate()
            .map(|x| (*x.1, x.0))
            .collect::<HashMap<i32, usize>>();

        arr.iter()
            .map(|x| *rank.get(x).unwrap() as i32 + 1)
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            vec![4, 1, 2, 3]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100]),
            vec![1, 1, 1]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}

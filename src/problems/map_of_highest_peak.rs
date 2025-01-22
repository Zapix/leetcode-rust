use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::convert::TryFrom;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn heighest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut heights = vec![vec![-1;is_water[0].len()]; is_water.len()];

        let mut que = BinaryHeap::new();
        for i in 0..is_water.len() {
            for j in 0..is_water[0].len() {
                if is_water[i][j] == 1 {
                    que.push((Reverse(0), (i,j)));
                }
            }
        }

        while let Some((Reverse(cost), (i,j))) = que.pop() {
            if heights[i][j] != -1 {
                continue;
            }
            heights[i][j] = cost;
            for (dx, dy) in [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
                match (usize::try_from(i as i32 + dx), (usize::try_from(j as i32 + dy))) {
                    (Ok(i), Ok(j)) if i <  heights.len() && j < heights[0].len() => {
                        que.push((Reverse(cost + 1), (i, j)))
                    },
                    _ => {},
                }
            }
        }

        heights
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_peak_example1() {
        let is_water = vec![vec![0, 1], vec![0, 0]];
        let expected = vec![vec![1, 0], vec![2, 1]];
        assert_eq!(Solution::heighest_peak(is_water), expected);
    }

    #[test]
    fn test_highest_peak_example2() {
        let is_water = vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]];
        let expected = vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]];
        assert_eq!(Solution::heighest_peak(is_water), expected);
    }

    #[test]
    fn test_highest_peak_all_water() {
        let is_water = vec![vec![1, 1], vec![1, 1]];
        let expected = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::heighest_peak(is_water), expected);
    }

    #[test]
    fn test_highest_peak_mixed() {
        let is_water = vec![vec![0, 1, 0], vec![0, 0, 0], vec![1, 0, 0]];
        let expected = vec![vec![1, 0, 1], vec![1, 1, 2], vec![0, 1, 2]];
        assert_eq!(Solution::heighest_peak(is_water), expected);
    }
}

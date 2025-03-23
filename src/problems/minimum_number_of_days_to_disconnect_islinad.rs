use std::collections::{HashSet, VecDeque};
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {

    fn get_cell(grid: &Vec<Vec<i32>>, x: i32, y: i32) -> Option<&i32> {
        if x < 0 || y < 0 {
            None
        } else {
            grid.get(x as usize)?.get(y as usize)
        }
    }
    fn count_islands(grid: &Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut visited = HashSet::new();
        let n = grid.len();
        let m = grid.get(0).unwrap_or(&vec![]).len();
        for i in 0..n {
            for j in 0..m {
                if visited.contains(&(i, j)) {
                    continue
                }
                match Self::get_cell(&grid, i as i32, j as i32){
                    Some(&1) => {
                        count += 1;

                        let mut que = VecDeque::new();
                        que.push_back((i, j));
                        while let Some(point) = que.pop_front() {
                            if visited.contains(&point) {
                                continue
                            }
                            visited.insert(point);
                            for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                                let i_new = point.0 as i32 + di;
                                let j_new = point.1 as i32 + dj;

                                match Self::get_cell(&grid, i_new, j_new) {
                                    Some(&1) => {
                                        que.push_back((i_new as usize, j_new as usize));
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        count
    }

    pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
        if Self::count_islands(&grid) != 1 {
            return 0
        }

        let n = grid.len();
        let m = grid.get(0).unwrap_or(&vec![]).len();

        for i in 0..n {
            for j in 0..m {
                match Self::get_cell(&grid, i as i32, j as i32) {
                    Some(&1) => {
                        grid[i][j] = 0;
                        if Self::count_islands(&grid) != 1 {
                           return 1;
                        }
                        grid[i][j] = 1;
                    }
                    _ => {}
                }
            }
        }

        2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::min_days(
                vec![
                    vec![0, 1, 1, 0],
                    vec![0, 1, 1, 0],
                    vec![0, 0, 0, 0],
                ]
            ),
            2
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::min_days(
                vec![
                    vec![1, 1]
                ]
            ),
            2
        )
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::min_days(
                vec![
                    vec![1, 0, 1]
                ]
            ),
            0
        );
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::min_days(
                vec![
                    vec![1, 0, 1],
                    vec![1, 1, 1],
                    vec![1, 0, 1],
                ]
            ),
            1
        );
    }
}
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn check_uniqueness(x: usize, y: usize, grid: &Vec<Vec<i32>>) -> Option<bool> {
        let mut used: [bool; 9] = [false; 9];

        for i in -1..2 {
            for j in -1..2 {
                let digit = *grid
                    .get((x as i32 + i) as usize)?
                    .get((y  as i32 + j) as usize)?;
                if digit < 1 || digit > 9 || used[(digit - 1) as usize] {
                    return Some(false);
                } else {
                    used[(digit - 1) as usize] = true;
                }
            }
        }
        Some(true)
    }

    fn check_sum(x: usize, y: usize, grid: &Vec<Vec<i32>>, mask: [[i32; 2]; 3]) -> Option<bool> {
        let result = mask
            .map(
                |[i,j]| *grid
                    .get((x as i32 + i) as usize).unwrap_or(&vec![])
                    .get((y as i32 + j) as usize).unwrap_or(&0)
            )
            .iter()
            .sum::<i32>();

        if result == 15 {
            Some(true)
        } else {
            None
        }
    }
    fn check_magic(x: usize, y: usize, grid: &Vec<Vec<i32>>) -> Option<bool> {

        if !Solution::check_uniqueness(x, y, grid).unwrap_or(false) {
            return Some(false);
        }

        if !Solution::check_sum(x, y, grid, [[-1, -1], [-1, 0], [-1, 1]]).unwrap_or(false) {
            return Some(false);
        }
        if !Solution::check_sum(x, y, grid, [[0, -1], [0, 0], [0, 1]]).unwrap_or(false) {
            return Some(false);
        }
        if !Solution::check_sum(x, y, grid, [[1, -1], [1, 0], [1, 1]]).unwrap_or(false) {
            return Some(false);
        }

        if !Solution::check_sum(x, y, grid, [[-1, -1], [0, -1], [1, -1]]).unwrap_or(false) {
            return Some(false);
        };
        if !Solution::check_sum(x, y, grid, [[-1, 0], [0, 0], [1, 0]]).unwrap_or(false) {
            return Some(false);
        };
        if !Solution::check_sum(x, y, grid, [[-1, 1], [0, 1], [1, 1]]).unwrap_or(false) {
            return Some(false);
        };

        if !Solution::check_sum(x, y, grid, [[-1, -1], [0, 0], [1, 1]]).unwrap_or(false) {
            return Some(false);
        };
        if !Solution::check_sum(x, y, grid, [[-1, 1], [0, 0], [1, -1]]).unwrap_or(false) {
            return Some(false);
        };

        Some(true)
    }
    fn is_magic_square(i: usize, j: usize, grid: &Vec<Vec<i32>>) -> bool {
        Solution::check_magic(i, j, grid).unwrap_or(false)
    }

    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut counter = 0;
        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                if Solution::is_magic_square(i, j, &grid) {
                    counter += 1
                }
            }
        }
        counter
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::num_magic_squares_inside(
                vec![
                    vec![4, 3, 8, 4],
                    vec![9, 5, 1, 9],
                    vec![2, 7, 6, 2]
                ]
            ),
            1
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::num_magic_squares_inside(
                vec![
                    vec![8]
                ]
            ),
            0
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::num_magic_squares_inside(
                vec![
                    vec![10,3,5],
                    vec![1,6,11],
                    vec![7,9,2]
                ]
            ),
            0
        )
    }
}
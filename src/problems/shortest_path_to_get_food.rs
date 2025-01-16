use std::collections::{HashSet, VecDeque};
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_food(grid: Vec<Vec<char>>) -> i32 {
        let mut start = None;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '*' {
                    start = Some((i, j));
                    break;
                }
            }
            if start.is_some() {
                break;
            }
        }
        let start = start.unwrap();
        let mut que = VecDeque::new();
        que.push_back((start, 0));
        let mut visited = HashSet::new();
        while let Some((position, steps)) = que.pop_front() {
            if grid[position.0][position.1] == '#' {
                return steps;
            }
            if visited.contains(&position) || grid[position.0][position.1] == 'X' {
                continue;
            }
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                match (
                    usize::try_from(position.0 as i32 + dx),
                    usize::try_from(position.1 as i32 + dy),
                ) {
                    (Ok(x), Ok(y)) if x < grid.len() && y < grid[0].len() => {
                        que.push_back(((x, y), steps + 1))
                    }
                    _ => {}
                }
            }
            visited.insert(position);
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec!['X', 'X', 'X', 'X', 'X', 'X'],
            vec!['X', '*', 'O', 'O', 'O', 'X'],
            vec!['X', 'O', 'O', '#', 'O', 'X'],
            vec!['X', 'X', 'X', 'X', 'X', 'X'],
        ];
        assert_eq!(Solution::get_food(grid), 3);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec!['X', 'X', 'X', 'X', 'X'],
            vec!['X', '*', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', '#', 'X'],
            vec!['X', 'X', 'X', 'X', 'X'],
        ];
        assert_eq!(Solution::get_food(grid), -1);
    }

    #[test]
    fn test_example_3() {
        let grid = vec![
            vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
            vec!['X', '*', 'O', 'X', 'O', '#', 'O', 'X'],
            vec!['X', 'O', 'O', 'X', 'O', 'O', 'X', 'X'],
            vec!['X', 'O', 'O', 'O', 'O', '#', 'O', 'X'],
            vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        ];
        assert_eq!(Solution::get_food(grid), 6);
    }
}

#[derive(Eq, PartialEq)]
enum Cell {
    Guard,
    Wall,
    Guarded,
    Free,
}

impl Cell {
    pub fn is_stop(&self) -> bool {
        *self == Self::Wall || *self == Self::Guard
    }
}
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_ungrounded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut grid: Vec<Vec<Cell>> = (0..m)
            .map(|_| (0..n).map(|_| Cell::Free).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for guard in guards.iter() {
            grid[guard[0] as usize][guard[1] as usize] = Cell::Wall;
        }

        for wall in walls {
            grid[wall[0] as usize][wall[1] as usize] = Cell::Wall;
        }

        for guard in guards {
            grid[guard[0] as usize][guard[1] as usize] = Cell::Guard;

            for j in (0..guard[0])
                .rev()
                .take_while(|i| !grid[*i as usize][guard[1] as usize].is_stop())
                .collect::<Vec<i32>>()
            {
                if grid[j as usize][guard[1] as usize] == Cell::Free {
                    grid[j as usize][guard[1] as usize] = Cell::Guarded;
                }
            }

            for j in (guard[0] + 1..m)
                .take_while(|i| !grid[*i as usize][guard[1] as usize].is_stop())
                .collect::<Vec<i32>>()
            {
                if grid[j as usize][guard[1] as usize] == Cell::Free {
                    grid[j as usize][guard[1] as usize] = Cell::Guarded;
                }
            }

            for j in (0..guard[1])
                .rev()
                .take_while(|i| !grid[guard[0] as usize][*i as usize].is_stop())
                .collect::<Vec<i32>>()
            {
                if grid[guard[0] as usize][j as usize] == Cell::Free {
                    grid[guard[0] as usize][j as usize] = Cell::Guarded;
                }
            }

            for j in (guard[1] + 1..n)
                .take_while(|i| !grid[guard[0] as usize][*i as usize].is_stop())
                .collect::<Vec<i32>>()
            {
                if grid[guard[0] as usize][j as usize] == Cell::Free {
                    grid[guard[0] as usize][j as usize] = Cell::Guarded;
                }
            }
        }

        let mut result = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i as usize][j as usize] == Cell::Free {
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let m = 4;
        let n = 6;
        let guards = vec![vec![0, 0], vec![1, 1], vec![2, 3]];
        let walls = vec![vec![0, 1], vec![2, 2], vec![1, 4]];
        let result = Solution::count_ungrounded(m, n, guards, walls);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_example_2() {
        let m = 3;
        let n = 3;
        let guards = vec![vec![1, 1]];
        let walls = vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]];
        let result = Solution::count_ungrounded(m, n, guards, walls);
        assert_eq!(result, 4);
    }
}

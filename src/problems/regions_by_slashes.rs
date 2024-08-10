use std::collections::{HashSet, VecDeque};
#[allow(dead_code)]
struct Solution;

enum CellType {
    Empty,
    Wall
}

fn get_cell_type_from_char (char: &char, i: usize, j: usize) -> Option<CellType>{
    match char {
        ' ' => Some(CellType::Empty),
        '/' =>  match [i, j] {
            [0, 0] => Some(CellType::Empty),
            [0, 1] => Some(CellType::Empty),
            [1, 0] => Some(CellType::Empty),
            [1, 2] => Some(CellType::Empty),
            [2, 1] => Some(CellType::Empty),
            [2, 2] => Some(CellType::Empty),
            _ => Some(CellType::Wall)
        },
        '\\' => match [i, j] {
            [0, 1] => Some(CellType::Empty),
            [0, 2] => Some(CellType::Empty),
            [1, 2] => Some(CellType::Empty),
            [1, 0] => Some(CellType::Empty),
            [2, 0] => Some(CellType::Empty),
            [2, 1] => Some(CellType::Empty),
            _ => Some(CellType::Wall),
        }
        _ => None
    }
}

struct Grid<'a> {
    raw_data: &'a Vec<String>,
}

impl<'a> Grid<'a> {
    pub fn from(raw_data: &'a Vec<String>) -> Self {
        Self {
            raw_data,
        }
    }

    pub fn get_n(&self) -> usize {
        self.raw_data.len() * 3
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<CellType> {
        if x < 0 || y < 0 {
            return None
        }
        let (x_raw, x_delta) = ((x / 3) as usize, (x % 3) as usize);
        let (y_raw, y_delta) = ((y / 3) as usize, (y % 3) as usize);
        let ch = *self.raw_data.get(x_raw).unwrap_or(&String::from("")).chars().collect::<Vec<char>>().get(y_raw)?;
        get_cell_type_from_char(&ch, x_delta, y_delta)
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let grid = Grid::from(&grid);
        let mut count = 0;
        let mut visited = HashSet::new();
        for i in 0..(grid.get_n()) {
            for j in 0..(grid.get_n()) {
                if visited.contains(&(i, j)) {
                    continue
                }
                match grid.get_cell(i as i32, j as i32) {
                    Some(CellType::Empty) => {
                        count += 1;
                        let mut que = VecDeque::new();
                        que.push_back((i, j));
                        while let Some((x, y)) = que.pop_front() {
                            if visited.contains(&(x, y)) {
                                continue
                            }
                            visited.insert((x, y));
                            for (dx, dy) in [(0i32, 1i32), (1, 0), (-1, 0), (0, -1)] {
                                match grid.get_cell(x as i32 + dx, y as i32 + dy) {
                                    Some(CellType::Empty) => {
                                        que.push_back(((x as i32 + dx) as usize, (y as i32 + dy) as usize))
                                    },
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::regions_by_slashes(vec![
                String::from(" /"),
                String::from("/ ")
            ]),
            2
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::regions_by_slashes(vec![
                String::from(" /"),
                String::from("  ")
            ]),
            1
        );
    }

    #[test]
    fn leetcode_3() {
        assert_eq!(
            Solution::regions_by_slashes(vec![String::from("/\\"), String::from("\\/")]),
            5
        );
    }

    #[test]
    fn leetcode_4() {
        assert_eq!(
            Solution::regions_by_slashes(vec![String::from("//"),String::from("/ ")]),
            3
        );
    }
}

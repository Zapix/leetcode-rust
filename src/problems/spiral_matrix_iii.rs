enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP
}

struct Spiral {
    x: i32,
    y: i32,
    direction: Direction,
    side_length: i32,
    counter: i32,
}

impl Spiral {
    pub fn new() -> Spiral {
        Spiral {
            x: 0,
            y: 0,
            direction: Direction::RIGHT,
            side_length: 1,
            counter: 0,
        }
    }
}

impl Iterator for Spiral {
    type Item = [i32; 2];
    fn next(&mut self) -> Option<Self::Item> {
        let cell = [self.x, self.y];
        match self.direction {
            Direction::RIGHT => {
                self.y += 1
            }
            Direction::DOWN => {
                self.x += 1
            }
            Direction::LEFT => {
                self.y -= 1
            }
            Direction::UP => {
                self.x -= 1
            }
        }
        self.counter += 1;
        if self.counter % self.side_length == 0 {
            self.direction = match self.direction {
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
                Direction::UP => Direction::RIGHT,
            };
            if self.counter / self.side_length == 2 {
                self.counter = 0;
                self.side_length += 1;
            }
        }
        Some(cell)
    }
}
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let rows = rows as usize;
        let cols = cols as usize;
        let mut visitied: Vec<[i32; 2]> = Vec::new();
        let mut spiral = Spiral::new();

        while visitied.len() < rows * cols {
            let mut cell = spiral.next().unwrap();
            cell[0] += r_start;
            cell[1] += c_start;

            if 0 <= cell[0] && cell[0] < (rows as i32) && 0 <= cell[1] && cell[1] < (cols as i32) {
                visitied.push(cell)
            }
        }
        visitied.iter().map(Vec::from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            vec![
                vec![0, 0],
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
            ]
        );
    }

    #[test]
    fn leetcode_2() {
        assert_eq!(
            Solution::spiral_matrix_iii(5, 6, 1, 4),
            vec![
                vec![1, 4],
                vec![1, 5],
                vec![2, 5],
                vec![2, 4],
                vec![2, 3],
                vec![1, 3],
                vec![0, 3],
                vec![0, 4],
                vec![0, 5],
                vec![3, 5],
                vec![3, 4],
                vec![3, 3],
                vec![3, 2],
                vec![2, 2],
                vec![1, 2],
                vec![0, 2],
                vec![4, 5],
                vec![4, 4],
                vec![4, 3],
                vec![4, 2],
                vec![4, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1, 1],
                vec![0, 1],
                vec![4, 0],
                vec![3, 0],
                vec![2, 0],
                vec![1, 0],
                vec![0, 0],
            ]
        )
    }
}
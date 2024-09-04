use std::collections::HashMap;

enum Directions {
    North,
    East,
    South,
    West,
}

impl Directions {
    pub fn rotate_right(&self) -> Directions {
        match self {
            Directions::North => Directions::East,
            Directions::East => Directions::South,
            Directions::South => Directions::West,
            Directions::West => Directions::North,
        }
    }

    pub fn rotate_left(&mut self) -> Directions {
        match self {
            Directions::North => Directions::West,
            Directions::West => Directions::South,
            Directions::South => Directions::East,
            Directions::East => Directions::North,
        }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut x_obstacles: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut y_obstacles: HashMap<i32, Vec<i32>> = HashMap::new();

        for obstacle in obstacles {
            let (x, y) = (obstacle[0], obstacle[1]);
            if !x_obstacles.contains_key(&x) {
                x_obstacles.insert(x, vec![]);
            }
            let x_vec = x_obstacles.get_mut(&x).unwrap();
            x_vec.push(y);

            if !y_obstacles.contains_key(&y) {
                y_obstacles.insert(y, vec![]);
            }
            let y_vec = y_obstacles.get_mut(&y).unwrap();
            y_vec.push(x);
        }

        let mut max_distance = 0;
        let mut position = (0, 0);
        let mut direction = Directions::North;

        for command in commands {
            match command {
                -1 => direction = direction.rotate_right(),
                -2 => direction = direction.rotate_left(),
                k => match direction {
                    Directions::North => {
                        let mut k = k;
                        for y in x_obstacles.get(&position.0).unwrap_or(&vec![]) {
                            if *y <= position.1 {
                                continue;
                            }
                            k = k.min((*y - position.1).abs() - 1)
                        }
                        position.1 += k;
                    }
                    Directions::East => {
                        let mut k = k;
                        for x in y_obstacles.get(&position.1).unwrap_or(&vec![]) {
                            if *x <= position.0 {
                                continue;
                            }
                            k = k.min((*x - position.0).abs() - 1);
                        }
                        position.0 += k;
                    }
                    Directions::South => {
                        let mut k = k;
                        for y in x_obstacles.get(&position.0).unwrap_or(&vec![]) {
                            if *y >= position.1 {
                                continue;
                            }
                            k = k.min((*y - position.1).abs() - 1)
                        }
                        position.1 -= k;
                    }
                    Directions::West => {
                        let mut k = k;
                        for x in y_obstacles.get(&position.1).unwrap_or(&vec![]) {
                            if *x >= position.0 {
                                continue;
                            }
                            k = k.min((*x - position.0).abs() - 1);
                        }
                        position.0 -= k;
                    }
                },
            }
            max_distance = max_distance.max(position.0 * position.0 + position.1 * position.1)
        }

        max_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let commands = vec![4, -1, 3];
        let obstacles = vec![];
        let result = Solution::robot_sim(commands, obstacles);
        assert_eq!(25, result);
    }

    #[test]
    fn leetcode_2() {
        let commands = vec![4, -1, 4, -2, 4];
        let obstacles = vec![vec![2, 4]];
        let result = Solution::robot_sim(commands, obstacles);
        assert_eq!(65, result);
    }

    #[test]
    fn leetcode_3() {
        let commands = vec![6, -1, -1, 6];
        let obstacles = vec![];
        let result = Solution::robot_sim(commands, obstacles);
        assert_eq!(36, result);
    }

    #[test]
    fn leetcode_4() {
        let commands = vec![-2, -1, -2, 3, 7];
        let obstacles = vec![
            vec![1, -3],
            vec![2, -3],
            vec![4, 0],
            vec![-2, 5],
            vec![-5, 2],
            vec![0, 0],
            vec![4, -4],
            vec![-2, -5],
            vec![-1, -2],
            vec![0, 2],
        ];
        let result = Solution::robot_sim(commands, obstacles);
        assert_eq!(100, result);
    }
}

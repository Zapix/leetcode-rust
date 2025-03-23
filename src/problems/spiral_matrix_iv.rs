use std::boxed::Box;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn rotate(&self) -> Direction {
        match *self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    pub fn next_step(&self) -> (i32, i32) {
        match self {
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
            Direction::Up => (-1, 0),
        }
    }
}

struct SpiralMatrix {
    m: usize,
    n: usize,
    direction: Direction,
    step_count: usize,
    i: i32,
    j: i32,
    visited: Vec<Vec<bool>>,
}

impl SpiralMatrix {
    pub fn new(m: usize, n: usize) -> Self {
        Self {
            m,
            n,
            direction: Direction::Right,
            step_count: 0,
            i: 0,
            j: -1,
            visited: vec![vec![false; n]; m],
        }
    }

    pub fn get_next(&self) -> (i32, i32) {
        let delta = self.direction.next_step();
        (self.i + delta.0, self.j + delta.1)
    }

    fn can_move_forward(&self) -> bool {
        let delta = self.direction.next_step();
        !*self
            .visited
            .get((self.i + delta.0) as usize)
            .unwrap_or(&vec![])
            .get((self.j + delta.1) as usize)
            .unwrap_or(&true)
    }
}

impl Iterator for SpiralMatrix {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.step_count == self.m * self.n {
            return None;
        }
        while !self.can_move_forward() {
            self.direction = self.direction.rotate();
        }
        (self.i, self.j) = self.get_next();
        self.visited[self.i as usize][self.j as usize] = true;
        Some((self.i, self.j))
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![-1; n as usize]; m as usize];
        let mut current = head;

        let mut matrix = SpiralMatrix::new(m as usize, n as usize);
        while let Some(node) = current {
            let (i, j) = matrix.next().unwrap();
            result[i as usize][j as usize] = node.val;
            current = node.next;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_1() {
        let m = 3;
        let n = 3;
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode {
                                    val: 7,
                                    next: Some(Box::new(ListNode {
                                        val: 8,
                                        next: Some(Box::new(ListNode { val: 9, next: None })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let result = Solution::spiral_matrix(m, n, head);
        assert_eq!(vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]], result);
    }

    #[test]
    fn leetcode_2() {
        let m = 1;
        let n = 4;
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let result = Solution::spiral_matrix(m, n, head);
        assert_eq!(vec![vec![1, 2, 3, -1]], result);
    }

    #[test]
    fn leetcode_3() {
        let m = 3;
        let n = 5;
        let head = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode {
                            val: 8,
                            next: Some(Box::new(ListNode {
                                val: 1,
                                next: Some(Box::new(ListNode {
                                    val: 7,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode {
                                            val: 4,
                                            next: Some(Box::new(ListNode {
                                                val: 2,
                                                next: Some(Box::new(ListNode {
                                                    val: 5,
                                                    next: Some(Box::new(ListNode {
                                                        val: 5,
                                                        next: Some(Box::new(ListNode {
                                                            val: 0,
                                                            next: None,
                                                        })),
                                                    })),
                                                })),
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let result = Solution::spiral_matrix(m, n, head);
        assert_eq!(
            vec![
                vec![3, 0, 2, 6, 8],
                vec![5, 0, -1, -1, 1],
                vec![5, 2, 4, 9, 7]
            ],
            result
        )
    }
}

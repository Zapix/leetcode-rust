use std::collections::{HashSet, VecDeque};

#[derive(Hash, Eq, PartialEq)]
struct GameKey([i32; 6]);

impl GameKey {
    pub fn from(board: &Vec<Vec<i32>>) -> Self {
        let mut key = [0; 6];
        for i in 0..2 {
            for j in 0..3 {
                key[i * 3 + j] = board[i][j];
            }
        }
        Self(key)
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let finish_key = GameKey::from(&vec![vec![1, 2, 3], vec![4, 5, 0]]);

        let mut x = 0;
        let mut y = 0;
        for i in 0..2 {
            for j in 0..3 {
                if board[i][j] == 0 {
                    x = i;
                    y = j;
                    break;
                }
            }
        }

        let mut que = VecDeque::new();
        que.push_back((board, 0, (x, y)));
        let mut used: HashSet<GameKey> = HashSet::new();

        while let Some((board, count, (x, y))) = que.pop_front() {
            let key = GameKey::from(&board);
            if key == finish_key {
                return count;
            }
            if used.contains(&key) {
                continue;
            }

            for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = x as i32 + i;
                let ny = y as i32 + j;
                if nx >= 0 && nx < 2 && ny >= 0 && ny < 3 {
                    let mut board = board.clone();
                    let nx = nx as usize;
                    let ny = ny as usize;
                    board[x][y] = board[nx][ny];
                    board[nx][ny] = 0;
                    que.push_back((board, count + 1, (nx, ny)));
                }
            }
            used.insert(key);
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let board = vec![vec![1, 2, 3], vec![4, 0, 5]];
        assert_eq!(Solution::sliding_puzzle(board), 1);
    }

    #[test]
    fn test_example_2() {
        let board = vec![vec![1, 2, 3], vec![5, 4, 0]];
        assert_eq!(Solution::sliding_puzzle(board), -1);
    }

    #[test]
    fn test_example_3() {
        let board = vec![vec![4, 1, 2], vec![5, 0, 3]];
        assert_eq!(Solution::sliding_puzzle(board), 5);
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rotate_the_box(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = grid.len();
        let n = grid[0].len();

        let mut rotated_grid = Vec::from(
            (0..n)
                .map(|_| (0..m).map(|_| '.').collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
        for i in 0..m {
            let mut shift = 0;
            for j in (0..n).rev() {
                match grid[i][j] {
                    '*' => {
                        shift = 0;
                        rotated_grid[j][m - i - 1] = '*';
                    }
                    '#' => rotated_grid[j + shift][m - i - 1] = '#',
                    '.' => {
                        shift += 1;
                    }
                    _ => panic!("Unexpected character"),
                }
            }
        }

        rotated_grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_the_box_example_1() {
        let box_input = vec![vec!['#', '.', '#']];
        let expected_output = vec![vec!['.'], vec!['#'], vec!['#']];
        assert_eq!(Solution::rotate_the_box(box_input), expected_output);
    }

    #[test]
    fn test_rotate_the_box_example_2() {
        let box_input = vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']];
        let expected_output = vec![
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['*', '*'],
            vec!['.', '.'],
        ];
        assert_eq!(Solution::rotate_the_box(box_input), expected_output);
    }

    #[test]
    fn test_rotate_the_box_example_3() {
        let box_input = vec![
            vec!['#', '#', '*', '.', '*', '.'],
            vec!['#', '#', '#', '*', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.'],
        ];
        let expected_output = vec![
            vec!['.', '#', '#'],
            vec!['.', '#', '#'],
            vec!['#', '#', '*'],
            vec!['#', '*', '.'],
            vec!['#', '.', '*'],
            vec!['#', '.', '.'],
        ];
        assert_eq!(Solution::rotate_the_box(box_input), expected_output);
    }
}

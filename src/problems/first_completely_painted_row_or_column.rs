use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut coords_map: HashMap<i32, (usize, usize)>= HashMap::new();

        for i in 0..n {
            for j in 0..m {
                coords_map.insert(mat[i][j], (i, j));
            }
        }

        let mut rows = HashMap::new();
        let mut cols = HashMap::new();
        
        for (i, val) in arr.iter().enumerate() {
            let coord = coords_map.get(val).unwrap();
            let row_count = rows.entry(coord.0).or_insert(0);
            *row_count += 1;
            let col_count = cols.entry(coord.1).or_insert(0);
            *col_count += 1;

            if *col_count == n  || *row_count == m {
                return i as i32
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_complete_index_example1() {
        let arr = vec![1, 3, 4, 2];
        let mat = vec![vec![1, 4], vec![2, 3]];
        assert_eq!(Solution::first_complete_index(arr, mat), 2);
    }

    #[test]
    fn test_first_complete_index_example2() {
        let arr = vec![2, 8, 7, 4, 1, 3, 5, 6, 9];
        let mat = vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]];
        assert_eq!(Solution::first_complete_index(arr, mat), 3);
    }


    #[test]
    fn test_first_complete_index_single_element() {
        let arr = vec![1];
        let mat = vec![vec![1]];
        assert_eq!(Solution::first_complete_index(arr, mat), 0);
    }

    #[test]
    fn test_first_complete_index_example3() {
        let arr = vec![1, 4, 5, 2, 6, 3];
        let mat = vec![vec![4, 3, 5], vec![1, 2, 6]];
        assert_eq!(Solution::first_complete_index(arr, mat), 1);
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_area_from_corner(i: i32, j: i32, areas: &Vec<Vec<i32>>) -> i32 {
        if i < 0 || i >= areas.len() as i32 || j < 0 || j >= areas[0].len() as i32 {
            return 0;
        }
        areas[i as usize][j as usize]
    }

    pub fn compute_area_from_corner(
        i: usize,
        j: usize,
        matrix: &Vec<Vec<i32>>,
        areas: &Vec<Vec<i32>>,
    ) -> i32 {
        matrix[i][j]
            + Solution::get_area_from_corner(i as i32 - 1, j as i32, areas)
            + Solution::get_area_from_corner(i as i32, j as i32 - 1, areas)
            - Solution::get_area_from_corner(i as i32 - 1, j as i32 - 1, areas)
    }

    pub fn compute_area(i: usize, j: usize, x: usize, y: usize, areas: &Vec<Vec<i32>>) -> i32 {
        Solution::get_area_from_corner(x as i32, y as i32, areas)
            - Solution::get_area_from_corner(x as i32, j as i32 - 1, areas)
            - Solution::get_area_from_corner(i as i32 - 1, y as i32, areas)
            + Solution::get_area_from_corner(i as i32 - 1, j as i32 - 1, areas)
    }

    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut areas = vec![vec![0; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                let area = Self::compute_area_from_corner(i, j, &matrix, &areas);
                areas[i][j] = area;
            }
        }

        let mut result = 0i32;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                let mut k = 0usize;
                while Solution::compute_area(i, j, i + k, j + k, &areas) == (k + 1).pow(2) as i32 {
                    k += 1;
                }
                result += k as i32;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
        let res = Solution::count_squares(matrix);
        assert_eq!(res, 15);
    }

    #[test]
    fn test2() {
        let matrix = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
        let res = Solution::count_squares(matrix);
        assert_eq!(res, 7);
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn get_break_points(nums: &Vec<i32>) -> Vec<usize> {
        let mut break_points = vec![];
        for (i, value) in nums.iter().enumerate().skip(1) {
            if nums[i - 1] % 2 == value % 2 {
                break_points.push(i);
            }
        }

        break_points
    }

    fn has_number(points: &Vec<usize>, start: usize, end: usize) -> bool {
        if points.is_empty() || start > end {
            return false;
        }
        let mut i = 0;
        let mut j = points.len() - 1;
        while i <= j {
            let middle = (i + j) / 2;
            if points[middle] < start {
                if middle == points.len() - 1 {
                    break;
                }
                i = middle + 1
            } else if points[middle] > end {
                if middle == 0 {
                    break;
                }
                j = middle - 1;
            } else {
                return true;
            }
        }
        return false;
    }
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let break_points = Solution::get_break_points(&nums);

        queries
            .iter()
            .map(|x| !Solution::has_number(&break_points, x[0] as usize + 1, x[1] as usize))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 4, 1, 2, 6];
        let queries = vec![vec![0, 4]];
        assert_eq!(Solution::is_array_special(nums, queries), vec![false]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![4, 3, 1, 6];
        let queries = vec![vec![0, 2], vec![2, 3]];
        assert_eq!(Solution::is_array_special(nums, queries), vec![false, true]);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![3, 6, 2, 1];
        let queries = vec![vec![0, 1]];
        assert_eq!(Solution::is_array_special(nums, queries), vec![true]);
    }
}

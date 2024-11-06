#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_sort_arrays(nums: Vec<i32>) -> bool {
        let pairs = nums
            .chunk_by(|x, y| x.count_ones() == y.count_ones())
            .map(|x| (x.iter().min().unwrap(), x.iter().max().unwrap()))
            .collect::<Vec<_>>();

        pairs.clone().iter().zip(pairs.clone().iter().skip(1)).all(|(x, y)| x.1 <= y.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![8, 4, 2, 30, 15];
        assert_eq!(Solution::can_sort_arrays(nums), true);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::can_sort_arrays(nums), true);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 16, 8, 4, 2];
        assert_eq!(Solution::can_sort_arrays(nums), false);
    }
}
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let count_ones = data.iter().fold(0, |acc, x| acc + x);
        let mut min_swaps = count_ones;
        let mut cnt = data
            .iter()
            .take(count_ones as usize)
            .fold(0, |acc, x| acc + x);

        min_swaps = min_swaps.min(count_ones - cnt);
        for (i, value) in data.iter().enumerate().skip(count_ones as usize) {
            cnt += value - data[i - count_ones as usize];
            min_swaps = min_swaps.min(count_ones - cnt);
        }

        min_swaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let data = vec![1, 0, 1, 0, 1];
        assert_eq!(Solution::min_swaps(data), 1);
    }

    #[test]
    fn test_example_2() {
        let data = vec![0, 0, 0, 1, 0];
        assert_eq!(Solution::min_swaps(data), 0);
    }

    #[test]
    fn test_example_3() {
        let data = vec![1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1];
        assert_eq!(Solution::min_swaps(data), 3);
    }
}

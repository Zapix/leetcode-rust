#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let m = m as usize;
        let n = n as usize;

        if (m * n) != original.len() {
            return vec![];
        }

        let mut result = vec![vec![0; n as usize]; m as usize];

        for (i, val) in original.iter().enumerate() {
            result[i / n][i % n] = *val;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn leetcode_1() {
        let original = vec![1, 2, 3, 4];
        let m = 2;
        let n = 2;
        let result = Solution::construct2_d_array(original, m, n);
        assert_eq!(vec![vec![1, 2], vec![3, 4]], result);
    }

    #[test]
    fn leetcode_2() {
        let original = vec![1, 2, 3];
        let m = 1;
        let n = 3;
        let result = Solution::construct2_d_array(original, m, n);
        assert_eq!(vec![vec![1, 2, 3]], result);
    }

    #[test]
    fn leetcode_3() {
        let original = vec![1, 2];
        let m = 1;
        let n = 1;
        let result = Solution::construct2_d_array(original, m, n);
        assert_eq!(vec![vec![1]], result);
    }
}

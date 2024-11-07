#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut counter = [0; 32];

        for value in candidates {
            for i in 0..32 {
                if value & (1 << i) > 0 {
                    counter[i] += 1
                }
            }
        }

        *counter.iter().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_combination_case_1() {
        let candidates = vec![16, 17, 71, 62, 12, 24, 14];
        assert_eq!(Solution::largest_combination(candidates), 4);
    }

    #[test]
    fn test_largest_combination_case_2() {
        let candidates = vec![8, 8];
        assert_eq!(Solution::largest_combination(candidates), 2);
    }

    #[test]
    fn test_largest_combination_case_3() {
        let candidates = vec![1, 2, 4, 8];
        assert_eq!(Solution::largest_combination(candidates), 1);
    }

    #[test]
    fn test_largest_combination_case_4() {
        let candidates = vec![0, 0, 0];
        assert_eq!(Solution::largest_combination(candidates), 0);
    }
}

const MODULO: i32 = 10_i32.pow(9) + 7;
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut total = 0;
        let mut odd_count = 0;
        let mut even_count = 0;
        for value in arr {
            sum += value;
            if sum % 2 == 0 {
                total = (total + odd_count) % MODULO;
                even_count += 1;
            } else {
                total = (total + 1 + even_count) % MODULO;
                odd_count += 1;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let arr = vec![1, 3, 5];
        assert_eq!(Solution::num_of_subarrays(arr), 4);
    }

    #[test]
    fn test_example_2() {
        let arr = vec![2, 4, 6];
        assert_eq!(Solution::num_of_subarrays(arr), 0);
    }

    #[test]
    fn test_example_3() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(Solution::num_of_subarrays(arr), 16);
    }
}
